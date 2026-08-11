use rand::Rng;

const NULL_ID: usize = 0;
#[derive(Clone)]
struct Node {
    id: usize,
    left_child_id: usize,
    right_child_id: usize,
    pub val: i64,
    prior: u64,
    size: usize,
}

impl Node {
    fn new(val: i64) -> Self {
        Self {
            id: NULL_ID,
            left_child_id: NULL_ID,
            right_child_id: NULL_ID,
            val,
            prior: 0,
            size: 1,
        }
    }
    fn get_size(&self) -> usize {
        if self.id == NULL_ID { 0 } else { self.size }
    }
}

struct Treap {
    free_node_ids: Vec<usize>,
    nodes: Vec<Node>,
}

impl Treap {
    pub fn new(size: usize) -> Self {
        let nodes: Vec<Node> = (0..=size).map(|_| Node::new(0)).collect();
        Self {
            free_node_ids: (1..=size).rev().collect(),
            nodes,
        }
    }

    fn free_id(&mut self) -> usize {
        self.free_node_ids
            .pop()
            .expect("Running out of nodes => increase max size may help")
    }

    /// Create a Node and return node id
    pub fn new_node(&mut self, val: i64) -> usize {
        let mut rng = rand::rng();
        let id = self.free_id();
        self.nodes[id] = Node::new(val);
        self.nodes[id].prior = rng.random::<u64>();
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
            (node.left_child_id, node.right_child_id)
        };
        self.recycle(lc);
        self.recycle(rc);
    }

    fn recalc(&mut self, node_id: usize) {
        assert!(node_id != NULL_ID);
        self.nodes[node_id].size = 1
            + self.nodes[self.nodes[node_id].left_child_id].get_size()
            + self.nodes[self.nodes[node_id].right_child_id].get_size();
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
            let p = self.split_by_val(self.nodes[id].left_child_id, val);
            self.nodes[id].left_child_id = p.1;
            self.recalc(self.nodes[id].id);
            return (p.0, self.nodes[id].id);
        }

        let p = self.split_by_val(self.nodes[id].right_child_id, val);
        self.nodes[id].right_child_id = p.0;
        self.recalc(self.nodes[id].id);
        (self.nodes[id].id, p.1)
    }

    fn split_by_size(&mut self, id: usize, sz: usize) -> (usize, usize) {
        if id == NULL_ID {
            return (NULL_ID, NULL_ID);
        }
        if self.nodes[self.nodes[id].left_child_id].get_size() >= sz {
            let p = self.split_by_size(self.nodes[id].left_child_id, sz);
            self.nodes[id].left_child_id = p.1;
            self.recalc(id);
            return (p.0, id);
        }
        let p = self.split_by_size(
            self.nodes[id].right_child_id,
            sz - self.nodes[self.nodes[id].left_child_id].get_size() - 1,
        );
        self.nodes[id].right_child_id = p.0;
        self.recalc(id);
        (id, p.1)
    }

    // Require: every value in l <= every value in r
    fn merge(&mut self, l: usize, r: usize) -> usize {
        if l == NULL_ID {
            return r;
        }
        if r == NULL_ID {
            return l;
        }
        let left_prior = self.nodes[l].prior;
        let right_prior = self.nodes[r].prior;

        let merged_id: usize;
        if left_prior > right_prior {
            let right_child_id = self.nodes[l].right_child_id;
            let id = self.merge(right_child_id, r);
            self.nodes[l].right_child_id = id;
            let left_node_id = self.nodes[l].id;
            merged_id = left_node_id;
        } else {
            let left_child_id = self.nodes[r].left_child_id;
            let id = self.merge(l, left_child_id);
            self.nodes[r].left_child_id = id;
            let right_node_id = self.nodes[r].id;
            merged_id = right_node_id;
        }
        self.recalc(merged_id);
        merged_id
    }

    // Add a new number to treap
    pub fn insert(&mut self, root: usize, val: i64) -> usize {
        let (l, r) = self.split_by_val(root, val);
        let node = self.new_node(val);
        let node_r = self.merge(node, r);
        self.merge(l, node_r)
    }

    /// Remove a node of val
    pub fn remove(&mut self, root: usize, val: i64) -> usize {
        let (l, eq_r) = self.split_by_val(root, val);
        let (eq, r) = self.split_by_size(eq_r, 1);
        if self.nodes[eq].val == val {
            self.recycle(eq);
            self.merge(l, r)
        } else {
            let eq_r = self.merge(eq, r);
            self.merge(l, eq_r)
        }
    }

    /// Remove all nodes of val
    pub fn remove_all(&mut self, root: usize, val: i64) -> usize {
        let (l, r) = self.split_by_val(root, val);
        if val == i64::MAX {
            self.recycle(r);
            return l;
        }
        let (eq, r) = self.split_by_val(r, val + 1);
        self.recycle(eq);
        self.merge(l, r)
    }

    pub fn count_less(&self, root: usize, val: i64) -> usize {
        if root == NULL_ID {
            return 0;
        }
        if self.nodes[root].val < val {
            1 + self.nodes[self.nodes[root].left_child_id].get_size()
                + self.count_less(self.nodes[root].right_child_id, val)
        } else {
            self.count_less(self.nodes[root].left_child_id, val)
        }
    }

    pub fn count_greater(&self, root: usize, val: i64) -> usize {
        if root == NULL_ID {
            return 0;
        }
        if self.nodes[root].val <= val {
            self.count_greater(self.nodes[root].right_child_id, val)
        } else {
            1 + self.nodes[self.nodes[root].right_child_id].get_size()
                + self.count_greater(self.nodes[root].left_child_id, val)
        }
    }

    pub fn count(&self, root: usize, val: i64) -> usize {
        self.nodes[root].get_size() - self.count_less(root, val) - self.count_greater(root, val)
    }

    /// Get node by index
    pub fn at(&self, root: usize, mut index: usize) -> Option<usize> {
        if root == NULL_ID {
            return None;
        }
        if index >= self.nodes[root].get_size() {
            return None;
        }
        if self.nodes[self.nodes[root].left_child_id].get_size() > index {
            return self.at(self.nodes[root].left_child_id, index);
        }
        index -= self.nodes[self.nodes[root].left_child_id].get_size();
        if index == 0 {
            return Some(root);
        }
        self.at(self.nodes[root].right_child_id, index - 1)
    }

    // pub remove_index
    // count (equal)
    // nearest_down
    // nearest_up
    // count between
    // get_range
    // split3PartsBySize
    // remove_range
}

