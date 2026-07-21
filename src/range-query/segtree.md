# Segment tree

## Template
```rust,editable,ignore
{{#include ../../code/src/range_query/segtree.rs:segtree}}
```

## Nodes
### SumNode
```rust,editable,ignore
{{#include ../../code/src/range_query/segtree.rs:SumNode}}
```

### MinNode
```rust,editable,ignore
{{#include ../../code/src/range_query/segtree.rs:MinNode}}
```

### MaxNode
```rust,editable,ignore
{{#include ../../code/src/range_query/segtree.rs:MaxNode}}
```

### Subrange
```rust,editable,ignore
{{#include ../../code/src/range_query/segtree.rs:Subrange}}
```

## Usage
### Init
```rust,editable,ignore
let nodes: Vec<MaxNode> = (0..n).map(|_| MaxNode::new(s.token())).collect();
let mut seg = SegTree::from(&nodes);
```

### Set value
```rust,editable,ignore
seg.set(index, &MaxNode::new(val));
```

### Query
```rust,editable,ignore
let node = seg.query(u - 1, v - 1);
```

### Find right/left (binary search)
```rust,editable,ignore
seg.find_right(start_index, |range_node| range_node.val >= v);
seg.find_left(end_index, |range_node| range_node.val <= v);
```
