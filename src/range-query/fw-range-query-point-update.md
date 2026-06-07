# Range query point update

## Template
```rust 
{{#include ../../code/src/range_query/fwtree.rs:RangeQueryPointUpdate}}
```

## Usage
### Init
```rust
let mut fw = FwTree::new(n);
let mut fw = FwTree::from(&vec);
```

### Update
**add**
```rust
fw.add(index, val);
```

**set**
```rust
fw.set(index, val);
```

### Query
```rust
fw.sum(l, r);
fw.sum_to(l); // sum(0, l)
fw.sum_from(r); // sum(r, n)
```

### right_index_with_sum_from_k, left_index_with_sum_from_k
```rust
fw.right_index_with_sum_from_k(l, k);
```

```rust
fw.left_index_with_sum_from_k(r, k);
```