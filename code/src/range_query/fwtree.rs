// ANCHOR: RangeUpdatePointQuery

use std::ops::RangeBounds;

/// Fenwick tree for range update point query
///
/// # Usage
/// ```rust
/// use rs_space::range_query::fwtree::FenwickTree;
/// let fw = FenwickTree::new(10);
/// for index in 0..10 {
///     assert!(fw.at(index) == 0);
/// }
///
/// let mut fw = FenwickTree::from(&[1, 2, 3, 4]);
/// fw.add(0, 1, 10);
/// assert!(fw.at(0) == 11);
/// assert!(fw.len() == 4);
/// ```
#[derive(Debug)]
pub struct FenwickTree {
    bit: Vec<i64>,
}

impl FenwickTree {
    pub fn new(n: usize) -> Self {
        Self { bit: vec![0; n] }
    }

    pub fn from(v: &[i64]) -> Self {
        let mut fw = FenwickTree {
            bit: vec![0; v.len()],
        };
        for (index, &val) in v.iter().enumerate() {
            fw.add(index, index, val);
        }
        fw
    }

    pub fn len(&self) -> usize {
        self.bit.len()
    }

    fn internal_update(&mut self, mut i: usize, val: i64) {
        while i < self.len() {
            self.bit[i] += val;
            i |= i + 1;
        }
    }

    // add val to [l..=r]
    pub fn add(&mut self, l: usize, r: usize, val: i64) {
        self.internal_update(l, val);
        if r + 1 < self.len() {
            self.internal_update(r + 1, -val);
        }
    }

    pub fn at(&self, index: usize) -> i64 {
        assert!(index < self.len());
        let mut ans = 0;
        let mut i = index as i64;

        while i >= 0 {
            ans += self.bit[i as usize];
            i = (i & (i + 1)) - 1;
        }

        ans
    }
}
// ANCHOR_END: RangeUpdatePointQuery

// ANCHOR: RangeQueryPointUpdate
/// Fenwick tree for range query point update
///
/// # Usage
/// ```rust
/// use rs_space::range_query::fwtree::FwTree;
/// let fw = FwTree::new(10);
/// for i in 0..10 {
///     assert!(fw.sum(i..=i) == 0);
/// }
///
/// let mut fw = FwTree::from(&[1, 2, 3, 4]);
/// assert!(fw.len() == 4);
/// fw.add(0, 9);
/// fw.set(1, 5);
/// assert_eq!(fw.sum(0..4), 22);
/// assert_eq!(fw.sum(0..=3), 22);
/// assert_eq!(fw.right_index_with_sum_from_k(0, 16), Some(2));
/// assert_eq!(fw.left_index_with_sum_from_k(3, 100), None);
/// ```
#[derive(Debug)]
pub struct FwTree {
    bit: Vec<i64>,
}

impl FwTree {
    pub fn new(n: usize) -> Self {
        Self { bit: vec![0; n] }
    }

    pub fn from(v: &[i64]) -> Self {
        let mut fw = Self {
            bit: vec![0; v.len()],
        };
        for (i, &item) in v.iter().enumerate() {
            fw.add(i, item);
        }
        fw
    }

    pub fn len(&self) -> usize {
        self.bit.len()
    }

    pub fn add(&mut self, mut index: usize, delta: i64) {
        assert!(index < self.len());
        while index < self.len() {
            self.bit[index] += delta;
            index = index | (index + 1);
        }
    }

