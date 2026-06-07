# BTreeMap/HashMap
```rust showLineNumbers
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
```rust showLineNumbers
map.insert(key, value);
```
insert 2 lần để ghi đè value

**Lấy ra value**
```rust showLineNumbers
if let Some(val) = map.get(&key) {
    dbg!(val);
}
// OR
if map.contains_key(&key) {
    let val = map.get(&key).unwrap();
}
```

**Cập nhật key value**
```rust showLineNumbers
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
Cập nhật value mà không quan tâm key đã tồn tại hay chưa, dùng `map.insert(key, value)` là xong

**Remove**
```rust showLineNumbers
map.remove(&key);
```
