// https://judge.yosupo.jp/submission/377673
#![allow(dead_code)]
use std::io;
use std::io::Write;
use std::str;

#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::dbg;
#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::set_limit::timeout_secs;

use std::mem::swap;

pub struct PersistentDsu {
    ccnum: usize,
    rank: Vec<usize>,
    parent: Vec<usize>,
    op: Vec<(usize, usize, usize, usize)>,
}

impl PersistentDsu {
    pub fn new(n: usize) -> Self {
        Self {
            ccnum: n,
            rank: vec![0; n],
            parent: (0..n).collect(),
            op: vec![],
        }
    }

    pub fn find(&self, a: usize) -> usize {
        if a == self.parent[a] {
            a
        } else {
            self.find(self.parent[a])
        }
    }

    pub fn merge(&mut self, mut a: usize, mut b: usize) -> bool {
        a = self.find(a);
        b = self.find(b);
        if a == b {
            return false;
        }
        if self.rank[a] < self.rank[b] {
            swap(&mut a, &mut b);
        }
        self.op.push((a, b, self.rank[a], self.rank[b]));
        self.ccnum -= 1;
        self.parent[b] = a;
        if self.rank[a] == self.rank[b] {
            self.rank[a] += 1;
        }
        true
    }

    pub fn rollback(&mut self) {
        if let Some((a, b, rank_a, rank_b)) = self.op.pop() {
            self.ccnum += 1;
            self.parent[a] = a;
            self.parent[b] = b;
            self.rank[a] = rank_a;
            self.rank[b] = rank_b;
        }
    }

    pub fn is_same(&self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
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

struct Query {
    t: usize,
    k: usize,
    u: usize,
    v: usize,
}

fn main() {
    #[cfg(feature = "local")]
    timeout_secs(5);

    let mut scan = scanner();
    let mut out = writer();

    let n: usize = scan.token();
    let q: usize = scan.token();

    let mut queries = Vec::with_capacity(q + 1);
    queries.push(Query {
        t: 0,
        k: 0,
        u: 0,
        v: 0,
    });

    let mut queries_using_k = vec![Vec::<usize>::new(); q + 1];

    for i in 1..=q {
        let t: usize = scan.token();
        let k: isize = scan.token();
        let u: usize = scan.token();
        let v: usize = scan.token();

        let k = (k + 1) as usize;

        queries.push(Query { t, k, u, v });

        queries_using_k[k].push(i);
    }

    let mut ans = vec![false; q + 1];
    let mut dsu = PersistentDsu::new(n);

    fn dfs(
        idx: usize,
        queries: &[Query],
        queries_using_k: &[Vec<usize>],
        ans: &mut [bool],
        dsu: &mut PersistentDsu,
    ) {
        if queries[idx].t == 0 {
            dsu.merge(queries[idx].u, queries[idx].v);
        } else if queries[idx].t == 1 {
            ans[idx] = dsu.is_same(queries[idx].u, queries[idx].v);
        }

        for &child in &queries_using_k[idx] {
            dfs(child, queries, queries_using_k, ans, dsu);
        }

        if queries[idx].t == 0 {
            dsu.rollback();
        }
    }

    dfs(0, &queries, &queries_using_k, &mut ans, &mut dsu);

    for i in 1..=q {
        if queries[i].t == 1 {
            writeln!(out, "{}", if ans[i] { 1 } else { 0 }).unwrap();
        }
    }
}
