#![allow(dead_code)]
// ANCHOR: main
use crate::datastructure::dsu::DSU;

#[derive(Clone, Copy)]
pub struct Edge {
    u: usize,
    v: usize,
    w: i64,
}

pub fn kruskal(graph: &[Vec<(usize, i64)>]) -> Vec<Edge> {
    let n = graph.len();
    let mut edges: Vec<Edge> = Vec::new();
    for (u, neighbors) in graph.iter().enumerate() {
        for &(v, w) in neighbors {
            edges.push(Edge { u, v, w });
        }
    }
    edges.sort_by_key(|edge| edge.w);
    let mut dsu = DSU::new(n);
    let mut mst: Vec<Edge> = Vec::new();
    for edge in &edges {
        if !dsu.is_same(edge.u, edge.v) {
            mst.push(*edge);
            dsu.merge(edge.u, edge.v);
        }
    }
    if mst.len() + 1 != n {
        return vec![];
    }
    mst
}
// ANCHOR_END: main

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn test_mst() {
        let mut r = Random::new();
        let n = r.num(1000..5000);
        let tree = r.tree(n);
        let mut graph: Vec<Vec<(usize, i64)>> = vec![Vec::new(); n];
        for u in 0..n {
            for &v in &tree[u] {
                graph[u].push((v, 1));
                graph[v].push((u, 1));
            }
        }
        for _ in 0..2 * n {
            let u = r.num(0..n);
            let v = r.num(0..n);
            let w: i64 = r.num(2..100);
            if u != v {
                graph[u].push((v, w));
                graph[v].push((u, w));
            }
        }
        let mst = kruskal(&graph);
        assert_eq!(mst.len(), n - 1);
        for &edge in &mst {
            assert_eq!(edge.w, 1);
        }
    }

    #[test]
    fn test_no_mst() {
        let graph = vec![vec![], vec![(2, 1)], vec![(1, 1)]];
        let mst = kruskal(&graph);
        assert!(mst.is_empty());
    }
}
