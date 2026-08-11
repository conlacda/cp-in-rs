# Multiset

## Initialization

```rust,editable,ignore
use rs_space::datastructure::multiset::Multiset;

let mut mts = Multiset::new(1_000_000);
```

## Insert

```rust,editable,ignore
mts.insert(10);
```

## Remove/Clear

```rust,editable,ignore
mts.remove(10);
mts.remove_all(10);
mts.clear();
```

## Length

```rust,editable,ignore
mts.len();
mts.is_empty();
```

## Count

```rust,editable,ignore
mts.count(20);
mts.count_less(20);
mts.count_greater(-10);
mts.count_between(5, 15);
```

## Get a value by index

```rust,editable,ignore
mts.at(index); // Option<i64>
```

## Find the index of a value

Returns the smallest index for which `a[index] >= value`. If duplicate values
exist, it returns the index of the first occurrence.

```rust,editable,ignore
mts.index_of(value);
```

## Nearest value
```rust,editable,ignore
mts.nearest_down(value); // returns the greatest X in mts such that X <= value
mts.nearest_up(value);   // returns the smallest X in mts such that X >= value
```