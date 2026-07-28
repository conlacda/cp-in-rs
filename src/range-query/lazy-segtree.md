# Lazy segment tree

## Template
```rust,editable,ignore
{{#include ../../code/src/range_query/lazysegtree.rs:lazysegtree}}
```

## Nodes
### Range Affine
```rust,editable,ignore
{{#include ../../code/src/range_query/lazysegtree.rs:range_affine}}
```

## Usage
### Init
```rust,editable,ignore
let nodes: Vec<RangeAffineSumNode> = (0..n)
    .map(|_| RangeAffineSumNode::new(scan.token()))
    .collect();
let mut seg = LazySegTree::from(&nodes);
```

### Set value
```rust,editable,ignore
seg.set(index, RangeAffineSumNode::new(value));
```

### Query
```rust,editable,ignore
// Both endpoints are inclusive.
let range = seg.query(left, right).val;
let whole = seg.query_all().val;
```

### Range affine update
```rust,editable,ignore
// array[l..=r] = a * array[l..=r] + b
seg.update(left, right, RangeAffineLazyNode { a, b });
```

### Practice problems
- [K - Range Affine Range Sum](https://atcoder.jp/contests/practice2/tasks/practice2_k)
  - [Solution](../verify/range-query/Range_Affine_Range_Sum.rs)
- [Cses - Range Updates and Sums](https://cses.fi/problemset/task/1735/)
  - [Solution](../verify/range-query/Range_Updates_and_Sums.rs)
