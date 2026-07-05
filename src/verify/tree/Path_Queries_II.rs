#![allow(dead_code)]
use std::cell::RefCell;
use std::io;
use std::io::Write;
use std::str;

#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::dbg;
#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::set_limit::timeout_secs;

/// Same API as Scanner but nearly twice as fast, using horribly unsafe dark arts
pub struct UnsafeScanner<R> {
    reader: R,
    buf_str: Vec<u8>,
    buf_iter: str::SplitAsciiWhitespace<'static>,
}

impl<R: io::BufRead> UnsafeScanner<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buf_str: vec![],
            buf_iter: "".split_ascii_whitespace(),
        }
    }

    /// This function should be marked unsafe, but noone has time for that in a
    /// programming contest. Use at your own risk!
    pub fn token<T: str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buf_iter.next() {
                return token.parse().ok().expect("Failed parse");
            }
            self.buf_str.clear();
            self.reader
                .read_until(b'\n', &mut self.buf_str)
                .expect("Failed read");
            self.buf_iter = unsafe {
                let slice = str::from_utf8_unchecked(&self.buf_str);
                std::mem::transmute::<
                    std::str::SplitAsciiWhitespace<'_>,
                    std::str::SplitAsciiWhitespace<'static>,
                >(slice.split_ascii_whitespace())
            };
        }
    }
}

fn is_local() -> bool {
    // "--local"/"local" forces file input, "--stdin"/"--no-local" forces stdin.
    // Without flags, the default is set via the Cargo feature "local".
    let mut arg_mode: Option<bool> = None;
    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "--local" | "local" => arg_mode = Some(true),
            "--stdin" | "--no-local" => arg_mode = Some(false),
            _ => {}
        }
    }
    arg_mode.unwrap_or(cfg!(feature = "local"))
}

fn scanner() -> UnsafeScanner<Box<dyn io::BufRead>> {
    let reader: Box<dyn io::BufRead> = if is_local() {
        let input_path = format!("{}/inp.txt", std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let file = std::fs::File::open(input_path).expect("Input file not found");
        Box::new(io::BufReader::new(file))
    } else {
        Box::new(io::BufReader::new(io::stdin()))
    };

    UnsafeScanner::new(reader)
}

fn writer() -> io::BufWriter<Box<dyn Write>> {
    let writer: Box<dyn Write> = if is_local() {
        let output_path = format!("{}/out.txt", std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let file = std::fs::File::create(output_path).expect("Failed to create output file");
        Box::new(file)
    } else {
        Box::new(io::stdout())
    };

    io::BufWriter::new(writer)
}

use std::marker::PhantomData;

macro_rules! recursive_function {
    ($name: ident, $trait: ident, ($($type: ident $arg: ident,)*)) => {
        pub trait $trait<$($type, )*Output> {
            fn call(&mut self, $($arg: $type,)*) -> Output;
        }

        pub struct $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            f: std::cell::UnsafeCell<F>,
            $($arg: PhantomData<$type>,
            )*
            phantom_output: PhantomData<Output>,
        }

        impl<F, $($type, )*Output> $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            pub fn new(f: F) -> Self {
                Self {
                    f: std::cell::UnsafeCell::new(f),
                    $($arg: Default::default(),
                    )*
                    phantom_output: Default::default(),
                }
            }
        }

        impl<F, $($type, )*Output> $trait<$($type, )*Output> for $name<F, $($type, )*Output>
        where
            F: FnMut(&mut dyn $trait<$($type, )*Output>, $($type, )*) -> Output,
        {
            fn call(&mut self, $($arg: $type,)*) -> Output {
                unsafe { (*self.f.get())(self, $($arg, )*) }
            }
        }
    }
}

recursive_function!(RecursiveFunction0, Callable0, ());
recursive_function!(RecursiveFunction, Callable, (Arg arg,));
recursive_function!(RecursiveFunction2, Callable2, (Arg1 arg1, Arg2 arg2,));
recursive_function!(RecursiveFunction3, Callable3, (Arg1 arg1, Arg2 arg2, Arg3 arg3,));
recursive_function!(RecursiveFunction4, Callable4, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4,));
recursive_function!(RecursiveFunction5, Callable5, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5,));
recursive_function!(RecursiveFunction6, Callable6, (Arg1 arg1, Arg2 arg2, Arg3 arg3, Arg4 arg4, Arg5 arg5, Arg6 arg6,));
// ANCHOR_END: main

