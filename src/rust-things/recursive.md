# Recursive

## Template
```rust,editable,ignore
{{#include ../../code/src/recursive.rs:main}}
```

## Usage
```rust,editable,ignore
use crate::recursive::Callable;
use crate::recursive::RecursiveFunction;

let mut dfs = RecursiveFunction::new(|dfs, u: usize| {
    dfs.call(u);
});
dfs.call(0);
```
