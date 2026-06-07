# Weight DSU

## Template
```rust 
{{#include ../../code/src/weight_dsu.rs:main}}
```

## Usage
```rust
let mut dsu = WeightDsu::new(n);
dsu.find(1);
dsu.merge(a, b, dist); // height[a] - height[b] = dist
dsu.is_same(a, b);
dsu.distance(a, b);
```