#[derive(Default, Debug)]
pub struct RMQ<T: Clone> {
    values: Vec<T>,
    range_low: Vec<Vec<usize>>,
    max_mode: bool,
}

impl<T: Clone + Ord> RMQ<T> {
    pub fn new(values: &[T], max_mode: bool) -> Self {
        let mut rmq = Self {
            values: values.to_vec(),
            range_low: Vec::new(),
            max_mode,
        };
        rmq.build();
        rmq
    }

    pub fn build(&mut self) {
        let n = self.values.len();
        let levels = self.floor_log2(n) + 1;
        self.range_low.resize(levels, Vec::new());
        for k in 0..levels {
            self.range_low[k].resize(n - (1 << k) + 1, 0);
        }
        for i in 0..n {
            self.range_low[0][i] = i;
        }
        for k in 1..levels {
            for i in 0..=n - (1 << k) {
                self.range_low[k][i] = self.better_index(
                    self.range_low[k - 1][i],
                    self.range_low[k - 1][i + (1 << (k - 1))],
                );
            }
        }
    }

    fn floor_log2(&self, x: usize) -> usize {
        assert!(x > 0);
        (usize::BITS - 1 - x.leading_zeros()) as usize
        // Some(x.ilog2() as usize)
    }

    fn better_index(&self, a: usize, b: usize) -> usize {
        // use <= for the case when values[a] == values[b] then return a
        if (self.max_mode && self.values[b] < self.values[a])
            || (!self.max_mode && self.values[a] < self.values[b])
        {
            a
        } else {
            b
        }
    }

