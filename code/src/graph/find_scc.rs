use crate::graph::toposort::toposort;
use crate::recursive::Callable;
use crate::recursive::RecursiveFunction;
use std::cell::RefCell;

// ANCHOR: main
fn reversed_graph(g: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = g.len();
    let mut rg = vec![vec![]; n];
    for (u, edges) in g.iter().enumerate() {
        for &v in edges {
            rg[v].push(u);
        }
    }
    rg
}

pub fn find_scc(g: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let n = g.len();
    let rg = reversed_graph(g);
    let post_orders_on_rg = toposort(&rg);
    let mut result: Vec<Vec<usize>> = vec![];
    let vis = RefCell::new(vec![false; n]);
    let scc = RefCell::new(Vec::<usize>::new());
    // RefCell + RecursiveFunction can be removed if we pass them as references to the closure:
    // dfs(u, &g, &vis, &scc)
    //
    // When passing by reference, Rust checks that the previous borrow has been released
    // before the next one. After dfs() returns, it can borrow `vis` and `scc` again,
    // which satisfies the borrow rules.
    //
    // If we don’t pass references, Rust assumes the entire dfs() function scope
    // holds the borrow. Then calling dfs() again attempts to borrow them one more time,
    // which violates the borrow rules.
    let mut dfs = RecursiveFunction::new(|dfs, u: usize| {
        if vis.borrow()[u] {
            return;
        }
        scc.borrow_mut().push(u);
        vis.borrow_mut()[u] = true;
        for &v in &g[u] {
            dfs.call(v);
        }
    });
    for &i in post_orders_on_rg.iter() {
        if !vis.borrow()[i] {
            dfs.call(i);
        }
        if !scc.borrow().is_empty() {
            result.push(scc.borrow().clone());
            scc.borrow_mut().clear();
        }
    }
    result
}
// ANCHOR_END: main

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn test_toposort() {
        let mut r = Random::new();
        let size = r.num(5..10000);
        let dag = r.tree(size);
        let tps = toposort(&dag);
        // Duyệt qua từng node từ cuối trở lại, duyệt thì đẩy vào dead, còn lại là alive
        // toàn bộ node con của nó phải là dead
        let mut is_dead = vec![false; size];
        for node in (0..tps.len()).rev() {
            for child in &dag[node] {
                assert!(&is_dead[*child]);
            }
            is_dead[node] = true;
        }
    }

    #[test]
    fn test_scc() {
        let mut r = Random::new();
        let size = r.num(5..300);
        let directed_graph = r.directed_graph(size);
        let sccs = find_scc(&directed_graph);

        fn reachable(g: &[Vec<usize>], src: usize) -> Vec<bool> {
            let n = g.len();
            let mut vis = vec![false; n];
            let mut stack = vec![src];

            while let Some(u) = stack.pop() {
                if vis[u] {
                    continue;
                }
                vis[u] = true;
                for &v in &g[u] {
                    if !vis[v] {
                        stack.push(v);
                    }
                }
            }
            vis
        }

        // Every node must appear exactly once
        let mut belong = vec![usize::MAX; size];
        for (cid, scc) in sccs.iter().enumerate() {
            for &u in scc {
                assert_eq!(belong[u], usize::MAX, "node {} appears in multiple SCCs", u);
                belong[u] = cid;
            }
        }
        assert!(
            belong.iter().all(|&x| x != usize::MAX),
            "some nodes are missing"
        );

        // Nodes in the same SCC must be mutually reachable
        for scc in &sccs {
            for &u in scc {
                let ru = reachable(&directed_graph, u);
                for &v in scc {
                    assert!(ru[v], "node {} should reach {} in the same SCC", u, v);
                }
            }
        }

        // Nodes in different SCCs must not be mutually reachable both ways
        for u in 0..size {
            let ru = reachable(&directed_graph, u);
            for v in 0..size {
                if belong[u] != belong[v] && ru[v] {
                    let rv = reachable(&directed_graph, v);
                    assert!(
                        !rv[u],
                        "nodes {} and {} are mutually reachable but placed in different SCCs",
                        u, v
                    );
                }
            }
        }
    }
}
