// ANCHOR: segtree
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
    /// use rs_space::segtree::{SegTree, MaxNode};
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
// ANCHOR_END: segtree

// ANCHOR: SumNode
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
// ANCHOR_END: SumNode

// ANCHOR: MinNode
#[derive(Default, Clone, Copy)]
pub struct MinNode {
    pub val: i64,
    pub has_value: bool,
}

impl MinNode {
    pub fn new(val: i64) -> Self {
        Self {
            val,
            has_value: true,
        }
    }
}

impl Node for MinNode {
    fn combine(&self, other: Self) -> Self {
        if !self.has_value {
            return other;
        }
        if !other.has_value {
            return *self;
        }
        Self::new(self.val.min(other.val))
    }
}
// ANCHOR_END: MinNode

// ANCHOR: MaxNode
#[derive(Default, Clone, Copy)]
pub struct MaxNode {
    pub val: i64,
    pub has_value: bool,
}

impl MaxNode {
    pub fn new(val: i64) -> Self {
        Self {
            val,
            has_value: true,
        }
    }
}

impl Node for MaxNode {
    fn combine(&self, other: Self) -> Self {
        if !self.has_value {
            return other;
        }
        if !other.has_value {
            return *self;
        }
        Self::new(self.val.max(other.val))
    }
}
// ANCHOR_END: MaxNode

// ANCHOR: Subrange
#[derive(Default, Clone, Copy)]
pub struct Subrange {
    pub max_mid: i64,
    pub min_mid: i64,
    pub max_pref: i64,
    pub min_pref: i64,
    pub max_suf: i64,
    pub min_suf: i64,
    pub whole: i64,
    pub has_value: bool,
}

impl Subrange {
    pub fn new(val: i64) -> Self {
        Self {
            max_mid: val,
            min_mid: val,
            max_pref: val,
            min_pref: val,
            max_suf: val,
            min_suf: val,
            whole: val,
            has_value: true,
        }
    }
}

