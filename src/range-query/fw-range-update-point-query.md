# Range update point query

## Template
```rust,editable
{{#include ../../code/src/range_query/fwtree.rs:RangeUpdatePointQuery}}
```

## Usage
### Init
```rust,ignore
let mut fw = FenwickTree::new(n);
let mut fw = FenwickTree::from(&vec);
```

### Update
```rust,ignore
fw.add(l, r, val);
```

### Query
```rust,ignore
fw.at(index);
```