## VecDeque
Equivalent to `std:deque` in C++

```rust,editable,ignore
use std::collections::VecDeque;

let mut q = VecDeque::new();
q.push_back(1);
q.push_back(2);
q.pop_back();
q.pop_front();
q.is_empty();
q.len();
for i in q.iter() {} // iter_mut()
```
