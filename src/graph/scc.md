# SCC
## Template
```rust,editable,ignore
{{#include ../../code/src/graph/find_scc.rs:main}}
```

## Usage
```rust,editable,ignore
let directed_graph = Random::new().directed_graph(size);
let sccs: Vec<Vec<usize>> = find_scc(&directed_graph);
```
