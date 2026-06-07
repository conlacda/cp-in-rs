// ANCHOR: main
use std::mem::swap;

/// Polynomial rolling hash for sequences.
///
/// # Usage
/// ```rust
/// use rs_space::datastructure::persistent_dsu::PersistentDsu;
/// let mut dsu = PersistentDsu::new(5);
/// dsu.merge(1, 2);
/// assert_eq!(dsu.find(1), dsu.find(2));
/// assert!(dsu.is_same(1, 2));
/// dsu.rollback();
/// assert_eq!(dsu.find(1), 1);
/// assert_eq!(dsu.find(2), 2);
/// ```
pub struct PersistentDsu {
    ccnum: usize,
    rank: Vec<usize>,
    parent: Vec<usize>,
    op: Vec<(usize, usize, usize, usize)>,
}

impl PersistentDsu {
    pub fn new(n: usize) -> Self {
        Self {
            ccnum: n,
            rank: vec![0; n],
            parent: (0..n).collect(),
            op: vec![],
        }
    }

    pub fn find(&self, a: usize) -> usize {
        if a == self.parent[a] {
            a
        } else {
            self.find(self.parent[a])
        }
    }

    pub fn merge(&mut self, mut a: usize, mut b: usize) -> bool {
        a = self.find(a);
        b = self.find(b);
        if a == b {
            return false;
        }
        if self.rank[a] < self.rank[b] {
            swap(&mut a, &mut b);
        }
        self.op.push((a, b, self.rank[a], self.rank[b]));
        self.ccnum -= 1;
        self.parent[b] = a;
        if self.rank[a] == self.rank[b] {
            self.rank[a] += 1;
        }
        true
    }

    pub fn rollback(&mut self) {
        if let Some((a, b, rank_a, rank_b)) = self.op.pop() {
            self.ccnum += 1;
            self.parent[a] = a;
            self.parent[b] = b;
            self.rank[a] = rank_a;
            self.rank[b] = rank_b;
        }
    }

    pub fn is_same(&self, a: usize, b: usize) -> bool {
        self.find(a) == self.find(b)
    }
}
// ANCHOR_END: main

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::dd;
    use crate::random::Random;

    #[test]
    fn test_persistent_dsu() {
        let mut r = Random::new();
        let n = r.num(10..=2000);
        let mut dsu = PersistentDsu::new(n);
        let mut trivial: Vec<usize> = (0..n).collect();
        let q = n;
        for _ in 0..q {
            let t = r.num(0..=2);
            let u = r.num(0..n);
            let v = r.num(0..n);
            // merge
            if t == 0 {
                dsu.merge(u, v);
                let val_u = trivial[u];
                let val_v = trivial[v];
                for i in 0..n {
                    if trivial[i] == val_u || trivial[i] == val_v {
                        trivial[i] = val_u;
                    }
                }
            } else if t == 1 {
                // check same
                let same: bool = dsu.is_same(u, v);
                assert!(same == (trivial[u] == trivial[v]));
            } else {
                let ccnum = dsu.ccnum;
                let rank = dsu.rank.clone();
                let parent = dsu.parent.clone();

                let mut merged_count = 0;
                for _ in 0..5 {
                    let u = r.num(0..n);
                    let v = r.num(0..n);
                    if dsu.merge(u, v) {
                        merged_count += 1;
                    }
                }
                for _ in 0..merged_count {
                    dsu.rollback();
                }
                assert_eq!(ccnum, dsu.ccnum);
                assert_eq!(rank, dsu.rank);
                assert_eq!(parent, dsu.parent);
            }
        }
    }
}
