// ANCHOR: main
use std::mem::swap;

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
            let t = r.bool();
            let u = r.num(0..n);
            let v = r.num(0..n);
            // merge
            if t {
                let merged = dsu.merge(u, v);
                if merged {
                    dsu.rollback();
                    dsu.merge(u, v);
                }
                let val_u = trivial[u];
                let val_v = trivial[v];
                for i in 0..n {
                    if trivial[i] == val_u || trivial[i] == val_v {
                        trivial[i] = val_u;
                    }
                }
            } else {
                // check same
                let same: bool = dsu.is_same(u, v);
                assert!(same == (trivial[u] == trivial[v]));
            }
        }
    }
}
