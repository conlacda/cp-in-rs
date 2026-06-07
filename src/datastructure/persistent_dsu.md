# Persistent DSU

## Template
```rust 
{{#include ../../code/src/persistent_dsu.rs:main}}
```

## Usage
### Init
```rust
let mut dsu = PersitsentDsu::new(n);
```

### Find
```rust
let root = dsu.find(a);
```

### Merge, rollback
```rust
let merge = dsu.merge(a, b);
dsu.rollback();
```

### Check if same group
```rust
dsu.is_same(a, b);
```

## Practice problems
- [Persistent Unionfind](https://judge.yosupo.jp/problem/persistent_unionfind)
  - [Solution](../verify/dsu/persistent_dsu_library_checker.rs)