pub struct Multiset {
    treap: Treap,
    root: usize,
}

impl Multiset {
    pub fn new(capacity: usize) -> Self {
        Self {
            treap: Treap::new(capacity),
            root: NULL_ID,
        }
    }

    pub fn insert(&mut self, val: i64) {
        self.root = self.treap.insert(self.root, val);
    }

    /// Removes one occurrence of `val` and reports whether one existed.
    pub fn remove(&mut self, val: i64) -> bool {
        if self.count(val) == 0 {
            return false;
        }
        self.root = self.treap.remove(self.root, val);
        true
    }

    /// Removes every occurrence of `val` and returns the number removed.
    pub fn remove_all(&mut self, val: i64) -> usize {
        let removed = self.count(val);
        if removed != 0 {
            self.root = self.treap.remove_all(self.root, val);
        }
        removed
    }

    pub fn len(&self) -> usize {
        self.treap.nodes[self.root].get_size()
    }

    pub fn is_empty(&self) -> bool {
        self.root == NULL_ID
    }

    pub fn count(&self, val: i64) -> usize {
        self.treap.count(self.root, val)
    }

    pub fn count_less(&self, val: i64) -> usize {
        self.treap.count_less(self.root, val)
    }

    pub fn count_greater(&self, val: i64) -> usize {
        self.treap.count_greater(self.root, val)
    }

    pub fn count_between(&self, l: i64, r: i64) -> usize {
        self.len() - self.count_less(l) - self.count_greater(r)
    }

    /// Returns the lower-bound index of `val`.
    ///
    /// If `val` is absent, this is the index where it would be inserted while
    /// preserving sorted order. The returned index may equal [`Self::len`].
    /// When duplicates exist, this returns the index of the first occurrence.
    pub fn index_of(&self, val: i64) -> usize {
        self.count_less(val)
    }

    /// Returns the element at the zero-based index in sorted order.
    pub fn at(&self, index: usize) -> Option<i64> {
        self.treap
            .at(self.root, index)
            .map(|id| self.treap.nodes[id].val)
    }

    pub fn nearest_up(&self, val: i64) -> Option<i64> {
        let index = self.index_of(val);
        if index == self.len() {
            None
        } else {
            self.at(index)
        }
    }

    /// Returns the greatest element less than or equal to `val`.
    pub fn nearest_down(&self, val: i64) -> Option<i64> {
        let upper_bound = self.len() - self.count_greater(val);
        if upper_bound == 0 {
            None
        } else {
            self.at(upper_bound - 1)
        }
    }

