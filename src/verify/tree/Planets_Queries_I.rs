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
 
struct BinaryJumping {
    max_depth: usize,
    up: Vec<Vec<usize>>,
}
 
impl BinaryJumping {
    pub fn new(parent: &[usize], max_depth: usize) -> Self {
        assert!(max_depth != 0);
        let n = parent.len();
        let log2 = max_depth.next_power_of_two().ilog2() as usize;
        let mut up = vec![vec![0; n]; log2 + 1];
        for i in 0..n {
            up[0][i] = parent[i];
        }
        for k in 1..=log2 {
            for i in 0..n {
                up[k][i] = up[k - 1][up[k - 1][i]];
            }
        }
        Self { up, max_depth }
    }
    pub fn kth_parent(&self, mut node: usize, mut k: usize) -> usize {
        assert!(k <= self.max_depth, "k exceeds max_depth");
        let mut bit = 0;
        while k != 0 {
            if k & 1 != 0 {
                node = self.up[bit][node];
            }
            bit += 1;
            k >>= 1;
        }
        node
    }
}
 
fn main() {
    #[cfg(feature = "local")]
    timeout_secs(5);
    let mut scan = scanner();
    let mut out = writer();
    let n: usize = scan.token();
    let q: usize = scan.token();
    let mut parent = vec![0; n];
    for i in 0..n {
        let p: usize = scan.token();
        parent[i] = p - 1;
    }
    let bj = BinaryJumping::new(&parent, 1_000_000_001);
    for _ in 0..q {
        let mut node: usize = scan.token();
        node -= 1;
        let k: usize = scan.token();
        writeln!(out, "{:?}", bj.kth_parent(node, k) + 1).unwrap();
    }
}