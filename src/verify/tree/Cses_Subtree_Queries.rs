use rs_space::range_query::segtree::Node;
use rs_space::range_query::segtree::SegTree;
use rs_space::range_query::segtree::SumNode;
use rs_space::scanner_writer::writer;
use rs_space::set_limit::timeout_secs;
use rs_space::tree::mix::make_eulertour;
use std::io::Write;

use rs_space::scanner_writer::scanner;

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
            seg.set(inout[node].0, &SumNode::new(val));
            seg.set(inout[node].1, &SumNode::new(val));
        } else {
            let mut u: usize = scan.token();
            u -= 1;
            let node = seg.query(inout[u].0, inout[u].1);
            writeln!(out, "{:?}", node.val / 2).unwrap();
        }
    }
}
