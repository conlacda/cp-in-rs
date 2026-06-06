// START_TEMPLATE
pub struct DsuNode {
    size: i64,
}

impl DsuNode {
    fn new() -> Self {
        Self { size: 1 }
    }
}

#[allow(clippy::upper_case_acronyms)]
pub struct DSU {
    n: usize,
    parent: Vec<usize>,
    data: Vec<DsuNode>,
}

impl DSU {
    pub fn new(n: usize) -> Self {
        Self {
            n,
            parent: (0..n).collect(),
            data: (0..n).map(|_| DsuNode::new()).collect::<Vec<_>>(),
        }
    }
    pub fn find(&mut self, v: usize) -> usize {
        assert!(v < self.n);
        if v == self.parent[v] {
            return v;
        }
        self.parent[v] = self.find(self.parent[v]);
        self.parent[v]
    }

    /// merge u and v into a set
    ///
    /// return false if u, v is already in a same set
    /// otherwise, return true
    pub fn merge(&mut self, mut u: usize, mut v: usize) -> bool {
        u = self.find(u);
        v = self.find(v);
        if u == v {
            return false;
        }
        if self.data[u].size < self.data[v].size {
            std::mem::swap(&mut u, &mut v);
        }
        self.parent[v] = u;
        self.data[u].size += self.data[v].size;
        true
    }
    pub fn is_same(&mut self, u: usize, v: usize) -> bool {
        self.find(u) == self.find(v)
    }
    pub fn node_mut(&mut self, index: usize) -> &mut DsuNode {
        let root = self.find(index);
        &mut self.data[root]
    }
}
// END_TEMPLATE

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::dd;
    use crate::random::Random;

    #[test]
    fn test_dsu() {
        let mut r = Random::new();
        let n = r.num(10..=2000);
        let mut dsu = DSU::new(n);
        let mut trivial: Vec<usize> = (0..n).collect();
        let q = n;
        for _ in 0..q {
            let t = r.bool();
            let u = r.num(0..n);
            let v = r.num(0..n);
            // merge
            if t {
                dsu.merge(u, v);
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
