#![allow(dead_code)]
#![allow(unused)]
use std::io;
use std::io::Write;
use std::str;

#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::dbg;
#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::set_limit::timeout_secs;

use std::marker::PhantomData;

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

pub fn make_eulertour(tree: &[Vec<usize>], root: usize) -> (Vec<usize>, Vec<Vec<usize>>) {
    let n = tree.len();
    let mut eulertour: Vec<usize> = Vec::new();
    let mut dfs = RecursiveFunction2::new(|dfs, u: usize, p: usize| {
        eulertour.push(u);
        for &v in tree[u].iter() {
            if v != p {
                dfs.call(v, u);
            }
        }
        eulertour.push(u);
    });
    dfs.call(root, root);
    let mut inout: Vec<Vec<usize>> = vec![vec![]; n];
    for (index, &val) in eulertour.iter().enumerate() {
        inout[val].push(index);
    }
    (eulertour, inout)
}

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

pub trait Node: Default + Clone + Copy {
    fn combine(&self, other: Self) -> Self;
    fn right_to_left(&self) -> Self {
        *self
    }
}

pub struct SegTree<T> {
    n: usize,
    dat: Vec<T>,
}

impl<T> SegTree<T>
where
    T: Node,
{
    pub fn from(v: &[T]) -> Self {
        let n = v.len().next_power_of_two();
        let mut dat: Vec<T> = (0..2 * n - 1).map(|_| T::default()).collect();
        for i in 0..v.len() {
            dat[n + i - 1] = v[i];
        }
        for i in (0..n - 1).rev() {
            dat[i] = dat[i * 2 + 1].combine(dat[i * 2 + 2]);
        }
        Self { n, dat }
    }

    pub fn set(&mut self, mut index: usize, x: &T) {
        index += self.n - 1;
        self.dat[index] = *x;

        while index > 0 {
            index = (index - 1) / 2;
            self.dat[index] = self.dat[index * 2 + 1].combine(self.dat[index * 2 + 2]);
        }
    }

    pub fn query(&self, mut l: usize, mut r: usize) -> T {
        assert!(l <= r);
        let mut lnode = T::default();
        let mut rnode = T::default();
        l += self.n - 1;
        r += self.n;
        while l < r {
            if (l & 1) == 0 {
                lnode = lnode.combine(self.dat[l]);
            }
            if (r & 1) == 0 {
                rnode = self.dat[r - 1].combine(rnode);
            }
            l /= 2;
            r = (r - 1) / 2;
        }
        lnode.combine(rnode)
    }

    pub fn at(&self, index: usize) -> T {
        assert!(index < self.n);
        self.query(index, index)
    }

    /// Returns the smallest index `r >= start` such that `cmp(&query(start, r))` is `true`.
    ///
    /// The predicate must be monotonic with respect to `r`: once it becomes `true`,
    /// it must remain `true` for all larger indices, otherwise the binary search is invalid.
    ///
    /// Returns `None` if no such index exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rs_space::range_query::segtree::{SegTree, MaxNode};
    /// let nodes: Vec<MaxNode> = (0..100).map(|v| MaxNode::new(v)).collect();
    /// let mut seg = SegTree::from(&nodes);
    /// seg.find_right(0, |range_node| range_node.val >= 10);
    /// ```
    pub fn find_right<F>(&self, start: usize, cmp: F) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        assert!(start < self.n);
        let mut l = start;
        let mut r = self.n - 1;
        while l != r {
            let mid = (l + r) / 2;
            let acc = self.query(start, mid);
            if cmp(&acc) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        let acc = self.query(start, l);
        if cmp(&acc) { Some(l) } else { None }
    }

    /// Returns the largest index `l <= end` such that `cmp(&query(l, end))` is `true`.
    ///
    /// The predicate must be monotonic with respect to `l`: once it is `true` at some index,
    /// it must remain `true` for all larger indices up to `end`, otherwise the binary search is invalid.
    ///
    /// Returns `None` if no such index exists.
    pub fn find_left<F>(&self, end: usize, cmp: F) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        assert!(end < self.n);
        let mut l = 0;
        let mut r = end;
        while l != r {
            let mid = (l + r).div_ceil(2);
            let acc = self.query(mid, end);
            if cmp(&acc) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        let acc = self.query(l, end);
        if cmp(&acc) { Some(l) } else { None }
    }
}

#[derive(Default, Clone, Copy)]
pub struct SumNode {
    pub val: i64,
    pub has_value: bool,
}

impl SumNode {
    pub fn new(val: i64) -> Self {
        Self {
            val,
            has_value: true,
        }
    }
}

impl Node for SumNode {
    fn combine(&self, other: Self) -> Self {
        if !self.has_value {
            return other;
        }
        if !other.has_value {
            return *self;
        }
        Self::new(self.val + other.val)
    }
}

fn main() {
    #[cfg(feature = "local")]
    timeout_secs(5);
    let mut scan = scanner();
    let mut out = writer();
    let n: usize = scan.token();
    let q: usize = scan.token();
    let weight: Vec<i64> = (0..n).map(|_| scan.token()).collect();
    let mut tree: Vec<Vec<usize>> = vec![vec![]; n];
    for _ in 0..n - 1 {
        let mut u: usize = scan.token();
        let mut v: usize = scan.token();
        u -= 1;
        v -= 1;
        tree[u].push(v);
        tree[v].push(u);
    }
    let (et, inout) = make_eulertour(&tree, 0);
    let nodes: Vec<SumNode> = (0..2 * n).map(|i| SumNode::new(weight[et[i]])).collect();
    let mut seg = SegTree::from(&nodes);

    for _ in 0..q {
        let t: i32 = scan.token();
        if t == 1 {
            let mut node: usize = scan.token();
            node -= 1;
            let val: i64 = scan.token();
            seg.set(inout[node][0], &SumNode::new(val));
            seg.set(inout[node][1], &SumNode::new(val));
        } else {
            let mut u: usize = scan.token();
            u -= 1;
            let node = seg.query(inout[u][0], inout[u][1]);
            writeln!(out, "{:?}", node.val / 2).unwrap();
        }
    }
}