    pub fn sum<R>(&self, range: R) -> i64
    where
        R: RangeBounds<usize>,
    {
        let l = match range.start_bound() {
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Excluded(&x) => x + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let r = match range.end_bound() {
            std::ops::Bound::Included(&x) => x,
            std::ops::Bound::Excluded(&x) => x - 1,
            std::ops::Bound::Unbounded => self.len() - 1,
        };

        assert!(l <= r && r <= self.len());

        let sum_to = |mut r: usize| -> i64 {
            let mut result = 0;
            loop {
                result += self.bit[r];
                let index: i64 = (r as i64 & (r as i64 + 1)) - 1;
                if index < 0 {
                    break;
                }
                r = index as usize;
            }
            result
        };
        if l == 0 {
            sum_to(r)
        } else {
            sum_to(r) - sum_to(l - 1)
        }
    }

    pub fn set(&mut self, index: usize, val: i64) {
        let diff = val - self.sum(index..=index);
        self.add(index, diff);
    }

    // sum[start_index:R] >= k
    pub fn right_index_with_sum_from_k(&self, start_index: usize, k: i64) -> Option<usize> {
        let mut l = start_index;
        let mut r = self.len() - 1;
        while l < r {
            let mid = (l + r) / 2;
            if self.sum(start_index..=mid) >= k {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        if self.sum(start_index..=r) < k {
            return None;
        }
        Some(l)
    }

    // sum[L:end_index] >= k
    pub fn left_index_with_sum_from_k(&self, end_index: usize, k: i64) -> Option<usize> {
        let mut l = 0;
        let mut r = end_index;
        while l < r {
            let mid = if (l + r).is_multiple_of(2) {
                (l + r) / 2
            } else {
                (l + r).div_ceil(2)
            };
            if self.sum(mid..=end_index) >= k {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        if self.sum(l..=end_index) < k {
            return None;
        }
        Some(r)
    }
}
// ANCHOR_END: RangeQueryPointUpdate

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn test_range_update_point_query_fw() {
        let mut rand = Random::new();
        let n = 10000;
        let mut v = rand.vector(n, 0..=i32::MAX as i64);
        let mut fw = FenwickTree::from(&v);

        let q = 500;
        for _ in 0..q {
            let l = rand.num(0..n);
            let r = rand.num(l..n);
            let val = rand.num(i32::MIN as i64..i32::MAX as i64);

            fw.add(l, r, val);
            for x in &mut v[l..=r] {
                *x += val;
            }
        }

        for i in 0..n {
            assert_eq!(v[i], fw.at(i));
        }
    }

    #[test]
    fn test_range_query_point_update_fw() {
        let mut rand = Random::new();
        let n = 10000;
        let mut v: Vec<i64> = rand.vector(n, 0..=i32::MAX as i64);
        let mut fw = FwTree::from(&v);

        let q = 500;
        // fw.set()
        for _ in 0..q {
            let index = rand.num(0..n);
            let val = rand.num(i32::MIN as i64..i32::MAX as i64);
            fw.set(index, val);
            v[index] = val;
        }
        // fw.add()
        for _ in 0..q {
            let index = rand.num(0..n);
            let val = rand.num(i32::MIN as i64..=i32::MAX as i64);
            fw.add(index, val);
            v[index] += val;
        }
        // assert sum
        for _ in 0..q {
            let l = rand.num(0..n);
            let r = rand.num(l..n);
            assert!(fw.sum(l..=r) == v[l..=r].iter().sum());
        }
    }

    #[test]
    fn test_range_query_point_update_non_positive_fw() {
        let mut rand = Random::new();
        let n = 10000;
        let v: Vec<i64> = rand.vector(n, 0..i32::MAX as i64);
        let fw = FwTree::from(&v);

        let q = 500;

        // right_index_with_sum_from_k
        for _ in 0..q {
            let l = rand.num(0..n);
            let sum_from_l = v[l..].iter().sum();
            let random_sum = rand.num(0..=sum_from_l);
            let r = fw
                .right_index_with_sum_from_k(l, random_sum)
                .expect("implementation of right_index_with_sum_from_k is incorrect");
            let mut s = 0;
            for i in l..r {
                s += v[i];
                assert!(s < random_sum);
            }
            s += v[r];
            assert!(s >= random_sum);
        }
        // left_index_with_sum_from_k
        for _ in 0..q {
            let r = rand.num(0..n);
            let sum_to_r = v[..r].iter().sum();
            let random_sum = rand.num(0..=sum_to_r);
            let l = fw
                .left_index_with_sum_from_k(r, random_sum)
                .expect("implementation of left_index_with_sum_from_k is incorrect");
            let mut s = 0;
            for i in (l + 1)..=r {
                s += v[i];
                assert!(s < random_sum);
            }
            s += v[l];
            assert!(s >= random_sum);
        }
    }
}
