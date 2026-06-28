#![allow(dead_code)]
use std::io;
use std::io::Write;
use std::marker::PhantomData;
use std::str;

#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::dbg;
#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::set_limit::timeout_secs;

#[derive(Default, Debug)]
pub struct RMQ<T: Clone> {
    values: Vec<T>,
    range_low: Vec<Vec<usize>>,
    max_mode: bool,
}

impl<T: Clone + Ord> RMQ<T> {
    pub fn new(values: &[T], max_mode: bool) -> Self {
        let mut rmq = Self {
            values: values.to_vec(),
            range_low: Vec::new(),
            max_mode,
        };
        rmq.build();
        rmq
    }

    pub fn build(&mut self) {
        let n = self.values.len();
        let levels = self.floor_log2(n) + 1;
        self.range_low.resize(levels, Vec::new());
        for k in 0..levels {
            self.range_low[k].resize(n - (1 << k) + 1, 0);
        }
        for i in 0..n {
            self.range_low[0][i] = i;
        }
        for k in 1..levels {
            for i in 0..=n - (1 << k) {
                self.range_low[k][i] = self.better_index(
                    self.range_low[k - 1][i],
                    self.range_low[k - 1][i + (1 << (k - 1))],
                );
            }
        }
    }

    fn floor_log2(&self, x: usize) -> usize {
        assert!(x > 0);
        (usize::BITS - 1 - x.leading_zeros()) as usize
        // Some(x.ilog2() as usize)
    }

    fn better_index(&self, a: usize, b: usize) -> usize {
        // use <= for the case when values[a] == values[b] then return a
        if (self.max_mode && self.values[b] < self.values[a])
            || (!self.max_mode && self.values[a] < self.values[b])
        {
            a
        } else {
            b
        }
    }

    pub fn query_index(&self, l: usize, mut r: usize) -> usize {
        let n = self.values.len();
        assert!(l <= r && r <= n);
        r += 1;
        if l == r {
            return l;
        }
        let level = self.floor_log2(r - l);
        self.better_index(
            self.range_low[level][l],
            self.range_low[level][r - (1 << level)],
        )
    }
    pub fn query_value(&self, l: usize, r: usize) -> T {
        self.values[self.query_index(l, r)].clone()
    }
}

macro_rules! recursive_function {
    ($name: ident, $trait: ident, ($($type: ident $arg: ident,)*)) => {
        pub trait $trait<$($type, )*Output> {
            fn call(&mut self, $($arg: $type,)*) -> Output;
        }

        pub struct $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            f: std::cell::UnsafeCell<F>,
            $($arg: PhantomData<$type>,
            )*
            phantom_output: PhantomData<Output>,
        }

        impl<F, $($type, )*Output> $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            pub fn new(f: F) -> Self {
                Self {
                    f: std::cell::UnsafeCell::new(f),
                    $($arg: Default::default(),
                    )*
                    phantom_output: Default::default(),
                }
            }
        }

        impl<F, $($type, )*Output> $trait<$($type, )*Output> for $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            fn call(&mut self, $($arg: $type,)*) -> Output {
                unsafe { (*self.f.get())(self, $($arg, )*) }
            }
        }
    }
}

recursive_function!(RecursiveFunction0, Callable0, ());
recursive_function!(RecursiveFunction, Callable, (Arg arg,));
recursive_function!(RecursiveFunction2, Callable2, (Arg1 arg1, Arg2 arg2,));
recursive_function!(RecursiveFunction3, Callable3, (Arg1 arg1, Arg2 arg2, Arg3 arg3,));
recursive_function!(RecursiveFunction4, Callable4, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4,));
recursive_function!(RecursiveFunction5, Callable5, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5,));
recursive_function!(RecursiveFunction6, Callable6, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6,));

/// Same API as Scanner but nearly twice as fast, using horribly unsafe dark arts
pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    /// This function should be marked unsafe, but noone has time for that in a
    /// programming contest. Use at your own risk!
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute::<
                    std::str::SplitAsciiWhitespace<'_>,
                    std::str::SplitAsciiWhitespace<'static>,
                >(slice.split_ascii_whitespace())
            };
        }
    }
}

