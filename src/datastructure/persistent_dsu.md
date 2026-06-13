# Persistent DSU

## Template
```rust 
{{#include ../../code/src/datastructure/persistent_dsu.rs:main}}
```

## Usage
### Init
```rust,ignore
let mut dsu = PersistentDsu::new(n);
```

### Find
```rust,ignore
let root = dsu.find(a);
```

### Merge, rollback
```rust,ignore
let merge = dsu.merge(a, b);
dsu.rollback();
```

### Check if same group
```rust,ignore
dsu.is_same(a, b);
```

## Practice problems
- [Persistent Unionfind](https://judge.yosupo.jp/problem/persistent_unionfind)
  - [Solution](../verify/dsu/persistent_dsu_library_checker.rs)
