use crate::recursive::Callable;
use crate::recursive::RecursiveFunction;
// ANCHOR: main
pub fn toposort(g: &[Vec<usize>]) -> Vec<usize> {
    let n = g.len();
    let mut vis: Vec<bool> = vec![false; n];
    let mut order: Vec<usize> = Vec::new();
    let mut dfs = RecursiveFunction::new(|dfs, u: usize| {
        if vis[u] {
            return;
        }
        vis[u] = true;
        for &v in &g[u] {
            dfs.call(v);
        }
        order.push(u);
    });
    for i in 0..g.len() {
        dfs.call(i);
    }
    order.reverse();
    order
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
}
