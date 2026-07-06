// ANCHOR: main
use crate::range_query::segtree::{Node, SegTree};
use crate::recursive::{Callable, Callable2};
use crate::recursive::{RecursiveFunction, RecursiveFunction2};
use crate::tree::lca::LCA;
use std::cell::RefCell;

pub struct HLD<T> {
    parent: Vec<usize>,
    depth: Vec<usize>,
    head: Vec<usize>,
    pos: Vec<usize>,
    pre_order: Vec<usize>,
    post_order: Vec<usize>,
    pos2vertex: Vec<usize>,
    segtree: SegTree<T>,
    lca: LCA,
    weight_on_nodes: bool,
}

impl<T> HLD<T>
where
    T: Node,
{
    pub fn new(tree: &[Vec<usize>], weight: &[i64]) -> Self {
        let n = tree.len();
        let heavy = RefCell::new(vec![usize::MAX; n]);
        let parent = RefCell::new(vec![0; n]);
        let mut depth = vec![0; n];
        let mut head = vec![0; n];
        let mut pos = vec![0; n];
        let mut pos2vertex = vec![0; n];
        let mut pre_order = vec![0; n];
        let mut post_order = vec![0; n];
        let mut dfs = RecursiveFunction::new(|dfs, p: usize| {
            let mut size = 1;
            let mut max_c_size = 0;
            for &c in &tree[p] {
                if c != parent.borrow()[p] {
                    parent.borrow_mut()[c] = p;
                    depth[c] = depth[p] + 1;
                    let c_size = dfs.call(c);
                    size += c_size;
                    if c_size > max_c_size {
                        max_c_size = c_size;
                        heavy.borrow_mut()[p] = c;
                    }
                }
            }
            size
        });
        let mut clock = 0;
        let mut decompose = RecursiveFunction2::new(|decompose, u: usize, p: usize| {
            head[u] = p;
            pre_order[u] = clock;
            pos[u] = clock;
            clock += 1;
            if heavy.borrow()[u] != usize::MAX {
                decompose.call(heavy.borrow()[u], p);
            }
            for &v in &tree[u] {
                if v != parent.borrow()[u] && v != heavy.borrow()[u] {
                    decompose.call(v, v);
                }
            }
            post_order[u] = clock;
        });
        dfs.call(0);
        decompose.call(0, 0);
        for v in 0..n {
            pos2vertex[pos[v]] = v;
        }

        let mut nodes: Vec<T> = (0..n).map(|_| Node::new(0)).collect();
        for i in 0..n {
            nodes[pos[i]] = Node::new(weight[i]);
        }
        let segtree = SegTree::from(&nodes);

        let lca = LCA::new(tree, 0);
        let parent_vec = parent.borrow().clone(); // for Rust 2021
        Self {
            parent: parent_vec,
            depth,
            head,
            pos,
            pre_order,
            post_order,
            pos2vertex,
            segtree,
            lca,
            weight_on_nodes: false,
        }
    }

    pub fn set_weight_on_nodes(&mut self, weight_on_nodes: bool) {
        self.weight_on_nodes = weight_on_nodes;
    }

    fn exclude_root_path(&self, mut u: usize, p: usize) -> Vec<(usize, usize)> {
        let mut path: Vec<(usize, usize)> = Vec::new();
        while self.head[p] != self.head[u] {
            path.push((u, self.head[u]));
            u = self.parent[self.head[u]];
        }
        if u != p {
            path.push((u, self.pos2vertex[self.pos[p] + 1]));
        }
        path
    }

    pub fn query_path(&self, u: usize, v: usize) -> T {
        let parent = self.lca.lca(u, v);
        let mut left: T = T::default();
        let u2p: Vec<(usize, usize)> = self.exclude_root_path(u, parent);
        let p2u: Vec<(usize, usize)> = u2p.into_iter().rev().collect();
        for chain in p2u {
            left = left.combine(&self.segtree.query(self.pos[chain.1], self.pos[chain.0]));
        }
        let root: T = self.segtree.query(self.pos[parent], self.pos[parent]);
        let mut right = T::default();
        let v2p = self.exclude_root_path(v, parent);
        let p2v: Vec<(usize, usize)> = v2p.into_iter().rev().collect();
        for chain in p2v {
            right = right.combine(&self.segtree.query(self.pos[chain.1], self.pos[chain.0]));
        }
        if self.weight_on_nodes {
            left.right_to_left().combine(&root).combine(&right)
        } else {
            left.right_to_left().combine(&right)
        }
    }

    pub fn query_subtree(&self, root: usize) -> T {
        let start = if self.weight_on_nodes {
            self.pre_order[root]
        } else {
            self.pre_order[root] + 1
        };
        let end = self.post_order[root] - 1;
        self.segtree.query(start, end)
    }

    pub fn set_node(&mut self, u: usize, node: T) {
        self.segtree.set(self.pos[u], &node);
    }

    pub fn set_edge(&mut self, (u, v): (usize, usize), node: T) {
        let deeper_node = if self.depth[u] > self.depth[v] { u } else { v };
        self.set_node(deeper_node, node);
    }

    pub fn distance(&self, u: usize, v: usize) -> usize {
        let p = self.lca.lca(u, v);
        self.depth[u] + self.depth[v] - 2 * self.depth[p]
    }
}
// ANCHOR_END: main

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;
    use crate::range_query::segtree::{MaxNode, SumNode};
    use crate::tree::lca::LCA;
    use std::collections::HashMap;
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

    fn get_subtree_nodes(tree: &[Vec<usize>], node: usize, parent: usize) -> Vec<usize> {
        let mut subtree = Vec::new();
        let mut dfs = RecursiveFunction2::new(|dfs, node: usize, parent: usize| {
            subtree.push(node);
            for &child in &tree[node] {
                if child != parent {
                    dfs.call(child, node);
                }
            }
        });
        dfs.call(node, parent);
        subtree
    }

    #[test]
    fn test_weight_on_vertices_sum_node() {
        let mut r = Random::new();
        let n = r.num(1000..10000);
        let tree = r.tree(n);
        let mut weight: Vec<i64> = r.vector(n, 0..100_000);
        let mut hld: HLD<SumNode> = HLD::new(&tree, &weight);
        hld.set_weight_on_nodes(true);
        let lca = LCA::new(&tree, 0);
        let parent = get_parent(&tree, 0);
        let q = 1000;
        for _ in 0..q {
            let t = r.num(0..3);
            if t == 0 {
                // Query path
                let mut u: usize = r.num(0..n);
                let mut v: usize = r.num(0..n);
                let common_ancestor = lca.lca(u, v);
                let actual: i64 = hld.query_path(u, v).val;
                let mut expected: i64 = 0;
                while u != common_ancestor {
                    expected += weight[u];
                    u = parent[u];
                }
                while v != common_ancestor {
                    expected += weight[v];
                    v = parent[v];
                }
                expected += weight[common_ancestor];
                assert_eq!(actual, expected);
            } else if t == 1 {
                // Set
                let u: usize = r.num(0..n);
                let val: i64 = r.num(1000..1000_000);
                hld.set_node(u, SumNode::new(val));
                weight[u] = val;
            } else if t == 2 {
                // Query subtree
                let root = r.num(0..n);
                let st_nodes = get_subtree_nodes(&tree, root, parent[root]);
                let sum: i64 = st_nodes.iter().map(|&u| weight[u]).sum();
                assert_eq!(hld.query_subtree(root).val, sum);
            }
        }
    }

    #[test]
    fn test_weight_on_vertices_max_node() {
        let mut r = Random::new();
        let n = r.num(1000..10000);
        let tree = r.tree(n);
        let mut weight: Vec<i64> = r.vector(n, 0..100_000);
        let mut hld: HLD<MaxNode> = HLD::new(&tree, &weight);
        hld.set_weight_on_nodes(true);

        let lca = LCA::new(&tree, 0);
        let parent = get_parent(&tree, 0);
        let q = 1000;
        for _ in 0..q {
            let t = r.num(0..3);
            if t == 0 {
                // Query path
                let mut u: usize = r.num(0..n);
                let mut v: usize = r.num(0..n);
                let common_ancestor = lca.lca(u, v);
                let actual: i64 = hld.query_path(u, v).val;
                let mut expected: i64 = 0;
                while u != common_ancestor {
                    expected = expected.max(weight[u]);
                    u = parent[u];
                }
                while v != common_ancestor {
                    expected = expected.max(weight[v]);
                    v = parent[v];
                }
                expected = expected.max(weight[common_ancestor]);
                assert_eq!(actual, expected);
            } else if t == 1 {
                // Set
                let u: usize = r.num(0..n);
                let val: i64 = r.num(1000..1000_000);
                hld.set_node(u, MaxNode::new(val));
                weight[u] = val;
            } else if t == 2 {
                // Query subtree
                let root = r.num(0..n);
                let st_nodes = get_subtree_nodes(&tree, root, parent[root]);
                let sum = st_nodes.iter().map(|&u| weight[u]).max().unwrap();
                assert_eq!(hld.query_subtree(root).val, sum);
            }
        }
    }

    #[test]
    fn test_weight_on_edge() {
        let mut r = Random::new();
        let n = r.num(1000..10000);
        let tree = r.tree(n);
        let parent = get_parent(&tree, 0);
        let mut edge_weight: HashMap<(usize, usize), i64> = HashMap::new();
        let mut weight = vec![0; n];
        for node in 0..n {
            if node == 0 {
                continue;
            }
            let p = parent[node];
            let w: i64 = r.num(1000..1000_000);
            edge_weight.insert((node, p), w);
            weight[node] = w;
        }
        let mut hld: HLD<SumNode> = HLD::new(&tree, &weight);
        hld.set_weight_on_nodes(false);

        let lca = LCA::new(&tree, 0);
        let q = 1000;
        for _ in 0..q {
            let t = r.num(0..3);
            if t == 0 {
                // Query path
                let mut u: usize = r.num(0..n);
                let mut v: usize = r.num(0..n);
                let common_ancestor = lca.lca(u, v);
                let actual: i64 = hld.query_path(u, v).val;
                let mut expected: i64 = 0;
                while u != common_ancestor {
                    expected += edge_weight.get(&(u, parent[u])).unwrap();
                    u = parent[u];
                }
                while v != common_ancestor {
                    expected += edge_weight.get(&(v, parent[v])).unwrap();
                    v = parent[v];
                }
                assert_eq!(actual, expected);
            } else if t == 1 {
                // Set
                let u = r.num(1..n);
                let val: i64 = r.num(1000..1000_000);
                edge_weight.insert((u, parent[u]), val);
                hld.set_edge((u, parent[u]), SumNode::new(val));
            } else if t == 2 {
                // Query subtree
                let root = r.num(0..n);
                let st_nodes = get_subtree_nodes(&tree, root, parent[root]);
                if st_nodes.len() == 1 {
                    // Skip the leaf node, as its subtree does not contain any edges.
                    continue;
                }
                let sum: i64 = st_nodes
                    .iter()
                    .filter(|&&u| u != root) // skip root
                    .map(|&u| edge_weight.get(&(u, parent[u])).unwrap())
                    .sum();
                assert_eq!(hld.query_subtree(root).val, sum);
            }
        }
    }
}
