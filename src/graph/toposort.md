# Toposort

## Template
```rust 
{{#include ../../code/src/graph.rs:toposort}}
```

## Usage
```rust,ignore
let topo_order: Vec<usize> = toposort(&dag);
```
Topo is used for only DAG, see this graph `0 <-> 1, 1 <-> 2, 2 <-> 0, 0 <-> 3`, topo sort will be `0 3 1 2`, it is not correct when consider the last one is sink component in graph.
