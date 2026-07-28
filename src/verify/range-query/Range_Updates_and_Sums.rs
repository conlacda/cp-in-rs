use rs_space::range_query::lazysegtree::LazyNode;
use rs_space::range_query::lazysegtree::LazySegTree;
use rs_space::range_query::segtree::Node;
use rs_space::sw::{scanner, writer};
use std::io::Write;

#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::dbg;
#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::set_limit::timeout_secs;

#[derive(Default, Clone, Copy)]
pub struct RangeAffineSumNode {
    pub val: i64,
    pub size: usize,
}

impl Node for RangeAffineSumNode {
    fn new(val: i64) -> Self {
        Self {
            val: val.into(),
            size: 1,
        }
    }
    fn combine(&self, other: &Self) -> Self {
        Self {
            val: self.val + other.val,
            size: self.size + other.size,
        }
    }
}

#[derive(Copy, Clone)]
pub struct RangeAffineLazyNode {
    pub a: i64,
    pub b: i64,
}

impl Default for RangeAffineLazyNode {
    fn default() -> Self {
        Self {
            a: 1.into(),
            b: 0.into(),
        }
    }
}

impl LazyNode<RangeAffineSumNode> for RangeAffineLazyNode {
    fn compose(&mut self, other: &Self) {
        self.b = other.a * self.b + other.b;
        self.a *= other.a;
    }

    fn apply_to(&self, node: &mut RangeAffineSumNode) {
        node.val = node.val * self.a + self.b * node.size as i64;
    }
}

fn main() {
    #[cfg(feature = "local")]
    timeout_secs(5);
    let mut scan = scanner();
    let mut out = writer();
    let n: usize = scan.token();
    let q: usize = scan.token();
    let nodes: Vec<RangeAffineSumNode> = (0..n)
        .map(|_| RangeAffineSumNode::new(scan.token()))
        .collect();
    let mut seg = LazySegTree::from(&nodes);
    for _ in 0..q {
        let t: i8 = scan.token();
        let mut l: usize = scan.token();
        let mut r: usize = scan.token();
        l -= 1;
        r -= 1;
        if t == 1 {
            let val: i64 = scan.token();
            seg.update(
                l,
                r,
                RangeAffineLazyNode {
                    a: 1.into(),
                    b: val.into(),
                },
            );
        } else if t == 2 {
            let val: i64 = scan.token();
            seg.update(
                l,
                r,
                RangeAffineLazyNode {
                    a: 0.into(),
                    b: val.into(),
                },
            )
        } else if t == 3 {
            let sum = seg.query(l, r).val;
            writeln!(out, "{}", sum).unwrap();
        }
    }
}
