# Range query point update

## Template
```rust,editable,ignore
use std::ops::RangeBounds;
{{#include ../../code/src/range_query/fwtree.rs:RangeQueryPointUpdate}}
```

## Usage
### Init
```rust,editable,ignore
let mut fw = FwTree::new(n);
let mut fw = FwTree::from(&vec);
```

### Update
**add**
```rust,editable,ignore
fw.add(index, val);
```

**set**
```rust,editable,ignore
fw.set(index, val);
```

### Query
```rust,editable,ignore
fw.sum(l, r);
fw.sum_to(l); // sum(0, l)
fw.sum_from(r); // sum(r, n)
```

### right_index_with_sum_from_k, left_index_with_sum_from_k
```rust,editable,ignore
fw.right_index_with_sum_from_k(l, k);
```

```rust,editable,ignore
fw.left_index_with_sum_from_k(r, k);
```