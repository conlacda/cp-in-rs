# DSU

## Template
```rust 
{{#include ../../code/src/datastructure/dsu.rs:main}}
```

## Usage
```rust
let mut dsu = DSU::new(n);
dsu.find(u);
dsu.merge(u, v);
dsu.is_same(u, v);

let node = dsu.node_mut(index);
*node.value = 3;
```
