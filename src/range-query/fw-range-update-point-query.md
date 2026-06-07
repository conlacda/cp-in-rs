# Range update point query

## Template
```rust showLineNumbers
{{#include ../../code/src/range_query/fwtree.rs:RangeUpdatePointQuery}}
```

## Usage
### Init
```rust
let mut fw = FenwickTree::new(n);
let mut fw = FenwickTree::from(&vec);
```

### Update
```rust
fw.add(l, r, val);
```

### Query
```rust
fw.at(index);
```