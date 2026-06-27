// ANCHOR: main
pub struct BinaryJumping {
    max_depth: usize,
    up: Vec<Vec<usize>>,
}

impl BinaryJumping {
    pub fn new(parent: &[usize], max_depth: usize) -> Self {
        assert!(max_depth != 0);
        let n = parent.len();
        let log2 = max_depth.next_power_of_two().ilog2() as usize;
        let mut up = vec![vec![0; n]; log2 + 1];
        for i in 0..n {
            up[0][i] = parent[i];
        }
        for k in 1..=log2 {
            for i in 0..n {
                up[k][i] = up[k - 1][up[k - 1][i]];
            }
        }
        Self { up, max_depth }
    }
    pub fn kth_parent(&self, mut node: usize, mut k: usize) -> usize {
        assert!(k <= self.max_depth, "k exceeds max_depth");
        let mut bit = 0;
        while k != 0 {
            if k & 1 != 0 {
                node = self.up[bit][node];
            }
            bit += 1;
            k >>= 1;
        }
        node
    }
}
// ANCHOR_END: main

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    fn gen_parent(node_num: usize) -> Vec<usize> {
        let mut r = Random::new();
        let tree = r.tree(node_num);
        let mut parent = vec![0; node_num];
        for i in 0..node_num {
            for &j in tree[i].iter() {
                parent[j] = i;
            }
        }
        parent[0] = node_num - 1;
        parent
    }

    #[test]
    fn test() {
        let mut r = Random::new();
        let n = r.num(100..1000);
        let parent = gen_parent(n);
        let max_depth = 2_000;
        let bj = BinaryJumping::new(&parent, max_depth);
        for _ in 0..100 {
            let mut node = r.num(0..n);
            let k = r.num(0..=max_depth);
            let expected = bj.kth_parent(node, k);
            let actual = (|| {
                for _i in 0..k {
                    node = parent[node];
                }
                node
            })();
            assert_eq!(actual, expected);
        }
    }
}
