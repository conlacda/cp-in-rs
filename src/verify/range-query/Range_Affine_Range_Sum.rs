// https://atcoder.jp/contests/practice2/tasks/practice2_k
use rs_space::range_query::lazysegtree::{LazySegTree, RangeAffineLazyNode, RangeAffineSumNode};
use rs_space::range_query::segtree::Node;
use rs_space::sw::{scanner, writer};
use std::io::Write;

fn main() {
    let mut scan = scanner();
    let mut out = writer();

    let n: usize = scan.token();
    let q: usize = scan.token();

    let nodes: Vec<RangeAffineSumNode> = (0..n)
        .map(|_| RangeAffineSumNode::new(scan.token()))
        .collect();
    let mut seg = LazySegTree::from(&nodes);

    for _ in 0..q {
        let query_type: u8 = scan.token();
        let l: usize = scan.token();
        let r: usize = scan.token();

        if query_type == 0 {
            let a: i64 = scan.token();
            let b: i64 = scan.token();
            seg.update(
                l,
                r - 1,
                RangeAffineLazyNode {
                    a: a.into(),
                    b: b.into(),
                },
            );
        } else {
            writeln!(out, "{}", seg.query(l, r - 1).val).unwrap();
        }
    }
}
