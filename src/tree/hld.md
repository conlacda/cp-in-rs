# Heavy-light decomposition (HLD)

## Template
```rust,editable,ignore
{{#include ../../code/src/tree/hld.rs:main}}
```

## Usage
### Init
```rust,editable,ignore
let mut hld: HLD<dyn Node> = HLD::new(&tree, &weight);
hld.set_weight_on_nodes(true); // weight on node OR weight on edge?
```

### Set
```rust,editable,ignore
hld.set_node(u: usize, node: Node);
hld.set_edge((u, v): (usize, usize), node: Node);
```

### Query
```rust,editable,ignore
hld.query_subtree(root: usize);
hld.query_path(u: usize, v: usize);
```

### Distance
```rust,editable,ignore
hld.distance(u: usize, v: usize);
```

### Practice problems
- [Path Queries II](https://cses.fi/problemset/task/2134/)
  - [Solution](../verify/tree/Path_Queries_II.rs)
- [G - Distance Queries on a Tree](https://atcoder.jp/contests/abc294/tasks/abc294_g)
  - [Solution](https://atcoder.jp/contests/abc294/submissions/77271110)
