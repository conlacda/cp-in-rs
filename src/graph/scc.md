# SCC
## Template
```rust 
{{#include ../../code/src/graph.rs:find_scc}}
```

## Usage
```rust,ignore
let directed_graph = Random::new().directed_graph(size);
let sccs: Vec<Vec<usize>> = find_scc(&directed_graph);
```
