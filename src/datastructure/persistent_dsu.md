# Persistent DSU

## Template
```rust,editable,ignore
{{#include ../../code/src/datastructure/persistent_dsu.rs:main}}
```

## Usage
### Init
```rust,editable,ignore
let mut dsu = PersistentDsu::new(n);
```

### Find
```rust,editable,ignore
let root = dsu.find(a);
```

### Merge, rollback
```rust,editable,ignore
let merge = dsu.merge(a, b);
dsu.rollback();
```

### Check if same group
```rust,editable,ignore
dsu.is_same(a, b);
```

## Practice problems
- [Persistent Unionfind](https://judge.yosupo.jp/problem/persistent_unionfind)
  - [Solution](../verify/dsu/persistent_dsu_library_checker.rs)
