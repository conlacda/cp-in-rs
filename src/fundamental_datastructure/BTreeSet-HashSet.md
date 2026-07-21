# Set
HashSet is equivalent to `std::unordered_set` in c++, while BTreeSet is equivalent to `std::set`

```rust,editable,ignore
use std::collections::HashSet;
use std::collections::BTreeSet;

let mut s: HashSet<i64> = HashSet::new(); // BTreeSet::new()
s.insert(2);
s.remove(&2);
s.contains(&3);
s.len();
s.is_empty();
for i in s.iter() {
    dbg!(i);
}
```