    /// Removes all values while retaining the allocated capacity for reuse.
    pub fn clear(&mut self) {
        self.treap.recycle(self.root);
        self.root = NULL_ID;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;
    use std::collections::BTreeMap;

    #[test]
    fn test_treap() {
        let mut treap = Treap::new(10000);
        let mut root = NULL_ID;
        let mut multiset = BTreeMap::new();
        let mut rand = Random::new();
        // Insert, remove, remove_all
        for _ in 0..1000 {
            let op = rand.choose(&["insert", "remove", "remove_all"]);
            let val = rand.num(0..100);
            match op {
                "insert" => {
                    root = treap.insert(root, val);
                    *multiset.entry(val).or_insert(0) += 1;
                }
                "remove" => {
                    root = treap.remove(root, val);
                    if let Some(&count) = multiset.get(&val) {
                        if count <= 1 {
                            multiset.remove(&val);
                        } else {
                            *multiset.get_mut(&val).unwrap() = count - 1;
                        }
                    }
                }
                "remove_all" => {
                    root = treap.remove_all(root, val);
                    multiset.remove(&val);
                }
                _ => unreachable!(),
            }
        }
        // count, count_less, count_greater
        for (&key, &value) in &multiset {
            assert!(treap.count(root, key) == value);
        }
    }

    #[test]
    fn test_multiset() {
        let mut rand = Random::new();
        const MIN: i64 = -1_000_000;
        const MAX: i64 = 1_000_000;
        let mut vec = rand.vector(200, MIN..MAX);
        let mut mts = Multiset::new(1_000_000);
        for &val in &vec {
            mts.insert(val);
        }
        for _ in 0..100 {
            let op = rand.num(0..10);
            match op {
                0 => {
                    // insert
                    let val = rand.num(MIN..MAX);
                    mts.insert(val);
                    vec.push(val);
                }
                1 => {
                    // remove()
                    let val_to_remove = rand.choose(&vec);
                    mts.remove(val_to_remove);
                    if let Some(index) = vec.iter().position(|&x| x == val_to_remove) {
                        vec.remove(index);
                    }
                }
                2 => {
                    // remove_all()
                    let val_to_remove = rand.choose(&vec);
                    mts.remove_all(val_to_remove);
                    vec.retain(|&x| x != val_to_remove);
                }
                3 => {
                    assert_eq!(mts.len(), vec.len());
                }
                4 => {
                    assert_eq!(mts.is_empty(), vec.is_empty());
                    assert!(Multiset::new(0).is_empty());
                }
                5 => {
                    // count, count_less, count_greater
                    let val = rand.choose(&vec);
                    assert_eq!(
                        mts.count(val),
                        vec.iter().filter(|&&value| value == val).count()
                    );
                    assert_eq!(
                        mts.count_less(val),
                        vec.iter().filter(|&&value| value < val).count()
                    );
                    assert_eq!(
                        mts.count_greater(val),
                        vec.iter().filter(|&&value| value > val).count()
                    );
                }
                6 => {
                    // index_of
                    let val = rand.choose(&vec);
                    vec.sort();
                    let index = mts.index_of(val);
                    assert!(vec[index] == val);
                    if index > 0 {
                        assert!(vec[index - 1] < val);
                    }
                }
                7 => {
                    // at
                    vec.sort();
                    let index = rand.num(0..vec.len());
                    assert_eq!(mts.at(index).unwrap(), vec[index]);
                }
                8 => {
                    // nearest_up, nearest_down
                    let mut v = vec.clone();
                    v.sort_unstable();
                    v.dedup();
                    for (i, &val) in v.iter().enumerate() {
                        assert_eq!(mts.nearest_up(val).unwrap(), val);
                        assert_eq!(mts.nearest_down(val).unwrap(), val);
                        if i + 1 == v.len() {
                            assert_eq!(mts.nearest_up(val + 1), None);
                        } else {
                            assert_eq!(mts.nearest_up(val + 1), Some(v[i + 1]));
                        }
                        if i == 0 {
                            assert_eq!(mts.nearest_down(val - 1), None);
                        } else {
                            assert_eq!(mts.nearest_down(val - 1), Some(v[i - 1]));
                        }
                    }
                }
                9 => {
                    // clear
                    let mut m = Multiset::new(1);
                    m.insert(1);
                    m.clear();
                    assert!(m.is_empty());
                    assert_eq!(m.at(0), None);
                    // Clearing must recycle every node so the full capacity can be reused.
                    m.insert(1);
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_zero_capacity() {
        let mts = Multiset::new(0);

        assert!(mts.is_empty());
        assert_eq!(mts.len(), 0);
        assert_eq!(mts.index_of(0), 0);
        assert_eq!(mts.at(0), None);
        assert_eq!(mts.nearest_down(0), None);
    }

    #[test]
    fn test_extreme_and_absent_values() {
        let mut mts = Multiset::new(2);
        mts.insert(i64::MIN);
        mts.insert(i64::MAX);

        assert_eq!(mts.at(0), Some(i64::MIN));
        assert_eq!(mts.at(1), Some(i64::MAX));
        assert_eq!(mts.nearest_down(i64::MIN), Some(i64::MIN));
        assert_eq!(mts.nearest_down(i64::MAX), Some(i64::MAX));
        assert_eq!(mts.index_of(0), 1);
        assert_eq!(mts.count(0), 0);
        assert!(!mts.remove(0));
        assert_eq!(mts.remove_all(i64::MAX), 1);
    }

    #[test]
    fn test_capacity_reuse() {
        let mut mts = Multiset::new(1);
        mts.insert(10);
        assert!(mts.remove(10));

        // This would panic if the removed node were not recycled.
        mts.insert(20);
        assert_eq!(mts.at(0), Some(20));
    }
}
