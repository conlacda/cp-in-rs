# Range update point query

## Template
```rust,editable,ignore
{{#include ../../code/src/range_query/fwtree.rs:RangeUpdatePointQuery}}
```

## Usage
### Init
```rust,editable,ignore
let mut fw = FenwickTree::new(n);
let mut fw = FenwickTree::from(&vec);
```

### Update
```rust,editable,ignore
fw.add(l, r, val);
```

### Query
```rust,editable,ignore
fw.at(index);
```