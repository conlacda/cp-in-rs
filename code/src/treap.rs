/// TODO: this treap is based on traditional treap in C++ 
/// that C++ version is not being used in reality and maybe not correct
/// Or maybe c++ version is good, but rust version is not
/// Treap can be used for multiset which Rust does not have, and also replace policy-based structure
/// which supports index in multiset + all other multiset functions
#![allow(dead_code)]

use rand::Rng;

pub const NULL_ID: usize = 0;
#[derive(Clone)]
pub struct Node {
    id: usize,
    lc_id: usize, // left child id
    rc_id: usize, // right child id
    pr_id: usize, // parent id
    pub val: i64,
    prior: u64,
    size: usize,
}

impl Node {
    fn new(val: i64) -> Self {
        let mut rng = rand::rng();
        Self {
            id: NULL_ID,
            lc_id: NULL_ID,
            rc_id: NULL_ID,
            pr_id: NULL_ID,
            val,
            prior: rng.random::<u64>(),
            size: 1,
        }
    }
    fn get_size(&self) -> usize {
        if self.id == NULL_ID { 0 } else { self.size }
    }
}

pub struct Treap {
    free_node_ids: Vec<usize>,
    pub nodes: Vec<Node>,
}

impl Treap {
    pub fn new(size: usize) -> Self {
        let nodes: Vec<Node> = (0..size).map(|_| Node::new(0)).collect();
        Self {
            free_node_ids: (1..size).rev().collect(),
            nodes,
        }
    }

    fn free_id(&mut self) -> usize {
        self.free_node_ids
            .pop()
            .expect("Running out of nodes => increase max size may help")
    }

    /// Create a Node and return node id
    pub fn create_node(&mut self, val: i64) -> usize {
        let id = self.free_id();
        self.nodes[id] = Node::new(val);
        self.nodes[id].id = id;
        id
    }

    /// Release node and put it back to free nodes pool
    fn recycle(&mut self, node_id: usize) {
        if node_id == NULL_ID {
            return;
        }
        self.free_node_ids.push(node_id);
        let (lc, rc) = {
            let node = &self.nodes[node_id];
            (node.lc_id, node.rc_id)
        };
        self.recycle(lc);
        self.recycle(rc);
    }

    fn root_id(&self, id: usize) -> usize {
        let mut root: usize = id;
        while self.nodes[root].pr_id != NULL_ID {
            root = self.nodes[root].pr_id;
        }
        root
    }

    fn recalc(&mut self, node_id: usize) {
        assert!(node_id != NULL_ID);
        // nodes[node_id].size = 1 + size(nodes[node_id].lid) + size(nodes[node_id].rid);
        self.nodes[node_id].size = 1
            + self.nodes[self.nodes[node_id].lc_id].get_size()
            + self.nodes[self.nodes[node_id].rc_id].get_size();
    }

    /// Split the treap into 2 parts then return their ids
    ///
    /// The left part contains nodes with value less than val
    /// The right part contains nodes with value greater than or equal to val
    fn split_by_val(&mut self, id: usize, val: i64) -> (usize, usize) {
        if id == NULL_ID {
            return (NULL_ID, NULL_ID);
        }
        if self.nodes[id].val >= val {
            let p = self.split_by_val(self.nodes[id].lc_id, val);
            self.nodes[id].lc_id = p.1;
            self.nodes[p.1].pr_id = self.nodes[id].id;
            self.nodes[p.0].pr_id = NULL_ID;
            self.recalc(self.nodes[id].id);
            return (p.0, self.nodes[id].id);
        }

        let p = self.split_by_val(self.nodes[id].rc_id, val);
        self.nodes[id].rc_id = p.0;
        self.nodes[p.0].pr_id = self.nodes[id].id;
        self.nodes[p.1].pr_id = NULL_ID;
        self.recalc(self.nodes[id].id);
        (self.nodes[id].id, p.1)
    }

    fn split_by_size(&mut self, id: usize, sz: usize) -> (usize, usize) {
        if id == NULL_ID {
            return (NULL_ID, NULL_ID);
        }
        if self.nodes[self.nodes[id].lc_id].get_size() >= sz {
            let p = self.split_by_size(self.nodes[id].lc_id, sz);
            self.nodes[id].lc_id = p.1;
            self.nodes[p.1].pr_id = self.nodes[id].id;
            self.nodes[p.0].pr_id = NULL_ID;
            self.recalc(id);
            return (p.0, id);
        }
        let p = self.split_by_size(
            self.nodes[id].rc_id,
            sz - self.nodes[self.nodes[id].lc_id].get_size() - 1,
        );
        self.nodes[id].rc_id = p.0;
        self.nodes[p.0].pr_id = id;
        self.nodes[p.1].pr_id = NULL_ID;
        self.recalc(id);
        (id, p.1)
    }

