#![allow(dead_code)]
use rs_space::sw::UnsafeScanner;
use rs_space::sw::scanner;
use rs_space::sw::writer;
use rs_space::tree::lca::LCA;
use std::io;
use std::io::Write;

#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::dbg;
#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::set_limit::timeout_secs;

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