    pub fn query_index(&self, l: usize, mut r: usize) -> usize {
        let n = self.values.len();
        assert!(l <= r && r <= n);
        r += 1;
        if l == r {
            return l;
        }
        let level = self.floor_log2(r - l);
        self.better_index(
            self.range_low[level][l],
            self.range_low[level][r - (1 << level)],
        )
    }
    pub fn query_value(&self, l: usize, r: usize) -> T {
        self.values[self.query_index(l, r)].clone()
    }
}
// ANCHOR_END: main

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
        let mut first_in: Vec<usize> = vec![0; n];
        let mut tour = vec![];
        let mut depth = vec![];
        let mut dfs = RecursiveFunction2::new(|dfs, node: usize, parent: usize| {
            dep += 1;
            first_in[node] = tour.len();
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
pub trait Node: Default + Clone + Copy {
    fn new(val: i64) -> Self;
    fn combine(&self, other: &Self) -> Self;
    fn right_to_left(&self) -> Self {
        *self
    }
}

pub struct SegTree<T> {
    n: usize,
    dat: Vec<T>,
}

impl<T> SegTree<T>
where
    T: Node,
{
    pub fn from(v: &[T]) -> Self {
        let n = v.len().next_power_of_two();
        let mut dat: Vec<T> = (0..2 * n - 1).map(|_| T::default()).collect();
        for i in 0..v.len() {
            dat[n + i - 1] = v[i];
        }
        for i in (0..n - 1).rev() {
            dat[i] = dat[i * 2 + 1].combine(&dat[i * 2 + 2]);
        }
        Self { n, dat }
    }

    pub fn set(&mut self, mut index: usize, x: &T) {
        index += self.n - 1;
        self.dat[index] = *x;

        while index > 0 {
            index = (index - 1) / 2;
            self.dat[index] = self.dat[index * 2 + 1].combine(&self.dat[index * 2 + 2]);
        }
    }

    pub fn query(&self, mut l: usize, mut r: usize) -> T {
        assert!(l <= r);
        let mut lnode = T::default();
        let mut rnode = T::default();
        l += self.n - 1;
        r += self.n;
        while l < r {
            if (l & 1) == 0 {
                lnode = lnode.combine(&self.dat[l]);
            }
            if (r & 1) == 0 {
                rnode = self.dat[r - 1].combine(&rnode);
            }
            l /= 2;
            r = (r - 1) / 2;
        }
        lnode.combine(&rnode)
    }

    pub fn at(&self, index: usize) -> T {
        assert!(index < self.n);
        self.query(index, index)
    }

    /// Returns the smallest index `r >= start` such that `cmp(&query(start, r))` is `true`.
    ///
    /// The predicate must be monotonic with respect to `r`: once it becomes `true`,
    /// it must remain `true` for all larger indices, otherwise the binary search is invalid.
    ///
    /// Returns `None` if no such index exists.
    ///
    /// # Example
    ///
    /// ```rust
    /// use rs_space::range_query::segtree::{Node, SegTree, MaxNode};
    /// let nodes: Vec<MaxNode> = (0..100).map(|v| MaxNode::new(v)).collect();
    /// let mut seg = SegTree::from(&nodes);
    /// seg.find_right(0, |range_node| range_node.val >= 10);
    /// ```
    pub fn find_right<F>(&self, start: usize, cmp: F) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        assert!(start < self.n);
        let mut l = start;
        let mut r = self.n - 1;
        while l != r {
            let mid = (l + r) / 2;
            let acc = self.query(start, mid);
            if cmp(&acc) {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        let acc = self.query(start, l);
        if cmp(&acc) { Some(l) } else { None }
    }

    /// Returns the largest index `l <= end` such that `cmp(&query(l, end))` is `true`.
    ///
    /// The predicate must be monotonic with respect to `l`: once it is `true` at some index,
    /// it must remain `true` for all larger indices up to `end`, otherwise the binary search is invalid.
    ///
    /// Returns `None` if no such index exists.
    pub fn find_left<F>(&self, end: usize, cmp: F) -> Option<usize>
    where
        F: Fn(&T) -> bool,
    {
        assert!(end < self.n);
        let mut l = 0;
        let mut r = end;
        while l != r {
            let mid = (l + r).div_ceil(2);
            let acc = self.query(mid, end);
            if cmp(&acc) {
                l = mid;
            } else {
                r = mid - 1;
            }
        }
        let acc = self.query(l, end);
        if cmp(&acc) { Some(l) } else { None }
    }
}
// ANCHOR_END: segtree

// ANCHOR: SumNode
#[derive(Default, Clone, Copy)]
pub struct SumNode {
    pub val: i64,
    pub has_value: bool,
}

impl Node for SumNode {
    fn new(val: i64) -> Self {
        Self {
            val,
            has_value: true,
        }
    }
    fn combine(&self, other: &Self) -> Self {
        if !self.has_value {
            return *other;
        }
        if !other.has_value {
            return *self;
        }
        Self::new(self.val + other.val)
    }
}
// ANCHOR_END: SumNode

// ANCHOR: MinNode
#[derive(Default, Clone, Copy)]
pub struct MinNode {
    pub val: i64,
    pub has_value: bool,
}

impl Node for MinNode {
    fn new(val: i64) -> Self {
        Self {
            val,
            has_value: true,
        }
    }
    fn combine(&self, other: &Self) -> Self {
        if !self.has_value {
            return *other;
        }
        if !other.has_value {
            return *self;
        }
        Self::new(self.val.min(other.val))
    }
}
// ANCHOR_END: MinNode

// ANCHOR: MaxNode
#[derive(Default, Clone, Copy)]
pub struct MaxNode {
    pub val: i64,
    pub has_value: bool,
}

impl Node for MaxNode {
    fn new(val: i64) -> Self {
        Self {
            val,
            has_value: true,
        }
    }
    fn combine(&self, other: &Self) -> Self {
        if !self.has_value {
            return *other;
        }
        if !other.has_value {
            return *self;
        }
        Self::new(self.val.max(other.val))
    }
}
// ANCHOR_END: MaxNode

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
        let parent_vec = parent.borrow().clone();
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

    fn include_root_path(self, mut u: usize, p: usize) -> Vec<(usize, usize)> {
        let mut path: Vec<(usize, usize)> = Vec::new();
        while self.head[p] != self.head[u] {
            path.push((u, self.head[u]));
            u = self.parent[self.head[u]];
        }
        path.push((u, p));
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

fn main() {
    #[cfg(feature = "local")]
    timeout_secs(5);
    let mut scan = scanner();
    let mut out = writer();
    let n: usize = scan.token();
    let q: usize = scan.token();
    let weight: Vec<i64> = (0..n).map(|_| scan.token()).collect();
    let mut graph: Vec<Vec<usize>> = vec![Vec::new(); n];
    for _ in 0..n - 1 {
        let u: usize = scan.token();
        let v: usize = scan.token();
        graph[u - 1].push(v - 1);
        graph[v - 1].push(u - 1);
    }
    let mut hld: HLD<MaxNode> = HLD::new(&graph, &weight);
    for _ in 0..q {
        let t: i32 = scan.token();
        let u: usize = scan.token();
        let v: i64 = scan.token();
        if t == 1 {
            // Update
            hld.set_node(u - 1, Node::new(v));
        } else {
            // Query
            let res = hld.queyry_path(u - 1, (v - 1).try_into().unwrap());
            write!(out, "{:?} ", res.val).unwrap();
        }
    }
}
