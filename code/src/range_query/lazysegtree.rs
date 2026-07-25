// ANCHOR: lazysegtree
use crate::range_query::segtree::Node;

pub trait LazyNode<N: Node>: Default + Copy {
    fn compose(&mut self, other: &Self);
    fn apply_to(&self, node: &mut N);
}

#[derive(Default)]
pub struct LazySegTree<N, L> {
    len: usize,
    size: usize,
    log: usize,
    data: Vec<N>,
    lazy: Vec<L>,
}

impl<N, L> LazySegTree<N, L>
where
    N: Node,
    L: LazyNode<N>,
{
    pub fn from(v: &[N]) -> Self {
        let len = v.len();
        let size = len.next_power_of_two();
        let log: usize = size.trailing_zeros() as usize;
        let mut data: Vec<N> = vec![N::default(); 2 * size];
        let lazy: Vec<L> = vec![L::default(); size];
        data[size..size + len].copy_from_slice(v);
        for i in (1..size).rev() {
            data[i] = data[2 * i].combine(&data[2 * i + 1]);
        }
        Self {
            len,
            size,
            log,
            data,
            lazy,
        }
    }

    fn pull(&mut self, k: usize) {
        self.data[k] = self.data[2 * k].combine(&self.data[2 * k + 1]);
    }

    fn all_apply(&mut self, k: usize, lznode: L) {
        lznode.apply_to(&mut self.data[k]);
        if k < self.size {
            self.lazy[k].compose(&lznode);
        }
    }

    fn push(&mut self, k: usize) {
        self.all_apply(2 * k, self.lazy[k]);
        self.all_apply(2 * k + 1, self.lazy[k]);
        self.lazy[k] = L::default();
    }

    pub fn set(&mut self, mut p: usize, node: N) {
        assert!(p < self.len);
        p += self.size;
        for i in (1..=self.log).rev() {
            self.push(p >> i);
        }
        self.data[p] = node;
        for i in 1..=self.log {
            self.pull(p >> i);
        }
    }

    /// Returns the aggregate over the inclusive range `[l, r]`.
    pub fn query(&mut self, mut l: usize, mut r: usize) -> N {
        assert!(l <= r && r < self.len);
        r += 1;
        l += self.size;
        r += self.size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        let mut left_segment = N::default();
        let mut right_segment = N::default();
        while l < r {
            if (l & 1) != 0 {
                left_segment = left_segment.combine(&self.data[l]);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                right_segment = self.data[r].combine(&right_segment);
            }
            l >>= 1;
            r >>= 1;
        }
        left_segment.combine(&right_segment)
    }

    pub fn query_all(&self) -> N {
        self.data[1]
    }

    /// Applies `lznode` to the inclusive range `[l, r]`.
    pub fn update(&mut self, mut l: usize, mut r: usize, lznode: L) {
        assert!(l <= r && r < self.len);
        r += 1;
        l += self.size;
        r += self.size;
        for i in (1..=self.log).rev() {
            if ((l >> i) << i) != l {
                self.push(l >> i);
            }
            if ((r >> i) << i) != r {
                self.push((r - 1) >> i);
            }
        }
        let l2 = l;
        let r2 = r;
        while l < r {
            if (l & 1) != 0 {
                self.all_apply(l, lznode);
                l += 1;
            }
            if (r & 1) != 0 {
                r -= 1;
                self.all_apply(r, lznode);
            }
            l >>= 1;
            r >>= 1;
        }
        l = l2;
        r = r2;
        for i in 1..=self.log {
            if ((l >> i) << i) != l {
                self.pull(l >> i);
            }
            if ((r >> i) << i) != r {
                self.pull((r - 1) >> i);
            }
        }
    }
}
// ANCHOR_END: lazysegtree

// ANCHOR: range_affine
// Query sum
// Update node = a * node + b;
const MOD: i64 = 998244353;
#[derive(Default, Clone, Copy)]
pub struct RangeAffineSumNode {
    pub val: i64,
    pub size: usize,
}

impl Node for RangeAffineSumNode {
    fn new(val: i64) -> Self {
        Self {
            val: val.rem_euclid(MOD),
            size: 1,
        }
    }
    fn combine(&self, other: &Self) -> Self {
        Self {
            val: (self.val + other.val) % MOD,
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
        Self { a: 1, b: 0 }
    }
}

impl LazyNode<RangeAffineSumNode> for RangeAffineLazyNode {
    fn compose(&mut self, other: &Self) {
        self.b = (other.a * self.b + other.b) % MOD;
        self.a = (other.a * self.a) % MOD;
    }

    fn apply_to(&self, node: &mut RangeAffineSumNode) {
        node.val = (self.a * node.val + self.b * node.size as i64) % MOD;
    }
}
// ANCHOR_END: range_affine

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn test_range_affine() {
        let mut r = Random::new();
        let n = r.num(1000..10000);
        let mut array = r.vector(n, -5_000_000..5_000_000);
        let nodes: Vec<RangeAffineSumNode> =
            array.iter().copied().map(RangeAffineSumNode::new).collect();
        let mut seg: LazySegTree<RangeAffineSumNode, RangeAffineLazyNode> =
            LazySegTree::from(&nodes);
        let q = 1000;
        for _ in 0..q {
            let left = r.num(0..n);
            let right = r.num(left..n);
            let query_type = r.num(0..3);
            if query_type == 0 {
                // query
                let correct = array[left..=right].iter().sum::<i64>().rem_euclid(MOD);
                assert_eq!(correct, seg.query(left, right).val);
            } else if query_type == 1 {
                // update
                let a = r.num(0..100);
                let b = r.num(0..1000);
                for i in left..=right {
                    array[i] = (a * array[i] + b) % MOD;
                }
                seg.update(left, right, RangeAffineLazyNode { a, b });
            } else if query_type == 2 {
                // set
                let index = r.num(0..n);
                let value: i64 = r.num(-5_000_000..5_000_000);
                array[index] = value.rem_euclid(MOD);
                seg.set(index, RangeAffineSumNode::new(value));
            }
            let array_sum = array.iter().sum::<i64>().rem_euclid(MOD);
            assert_eq!(array_sum, seg.query(0, seg.len - 1).val);
            assert_eq!(array_sum, seg.query_all().val);
        }
    }
}
