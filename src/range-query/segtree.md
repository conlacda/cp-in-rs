# Segment tree

## Template
```rust
{{#include ../../code/src/segtree.rs:segtree}}
```

## Nodes
### SumNode
```rust
{{#include ../../code/src/segtree.rs:SumNode}}
```

### MinNode
```rust
{{#include ../../code/src/segtree.rs:MinNode}}
```

### MaxNode
```rust
{{#include ../../code/src/segtree.rs:MaxNode}}
```

### Subrange
```rust
{{#include ../../code/src/segtree.rs:Subrange}}
```

## Usage
### Init
```rust
let nodes: Vec<MaxNode> = (0..n).map(|_| MaxNode::new(s.token())).collect();
let mut seg = SegTree::from(&nodes);
```

### Set value
```rust
seg.set(index, &MaxNode::new(val));
```

### Query
```rust
let node = seg.query(u - 1, v - 1);
```

### Find right/left (binary search)
```rust
seg.find_right(start_index, |range_node| range_node.val >= v);
seg.find_left(end_index, |range_node| range_node.val <= v);
```
