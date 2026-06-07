// ANCHOR: main
pub struct WeightDsu {
    n: usize,
    parent: Vec<usize>,
    rank: Vec<usize>,
    height: Vec<i64>,
}

impl WeightDsu {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            parent: (0..n).collect(),
            rank: vec![0; n],
            height: vec![0; n],
        }
    }

    pub fn find(&mut self, v: usize) -> usize {
        assert!(v < self.n);
        if v == self.parent[v] {
            return v;
        }
        let root = self.find(self.parent[v]);
        self.height[v] += self.height[self.parent[v]];
        self.parent[v] = root;
        root
    }

    /// height[a] - height[b] = dist
    /// if a, b is already in the same group, return false
    /// otherwise return true
    pub fn merge(&mut self, mut a: usize, mut b: usize, mut dist: i64) -> bool {
        let mut ra = self.find(a);
        let mut rb = self.find(b);
        if ra == rb {
            return false;
        }
        dist = -self.distance(a, ra) + dist + self.distance(b, rb);
        a = ra;
        b = rb;
        if self.rank[ra] < self.rank[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        if self.rank[ra] == self.rank[rb] {
            self.rank[ra] += 1;
        }
        if ra == a {
            self.height[a] = 0;
            self.height[b] = -dist;
        } else {
            self.height[a] = dist;
            self.height[b] = 0;
        }
        true
    }

    // tính khoảng cách của 2 node. (dist(a, b) = - dist(b, a))
    pub fn distance(&mut self, u: usize, v: usize) -> i64 {
        if self.find(u) != self.find(v) {
            return i64::MAX;
        }
        self.height[u] - self.height[v]
    }

    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.find(u) == self.find(v)
    }
}
// ANCHOR_END: main

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn test_dsu() {
        let mut r = Random::new();
        // let n = r.num(10, 2000);
        let n = r.num(100..2000);
        let mut dsu = WeightDsu::new(n);
        let mut trivial_parent: Vec<usize> = (0..n).collect();
        let mut trivial_height: Vec<i64> = vec![0; n];
        // let mut trivial_height = vec![0i64; n];
        let q = n;
        for _ in 0..q {
            let t = r.num(0..=2);
            let a = r.num(0..n);
            let b = r.num(0..n);
            let dist: i64 = r.num(-50..50);
            // merge
            if t == 0 {
                let ra = trivial_parent[a];
                let rb = trivial_parent[b];
                let merged = dsu.merge(a, b, dist);
                if ra == rb {
                    assert!(!merged);
                    continue;
                }
                let delta = trivial_height[a] - trivial_height[b];
                /*
                a = 12
                b = 5 -> delta = 7
                dist = 10 => b + delta - dist
                */

                for i in 0..n {
                    if trivial_parent[i] == rb {
                        trivial_parent[i] = ra;
                        trivial_height[i] += delta - dist;
                    }
                }
            } else if t == 1 {
                // check same
                let same: bool = dsu.is_same(a, b);
                assert!(same == (trivial_parent[a] == trivial_parent[b]));
            } else if t == 2 {
                // check distance
                let ra = trivial_parent[a];
                let rb = trivial_parent[b];
                let trivial_distance = if ra != rb {
                    i64::MAX
                } else {
                    trivial_height[a] - trivial_height[b]
                };
                let dsu_distance = dsu.distance(a, b);
                assert!(trivial_distance == dsu_distance);
            }
        }
    }
}