fn is_local() -> bool {
    // "--local"/"local" forces file input, "--stdin"/"--no-local" forces stdin.
    // Without flags, the default is set via the Cargo feature "local".
    let mut arg_mode: Option<bool> = None;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--local" | "local" => arg_mode = Some(true),
            "--stdin" | "--no-local" => arg_mode = Some(false),
            _ => {}
        }
    }
    arg_mode.unwrap_or(cfg!(feature = "local"))
}

fn scanner() -> UnsafeScanner<Box<dyn io::BufRead>> {
    let reader: Box<dyn io::BufRead> = if is_local() {
        let input_path = format!("{}/inp.txt", std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let file = std::fs::File::open(input_path).expect("Input file not found");
        Box::new(io::BufReader::new(file))
    } else {
        Box::new(io::BufReader::new(io::stdin()))
    };

    UnsafeScanner::new(reader)
}

fn writer() -> io::BufWriter<Box<dyn Write>> {
    let writer: Box<dyn Write> = if is_local() {
        let output_path = format!("{}/out.txt", std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let file = std::fs::File::create(output_path).expect("Failed to create output file");
        Box::new(file)
    } else {
        Box::new(io::stdout())
    };

    io::BufWriter::new(writer)
}

struct Euler {
    pub first: Vec<usize>,
    pub tour: Vec<usize>,
    pub depth: Vec<usize>,
}

pub struct LCA {
    first: Vec<usize>,
    tour: Vec<usize>,
    rmq: RMQ<usize>,
}

impl LCA {
    pub fn new(tree: &[Vec<usize>], root: usize) -> Self {
        let euler = Self::make_eulertour(tree, root);
        Self {
            first: euler.first,
            tour: euler.tour,
            rmq: RMQ::new(&euler.depth, false),
        }
    }

    fn lca(&self, u: usize, v: usize) -> usize {
        let mut fu = self.first[u];
        let mut fv = self.first[v];
        if fu > fv {
            std::mem::swap(&mut fu, &mut fv);
        }
        let index = self.rmq.query_index(fu, fv);
        self.tour[index]
    }

    fn make_eulertour(tree: &[Vec<usize>], root: usize) -> Euler {
        let n = tree.len();
        let mut dep: usize = 0;
        let mut first: Vec<usize> = vec![usize::MAX; n];
        let mut tour = vec![];
        let mut depth = vec![];
        let mut dfs = RecursiveFunction2::new(|dfs, node: usize, parent: usize| {
            dep += 1;
            if first[node] == usize::MAX {
                first[node] = tour.len();
            }
            depth.push(dep);
            tour.push(node);
            for &child in &tree[node] {
                if child == parent {
                    continue;
                }
                dfs.call(child, node);
                dep -= 1;
                tour.push(node);
                depth.push(dep);
            }
        });
        dfs.call(root, root);
        Euler { first, tour, depth }
    }
}

fn solve(scan: &mut UnsafeScanner<Box<dyn io::BufRead>>, out: &mut io::BufWriter<Box<dyn Write>>) {
    let n: usize = scan.token();

    let mut tree = vec![Vec::new(); n];
    for u in 0..n {
        let m: usize = scan.token();
        for _ in 0..m {
            let v: usize = scan.token::<usize>() - 1;
            tree[u].push(v);
            tree[v].push(u);
        }
    }

    let lca = LCA::new(&tree, 0);

    let q: usize = scan.token();
    for _ in 0..q {
        let u: usize = scan.token::<usize>() - 1;
        let v: usize = scan.token::<usize>() - 1;
        writeln!(out, "{}", lca.lca(u, v) + 1).unwrap();
    }
}

fn main() {
    #[cfg(feature = "local")]
    timeout_secs(5);
    let mut scan = scanner();
    let mut out = writer();

    let t: usize = scan.token();
    for case in 1..=t {
        writeln!(out, "Case {}:", case).unwrap();
        solve(&mut scan, &mut out);
    }
}
