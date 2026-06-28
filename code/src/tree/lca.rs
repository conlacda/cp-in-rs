// ANCHOR: main
use crate::range_query::rmq::RMQ;
use crate::recursive::Callable2;
use crate::recursive::RecursiveFunction2;

struct Euler {
    first_in: Vec<usize>, // first_in[node] = first index of node in Euler tour
    tour: Vec<usize>,     // tour[order] = node
    depth: Vec<usize>,    // depth[i] = depth of tour[i] in the tree
}

pub struct LCA {
    first_in: Vec<usize>,
    tour: Vec<usize>,
    rmq: RMQ<usize>,
}

impl LCA {
    pub fn new(tree: &[Vec<usize>], root: usize) -> Self {
        let euler = Self::make_eulertour(tree, root);
        Self {
            first_in: euler.first_in,
            tour: euler.tour,
            rmq: RMQ::new(&euler.depth, false),
        }
    }

    pub fn lca(&self, u: usize, v: usize) -> usize {
        let mut fu = self.first_in[u];
        let mut fv = self.first_in[v];
        if fu > fv {
            std::mem::swap(&mut fu, &mut fv);
        }
        let index = self.rmq.query_index(fu, fv);
        self.tour[index]
    }

    fn make_eulertour(tree: &[Vec<usize>], root: usize) -> Euler {
        let n = tree.len();
        let mut dep: usize = 0;
        let mut first_in: Vec<usize> = vec![usize::MAX; n];
        let mut tour = vec![];
        let mut depth = vec![];
        let mut dfs = RecursiveFunction2::new(|dfs, node: usize, parent: usize| {
            dep += 1;
            if first_in[node] == usize::MAX {
                first_in[node] = tour.len();
            }
            depth.push(dep);
            tour.push(node);
            for &child in &tree[node] {
                if child == parent {
                    continue;
                }
                dfs.call(child, node);
                dep -= 1;
                tour.push(node);
                depth.push(dep);
            }
        });
        dfs.call(root, root);
        Euler {
            first_in,
            tour,
            depth,
        }
    }
}
// ANCHOR_END: main

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;
    use std::collections::VecDeque;

    fn get_parent(tree: &[Vec<usize>], root: usize) -> Vec<usize> {
        let n = tree.len();
        let mut parent = vec![usize::MAX; n];
        parent[root] = root;
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(root);
        while let Some(node) = q.pop_front() {
            for &child in &tree[node] {
                if parent[child] == usize::MAX {
                    parent[child] = node;
                    q.push_back(child);
                }
            }
        }
        parent
    }

    fn trivial_lca(tree: &[Vec<usize>], root: usize, mut u: usize, mut v: usize) -> usize {
        let parent = get_parent(&tree, root);
        let mut parent_u = vec![u];
        while u != root {
            u = parent[u];
            parent_u.push(u);
        }
        parent_u.reverse();
        let mut parent_v = vec![v];
        while v != root {
            v = parent[v];
            parent_v.push(v);
        }
        parent_v.reverse();
        let mut res = root;
        for i in 0..parent_u.len().min(parent_v.len()) {
            if parent_u[i] != parent_v[i] {
                break;
            }
            res = parent_u[i];
        }
        res
    }

    #[test]
    fn test() {
        let mut r = Random::new();
        let n = r.num(10000..100_000);
        let tree = r.tree(n);
        let lca = LCA::new(&tree, 0);
        let q = 100;
        let root = 0;
        for _ in 0..q {
            let u = r.num(0..n);
            let v = r.num(0..n);
            let actual = lca.lca(u, v);
            let expected = trivial_lca(&tree, root, u, v);
            assert_eq!(actual, expected);
        }
    }
}
