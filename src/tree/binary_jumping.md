## Binary jumping
> aka binary lifting

### Template
```rust,editable
{{#include ../../code/src/tree/binary_jumping.rs:main}}
```

### Usage
```rust,ignore
let bj = BinaryJumping::new(&parent, max_query_depth);
let node = bj.kth_parent(cur_node, k);
```

### Practice problems
- [Planets Queries I](https://cses.fi/problemset/task/1750/)
  - [Solution](../verify/tree/Planets_Queries_I.rs)
