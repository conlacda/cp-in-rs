#![allow(unused)]
// ANCHOR: eulertour
use crate::recursive::Callable2;
use crate::recursive::RecursiveFunction2;

pub fn make_eulertour(tree: &[Vec<usize>], root: usize) -> (Vec<usize>, Vec<(usize, usize)>) {
    let n = tree.len();
    let mut eulertour: Vec<usize> = Vec::new();
    let mut inout: Vec<(usize, usize)> = vec![(0, 0); n];
    let mut clock: usize = 0;
    let mut dfs = RecursiveFunction2::new(|dfs, u: usize, p: usize| {
        eulertour.push(u);
        inout[u].0 = clock;
        clock += 1;
        for &v in tree[u].iter() {
            if v != p {
                dfs.call(v, u);
            }
        }
        inout[u].1 = clock;
        clock += 1;
        eulertour.push(u);
    });
    dfs.call(root, root);
    (eulertour, inout)
}
// ANCHOR_END: eulertour

#[cfg(test)]
mod tests {
    use super::*;
    use crate::random::Random;

    #[test]
    fn test_euler_tour() {
        let tree: Vec<Vec<usize>> = vec![
            vec![1, 4],
            vec![2],
            vec![3],
            vec![],
            vec![5],
            vec![6],
            vec![7],
            vec![8],
            vec![9],
            vec![],
        ];
        let (expect_eulertour, expect_inout) = (
            [0, 1, 2, 3, 3, 2, 1, 4, 5, 6, 7, 8, 9, 9, 8, 7, 6, 5, 4, 0],
            [
                (0, 19),
                (1, 6),
                (2, 5),
                (3, 4),
                (7, 18),
                (8, 17),
                (9, 16),
                (10, 15),
                (11, 14),
                (12, 13),
            ],
        );
        let (eulertour, inout) = make_eulertour(&tree, 0);
        assert_eq!(expect_eulertour.to_vec(), eulertour);
        assert_eq!(expect_inout.to_vec(), inout);
    }
}
