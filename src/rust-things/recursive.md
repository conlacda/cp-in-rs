# Recursive

## Template
```rust,editable
{{#include ../../code/src/recursive.rs:main}}
```

## Usage
```rust,ignore
use crate::recursive::Callable;
use crate::recursive::RecursiveFunction;

let mut dfs = RecursiveFunction::new(|dfs, u: usize| {
    dfs.call(u);
});
dfs.call(0);
```
