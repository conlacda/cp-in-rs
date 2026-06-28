# RMQ
Range minimum/maximum query

## Template
```rust,editable
{{#include ../../code/src/range_query/rmq.rs:main}}
```

## Usage
### Init
```rust,ignore
let rmq: RMQ<i64> = RMQ::new(&vec![], true);
```

### Query
```rust,ignore
max_index = rmq.query_index(l, r);
max_value = rmq.query_value(l, r);
```