# RMQ
Range minimum/maximum query

## Template
```rust,editable
{{#include ../../code/src/range_query/rmq.rs:main}}
```

## Usage
### Init
```rust
let mut rmq: RMQ<i64> = RMQ::default();
rmq.set_max_mode(true).from(vec![0; 100]).build();
```

### Query
```rust
max_index = rmq.query_index(l, r);
max_value = rmq.query_value(l, r);
```