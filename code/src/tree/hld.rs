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
}

impl<T> HLD<T>
where
    T: Node,
{
    pub fn new(graph: &[Vec<usize>], weight: &[i64]) -> Self {
        let n = graph.len();
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
            for &c in &graph[p] {
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
            for &v in &graph[u] {
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

        let lca = LCA::new(graph, 0);
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
        }
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

    pub fn queyry_path(&self, u: usize, v: usize) -> T {
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
        left.right_to_left().combine(&root).combine(&right)
    }

    pub fn query_subtree(&self, root: usize) -> T {
        self.segtree
            .query(self.pre_order[root], self.post_order[root] - 1)
    }

    pub fn set_node(&mut self, u: usize, node: T) {
        self.segtree.set(self.pos[u], &node);
    }

    pub fn set_edge(&mut self, u: usize, v: usize, node: T) {
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
    // use super::*;
    // use crate::random::Random;

    fn test_query_path() {}
    fn test_query_subtree() {}
    fn random_set_node() {}
    fn random_set_edge() {}
    #[test]
    fn test_weight_on_vertices() {
        // new
        // query
        // random set
        // query
    }
    #[test]
    fn test_weight_on_edge() {
        // new
        // query
        // random set
        // query
    }

    #[test]
    fn test_exclude_root_path() {}
}
