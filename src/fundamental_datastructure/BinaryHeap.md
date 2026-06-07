## BinaryHeap (MaxHeap)
Equivalent to `std::priority_queue` in C++

```rust
use std::collections::BinaryHeap;
let mut pq = BinaryHeap::new();
pq.push(5);
pq.push(10);
pq.push(1);

println!("{:?}", pq.pop()); // Some(10)
```

**For min heap (min first)**
```rust showLineNumbers
use std::cmp::Reverse;
use std::collections::BinaryHeap;

let mut heap = BinaryHeap::new();
heap.push(Reverse(5));
heap.push(Reverse(1));
heap.push(Reverse(10));

while let Some(Reverse(x)) = heap.pop() {
    println!("{}", x); // 1, 5, 10
}
```
