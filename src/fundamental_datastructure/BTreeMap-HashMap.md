# BTreeMap/HashMap
```rust
use std::collections::HashMap;
use std::collections::BTreeMap;

let mut map = BTreeMap::new(); // HashMap::new()

map.len();
map.contains_key(&2);
map.is_empty();
// Loop
for (k, v) in &map {
    println!("{k} -> {v}");
}
```
**Insert**
```rust
map.insert(key, value);
```
Insert again with the same key to overwrite its value.

**Get a value**
```rust
if let Some(val) = map.get(&key) {
    dbg!(val);
}
// OR
if map.contains_key(&key) {
    let val = map.get(&key).unwrap();
}
```

**Update a value**
```rust
if let Some(value) = map.get_mut(&key) {
    *value = new_value; // *value += 3;
}
OR
if map.contains_key(&key) {
    map.insert(key, new_value);
    // OR
    map.get_mut(&2).unwrap() += 3;
}
```
To set a value regardless of whether the key already exists, simply use `map.insert(key, value)`.

**Remove**
```rust
map.remove(&key);
```
