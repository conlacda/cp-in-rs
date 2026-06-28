// ANCHOR: main
#[derive(Default, Debug)]
pub struct RMQ<T: Clone> {
    values: Vec<T>,
    range_low: Vec<Vec<usize>>,
    max_mode: bool,
}

impl<T: Clone + Ord> RMQ<T> {
    pub fn new(values: &[T], max_mode: bool) -> Self {
        let mut rmq = Self {
            values: values.to_vec(),
            range_low: Vec::new(),
            max_mode,
        };
        rmq.build();
        rmq
    }

    pub fn build(&mut self) {
        let n = self.values.len();
        let levels = self.floor_log2(n) + 1;
        self.range_low.resize(levels, Vec::new());
        for k in 0..levels {
            self.range_low[k].resize(n - (1 << k) + 1, 0);
        }
        for i in 0..n {
            self.range_low[0][i] = i;
        }
        for k in 1..levels {
            for i in 0..=n - (1 << k) {
                self.range_low[k][i] = self.better_index(
                    self.range_low[k - 1][i],
                    self.range_low[k - 1][i + (1 << (k - 1))],
                );
            }
        }
    }

    fn floor_log2(&self, x: usize) -> usize {
        assert!(x > 0);
        (usize::BITS - 1 - x.leading_zeros()) as usize
        // Some(x.ilog2() as usize)
    }

    fn better_index(&self, a: usize, b: usize) -> usize {
        // use <= for the case when values[a] == values[b] then return a
        if (self.max_mode && self.values[b] < self.values[a])
            || (!self.max_mode && self.values[a] < self.values[b])
        {
            a
        } else {
            b
        }
    }

    pub fn query_index(&self, l: usize, mut r: usize) -> usize {
        let n = self.values.len();
        assert!(l <= r && r <= n);
        r += 1;
        if l == r {
            return l;
        }
        let level = self.floor_log2(r - l);
        self.better_index(
            self.range_low[level][l],
            self.range_low[level][r - (1 << level)],
        )
    }
    pub fn query_value(&self, l: usize, r: usize) -> T {
        self.values[self.query_index(l, r)].clone()
    }
}
// ANCHOR_END: main

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn test_max_mode() {
        let mut r = Random::new();
        let v = r.vector(5000, i64::MIN..i64::MAX);
        let n = v.len();
        let rmq: RMQ<i64> = RMQ::new(&v, true);
        for _ in 0..500 {
            let l = r.num(0..n);
            let r = r.num(l..n);
            let mut max_index = l;
            for i in l..=r {
                if v[max_index] <= v[i] {
                    max_index = i;
                }
            }
            assert!(max_index == rmq.query_index(l, r));

            if let &Some(&max_value) = &v[l..=r].iter().max() {
                assert!(max_value == rmq.query_value(l, r));
            }
        }
    }

    #[test]
    fn test_min_mode() {
        let mut r = Random::new();
        let v = r.vector(5000, i64::MIN..i64::MAX);
        let n = v.len();
        let rmq: RMQ<i64> = RMQ::new(&v, false);
        for _ in 0..500 {
            let l = r.num(0..n);
            let r = r.num(l..n);
            let mut min_index = l;
            for i in l..=r {
                // use >= to set min_index = right if values[left] == values[right]
                if v[min_index] >= v[i] {
                    min_index = i;
                }
            }
            assert!(min_index == rmq.query_index(l, r));

            if let &Some(&min_value) = &v[l..=r].iter().min() {
                assert!(min_value == rmq.query_value(l, r));
            }
        }
    }

    #[test]
    fn test_narrow_range() {
        let mut r = Random::new();
        let v = r.vector(5000, -10..=10);
        let n = v.len();
        let rmq: RMQ<i64> = RMQ::new(&v, true);
        for _ in 0..500 {
            let l = r.num(0..n);
            let r = r.num(l..n);
            let mut max_index = l;
            for i in l..=r {
                if v[max_index] <= v[i] {
                    max_index = i;
                }
            }
            assert!(max_index == rmq.query_index(l, r));
            if let &Some(&max_value) = &v[l..=r].iter().max() {
                assert!(max_value == rmq.query_value(l, r));
            }
        }
    }
}
