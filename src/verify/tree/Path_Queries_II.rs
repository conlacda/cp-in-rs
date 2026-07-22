#![allow(dead_code)]
use rs_space::range_query::segtree::MaxNode;
use rs_space::range_query::segtree::Node;
use rs_space::sw::scanner;
use rs_space::sw::writer;
use rs_space::tree::hld::HLD;
use std::io::Write;

#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::dbg;
#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::set_limit::timeout_secs;

fn main() {
    #[cfg(feature = "local")]
    timeout_secs(5);
    let mut scan = scanner();
    let mut out = writer();
    let n: usize = scan.token();
    let q: usize = scan.token();
    let weight: Vec<i64> = (0..n).map(|_| scan.token()).collect();
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..n - 1 {
        let u: usize = scan.token();
        let v: usize = scan.token();
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    let mut hld: HLD<MaxNode> = HLD::new(&graph, &weight);
    hld.set_weight_on_nodes(true);
    for _ in 0..q {
        let t: i32 = scan.token();
        let u: usize = scan.token();
        let v: i64 = scan.token();
        if t == 1 {
            // Update
            hld.set_node(u - 1, Node::new(v));
        } else {
            // Query
            let res = hld.query_path(u - 1, (v - 1).try_into().unwrap());
            write!(out, "{:?} ", res.val).unwrap();
        }
    }
}
