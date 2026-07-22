// https://judge.yosupo.jp/submission/387280
#![allow(dead_code)]
use rs_space::datastructure::persistent_dsu::PersistentDsu;
use rs_space::sw::scanner;
use rs_space::sw::writer;
use std::io::Write;

#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::dbg;
#[cfg(feature = "local")]
#[allow(unused_imports)]
use rs_space::set_limit::timeout_secs;

struct Query {
    t: usize,
    k: usize,
    u: usize,
    v: usize,
}

fn main() {
    #[cfg(feature = "local")]
    timeout_secs(5);

    let mut scan = scanner();
    let mut out = writer();

    let n: usize = scan.token();
    let q: usize = scan.token();

    let mut queries = Vec::with_capacity(q + 1);
    queries.push(Query {
        t: 0,
        k: 0,
        u: 0,
        v: 0,
    });

    let mut queries_using_k = vec![Vec::<usize>::new(); q + 1];

    for i in 1..=q {
        let t: usize = scan.token();
        let k: isize = scan.token();
        let u: usize = scan.token();
        let v: usize = scan.token();

        let k = (k + 1) as usize;

        queries.push(Query { t, k, u, v });

        queries_using_k[k].push(i);
    }

    let mut ans = vec![false; q + 1];
    let mut dsu = PersistentDsu::new(n);

    fn dfs(
        idx: usize,
        queries: &[Query],
        queries_using_k: &[Vec<usize>],
        ans: &mut [bool],
        dsu: &mut PersistentDsu,
    ) {
        if queries[idx].t == 0 {
            dsu.merge(queries[idx].u, queries[idx].v);
        } else if queries[idx].t == 1 {
            ans[idx] = dsu.is_same(queries[idx].u, queries[idx].v);
        }

        for &child in &queries_using_k[idx] {
            dfs(child, queries, queries_using_k, ans, dsu);
        }

        if queries[idx].t == 0 {
            dsu.rollback();
        }
    }

    dfs(0, &queries, &queries_using_k, &mut ans, &mut dsu);

    for i in 1..=q {
        if queries[i].t == 1 {
            writeln!(out, "{}", if ans[i] { 1 } else { 0 }).unwrap();
        }
    }
}
