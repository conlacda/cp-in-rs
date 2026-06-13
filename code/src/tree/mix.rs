#![allow(unused)]
// ANCHOR: eulertour
use crate::recursive::Callable2;
use crate::recursive::RecursiveFunction2;

pub fn make_eulertour(tree: &[Vec<usize>], root: usize) -> (Vec<usize>, Vec<Vec<usize>>) {
    let n = tree.len();
    let mut eulertour: Vec<usize> = Vec::new();
    let mut dfs = RecursiveFunction2::new(|dfs, u: usize, p: usize| {
        eulertour.push(u);
        for &v in tree[u].iter() {
            if v != p {
                dfs.call(v, u);
            }
        }
        eulertour.push(u);
    });
    dfs.call(root, root);
    let mut inout: Vec<Vec<usize>> = vec![vec![]; n];
    for (index, &val) in eulertour.iter().enumerate() {
        inout[val].push(index);
    }
    (eulertour, inout)
}
// ANCHOR_END: eulertour