impl Node for Subrange {
    fn combine(&self, other: Self) -> Self {
        if !self.has_value {
            return other;
        }
        if !other.has_value {
            return *self;
        }
        Self {
            max_mid: self
                .max_mid
                .max(other.max_mid)
                .max(self.max_suf + other.max_pref),
            min_mid: self
                .min_mid
                .min(other.min_mid)
                .min(self.min_suf + other.min_pref),
            max_pref: self.max_pref.max(other.max_pref + self.whole),
            min_pref: self.min_pref.min(other.min_pref + self.whole),
            max_suf: other.max_suf.max(self.max_suf + other.whole),
            min_suf: other.min_suf.min(self.min_suf + other.whole),
            whole: self.whole + other.whole,
            has_value: true,
        }
    }
}
// ANCHOR_END: Subrange

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn test_sum_node() {
        let mut rand = Random::new();
        let n = 256;
        let mut values = rand.vector(n, i32::MIN as i64..i32::MAX as i64);
        let nodes: Vec<SumNode> = values.iter().map(|&val| SumNode::new(val)).collect();
        let mut seg = SegTree::from(&nodes);

        for _ in 0..500 {
            if rand.bool() {
                let index = rand.num(0..n);
                let val = rand.num(i32::MIN as i64..i32::MAX as i64);
                values[index] = val;
                seg.set(index, &SumNode::new(val));
            } else {
                let l = rand.num(0..n);
                let r = rand.num(l..n);
                let expected: i64 = values[l..=r].iter().sum();
                let actual = seg.query(l, r);
                assert!(actual.has_value);
                assert_eq!(actual.val, expected);
                assert_eq!(seg.at(l).val, values[l]);
            }
        }
    }

    #[test]
    fn test_min_node() {
        let mut rand = Random::new();
        let n = 256;
        let mut values = rand.vector(n, i32::MIN as i64..i32::MAX as i64);
        let nodes: Vec<MinNode> = values.iter().map(|&val| MinNode::new(val)).collect();
        let mut seg = SegTree::from(&nodes);

        for _ in 0..500 {
            if rand.bool() {
                let index = rand.num(0..n);
                let val = rand.num(i32::MIN as i64..i32::MAX as i64);
                values[index] = val;
                seg.set(index, &&MinNode::new(val));
            } else {
                let l = rand.num(0..n);
                let r = rand.num(l..n);
                let expected = *values[l..=r].iter().min().unwrap();
                let actual = seg.query(l, r);
                assert!(actual.has_value);
                assert_eq!(actual.val, expected);
            }
        }
    }

    #[test]
    fn test_find_right_sum_node() {
        let mut rand = Random::new();
        let n = 256;
        let values = rand.vector(n, 0..=20);
        let nodes: Vec<SumNode> = values.iter().map(|&val| SumNode::new(val)).collect();
        let seg = SegTree::from(&nodes);

        for _ in 0..500 {
            let start = rand.num(0..n);
            let target = rand.num(0..(values[start..].iter().sum::<i64>() + 5) as usize) as i64;
            let mut acc = 0;
            let expected = (start..n).find(|&index| {
                acc += values[index];
                acc >= target
            });
            let actual = seg.find_right(start, |node| node.val >= target);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_find_right_min_node() {
        let mut rand = Random::new();
        let n = 256;
        let values = rand.vector(n, i32::MIN as i64..i32::MAX as i64);
        let nodes: Vec<MinNode> = values.iter().map(|&val| MinNode::new(val)).collect();
        let seg = SegTree::from(&nodes);

        for _ in 0..500 {
            let start = rand.num(0..n);
            let target = rand.num(i32::MIN as i64..i32::MAX as i64);
            let mut current_min = i64::MAX;
            let expected = (start..n).find(|&index| {
                current_min = current_min.min(values[index]);
                current_min <= target
            });
            let actual = seg.find_right(start, |node| node.has_value && node.val <= target);
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn test_find_left_sum_node() {
        let mut rand = Random::new();
        let n = 5;
        let values = rand.vector(n, 0..i32::MAX as i64);
        let nodes: Vec<SumNode> = values.iter().map(|&val| SumNode::new(val)).collect();
        let seg = SegTree::from(&nodes);

        for _ in 0..500 {
            let end = rand.num(0..n);
            let target = rand.num(0..(values[..=end].iter().sum::<i64>() + 5) as usize) as i64;
            let mut acc = 0;
            let expected = (0..=end).rev().find(|&index| {
                acc += values[index];
                acc >= target
            });
            let actual = seg.find_left(end, |node| node.val >= target);
            dbg!(actual, expected);
            assert_eq!(actual, expected);
        }
    }

    fn calc_subrange(values: &[i64]) -> Subrange {
        let whole: i64 = values.iter().sum();
        let mut prefix = 0;
        let mut max_pref = i64::MIN;
        let mut min_pref = i64::MAX;
        for &value in values {
            prefix += value;
            max_pref = max_pref.max(prefix);
            min_pref = min_pref.min(prefix);
        }

        let mut suffix = 0;
        let mut max_suf = i64::MIN;
        let mut min_suf = i64::MAX;
        for &value in values.iter().rev() {
            suffix += value;
            max_suf = max_suf.max(suffix);
            min_suf = min_suf.min(suffix);
        }

        let mut max_mid = i64::MIN;
        let mut min_mid = i64::MAX;
        for l in 0..values.len() {
            let mut sum = 0;
            for &value in &values[l..] {
                sum += value;
                max_mid = max_mid.max(sum);
                min_mid = min_mid.min(sum);
            }
        }

        Subrange {
            max_mid,
            min_mid,
            max_pref,
            min_pref,
            max_suf,
            min_suf,
            whole,
            has_value: true,
        }
    }

    #[test]
    fn test_subrange() {
        let mut rand = Random::new();
        let n = 64;
        let mut values = rand.vector(n, -100..=100);
        let nodes: Vec<Subrange> = values.iter().map(|&val| Subrange::new(val)).collect();
        let mut seg = SegTree::from(&nodes);

        for _ in 0..500 {
            if rand.bool() {
                let index = rand.num(0..n);
                let val = rand.num(-100..100);
                values[index] = val;
                seg.set(index, &Subrange::new(val));
            } else {
                let l = rand.num(0..n);
                let r = rand.num(l..n);
                let actual = seg.query(l, r);
                let expected = calc_subrange(&values[l..=r]);
                assert!(actual.has_value);
                assert_eq!(actual.max_mid, expected.max_mid);
                assert_eq!(actual.min_mid, expected.min_mid);
                assert_eq!(actual.max_pref, expected.max_pref);
                assert_eq!(actual.min_pref, expected.min_pref);
                assert_eq!(actual.max_suf, expected.max_suf);
                assert_eq!(actual.min_suf, expected.min_suf);
                assert_eq!(actual.whole, expected.whole);
            }
        }
    }
}
