use rand::Rng;
use rand::distr::uniform::SampleRange;
use rand::distr::uniform::SampleUniform;
use rand::rngs::ThreadRng;
use std::collections::HashSet;
use std::collections::VecDeque;

/// Random module
///
/// let mut r = Random::new();
pub struct Random {
    rng: ThreadRng,
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

impl Random {
    pub fn new() -> Self {
        Self { rng: rand::rng() }
    }

    /// r.num(min, max)
    /// r.num(min..=max)
    pub fn num<T, R>(&mut self, range: R) -> T
    where
        T: SampleUniform,
        R: SampleRange<T>,
    {
        self.rng.random_range(range)
    }

    /// r.bool()
    pub fn bool(&mut self) -> bool {
        self.num(0..=1) == 1
    }

    pub fn choose<T: Clone>(&mut self, items: &[T]) -> T {
        let index = self.num(0..items.len());
        items[index].clone()
    }

    pub fn vector<T, R>(&mut self, n: usize, range: R) -> Vec<T>
    where
        T: rand::distr::uniform::SampleUniform + Copy,
        R: rand::distr::uniform::SampleRange<T> + Clone,
    {
        (0..n).map(|_| self.num(range.clone())).collect()
    }

    pub fn string(&mut self, len: usize) -> String {
        let charset = b"abcdefghijklmnopqrstuvwxyz";
        (0..len)
            .map(|_| charset[self.num(0..charset.len())] as char)
            .collect()
    }

    // Default root = 0
    pub fn tree(&mut self, size: usize) -> Vec<Vec<usize>> {
        let root = 0;
        let mut g: Vec<Vec<usize>> = vec![vec![]; size];
        let mut leaves: VecDeque<usize> = VecDeque::new();
        leaves.push_back(root);
        for i in 1..size {
            loop {
                let parent = leaves.pop_back().unwrap();
                let attach = if leaves.is_empty() { true } else { self.bool() };
                if !attach {
                    continue;
                }
                g[parent].push(i);
                leaves.push_front(parent); // đẩy ngược parent vào lại, nó vẫn còn đang được sử dụng
                leaves.push_back(i);
                break;
            }
        }
        g
    }

    pub fn directed_graph(&mut self, size: usize) -> Vec<Vec<usize>> {
        let mut edges: HashSet<(usize, usize)> = HashSet::new();
        let mut tree = self.tree(size);
        for (i, nodes) in tree.iter().enumerate() {
            for &j in nodes {
                edges.insert((i.min(j), i.max(j)));
            }
        }
        let added_edge_num = size * size;
        for _ in 0..added_edge_num {
            let u = self.num(0..size);
            let v = self.num(0..size);
            if u != v && !edges.contains(&(u, v)) {
                edges.insert((u, v));
                tree[u].push(v);
            }
        }

        tree
    }
}
