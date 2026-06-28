# Coordinate compress

## Template

```rust,editable
{{#include ../../code/src/techniques/coordinate_compress.rs:main}}
```

## Usage

### Init
Run in `O(NlogN)`
```rust,ignore
let c = Compress::new(&[20, 40, 10, 30]);
```

### Get by value
Run in `O(logN)`
```rust,ignore
// `up(x)` returns the compressed index of the smallest value >= x.
assert!(c.up(&21) == Some(2));
// `down(x)` returns the compressed index of the largest value <= x.
assert!(c.down(&21) == Some(1));
```

### Get compressed value of a[index]
Run in `O(1)`

`c.by_index(index) = compress of a[index]`
```rust,ignore
assert!(c.by_index(20) == 1);
```

### Get original value by compressed value
Run in `O(1)`

```rust,ignore
c.original_val(compressed_value); // = value before compressing
```
