# Recursive

## Template
```rust showLineNumbers
{{#include ../../code/src/recursive.rs:main}}
```

## Usage
```rust showLineNumbers
use crate::recursive::Callable;
use crate::recursive::RecursiveFunction;

let mut dfs = RecursiveFunction::new(|dfs, u: usize| {
    dfs.call(u);
});
dfs.call(0);
```