    pub fn merge(&mut self, l: usize, r: usize) -> usize {
        if l == NULL_ID {
            return r;
        }
        if r == NULL_ID {
            return l;
        }
        let (x, y) = if self.nodes[l].val < self.nodes[r].val {
            (l, r)
        } else {
            (r, l)
        };
        let merge_node: usize;
        if self.nodes[x].prior > self.nodes[y].prior {
            let id = self.merge(self.nodes[x].rc_id, y);
            self.nodes[x].rc_id = id;
            self.nodes[id].pr_id = self.nodes[x].id;
            merge_node = x;
        } else {
            let id = self.merge(x, self.nodes[y].lc_id);
            self.nodes[y].lc_id = id;
            self.nodes[id].pr_id = self.nodes[y].id;
            merge_node = y;
        }
        self.nodes[merge_node].pr_id = NULL_ID;
        self.recalc(merge_node);
        merge_node
    }

    /// Merge multiple nodes by order
    pub fn merge_many(&mut self, v: &[usize]) -> usize {
        assert!(!v.is_empty());
        let mut root = v[0];
        for &node in v.iter().skip(1) {
            root = self.merge(root, node);
        }
        root
    }

    // Add a new number to treap
    pub fn insert(&mut self, root: usize, val: i64) -> usize {
        let (l, r) = self.split_by_val(root, val);
        let node = self.create_node(val);
        let node_r = self.merge(node, r);
        self.merge(l, node_r)
    }

    /// Remove all nodes of val
    pub fn remove_all(&mut self, root: usize, val: i64) -> usize {
        let (l, r) = self.split_by_val(root, val);
        let (eq, r) = self.split_by_val(r, val + 1);
        self.recycle(eq);
        self.merge(l, r)
    }

    /// Remove a node of val
    pub fn remove(&mut self, root: usize, val: i64) -> usize {
        let (l, eq_r) = self.split_by_val(root, val);
        let (eq, r) = self.split_by_size(eq_r, 1);
        if self.nodes[eq].val == val {
            self.merge(l, r)
        } else {
            self.merge(l, eq_r)
        }
    }

    pub fn count_less(&self, root: usize, val: i64) -> usize {
        if root == NULL_ID {
            return 0;
        }
        if self.nodes[root].val < val {
            1 + self.nodes[self.nodes[root].lc_id].get_size()
                + self.count_less(self.nodes[root].rc_id, val)
        } else {
            self.count_less(self.nodes[root].lc_id, val)
        }
    }

    pub fn count_greater(&self, root: usize, val: i64) -> usize {
        if root == NULL_ID {
            return 0;
        }
        if self.nodes[root].val <= val {
            self.count_greater(self.nodes[root].rc_id, val)
        } else {
            1 + self.nodes[self.nodes[root].rc_id].get_size()
                + self.count_greater(self.nodes[root].lc_id, val)
        }
    }

    pub fn count(&self, root: usize, val: i64) -> usize {
        self.nodes[root].get_size() - self.count_less(root, val) - self.count_greater(root, val)
    }

    /// Get node by index
    pub fn at(&mut self, root: usize, mut index: usize) -> Option<usize> {
        if root == NULL_ID {
            return None;
        }
        if index >= self.nodes[root].get_size() {
            return None;
        }
        if self.nodes[self.nodes[root].lc_id].get_size() > index {
            return self.at(self.nodes[root].lc_id, index);
        }
        index -= self.nodes[self.nodes[root].lc_id].get_size();
        if index == 0 {
            return Some(root);
        }
        self.at(self.nodes[root].rc_id, index - 1)
    }

    // pub remove_index
    // count_greater
    // count (equal)
    // nearest_down
    // nearest_up
    // count between
    // get_range
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;
    use indicatif::{ProgressBar, ProgressIterator};
    use std::collections::BTreeMap;

    #[test]
    fn test_() {
        let pb = ProgressBar::new(100);
        (0..100).progress_with(pb).for_each(|_| {
            let mut treap = Treap::new(10000);
            let mut root = NULL_ID;
            let mut multiset = BTreeMap::new();
            let mut rand = Random::new();
            // Insert, remove, remove_all
            for _ in 0..10 {
                let op = rand.choose(&["insert", "remove", "remove_all"]);
                let val = rand.num(0..100);
                if op == "insert" {
                    root = treap.insert(root, val);
                    *multiset.entry(val).or_insert(0) += 1;
                } else if op == "remove" {
                    root = treap.remove(root, val);
                    if let Some(&count) = multiset.get(&val) {
                        if count <= 1 {
                            multiset.remove(&val);
                        } else {
                            *multiset.get_mut(&val).unwrap() = count - 1;
                        }
                    }
                } else if op == "remove_all" {
                    root = treap.remove_all(root, val);
                    multiset.remove(&val);
                }
                dbg!(op, val);
                dbg!(&multiset);
            }
            // count, count_less, count_greater
            for (&key, &value) in &multiset {
                dbg!(treap.count(root, key), value);
                assert!(treap.count(root, key) == value);
            }
        });
    }
}
