# Heavy-light decomposition (HLD)

## Template
```rust,editable,ignore
{{#include ../../code/src/tree/hld.rs:main}}
```

## Usage
### Init
```rust,ignore
let mut hld = HLD::new(&graph, &weight);
```

### Set
```rust,ignore
hld.set_node(u: usize, node: Node);
hld.set_edge(u: usize, v: usize, node: Node);
```

### Query
```rust,ignore
hld.query_subtree(root: usize);
hld.queyry_path(u: usize, v: usize);
```

### Distance
```rust,ignore
hld.distance(u: usize, v: usize);
```

### Practice problems
- [Path Queries II](https://cses.fi/problemset/task/2134/)
  - [Solution](../verify/tree/Path_Queries_II.rs)
