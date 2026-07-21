# Mod int

## Template
```rust 
{{#include ../../code/src/math/mint.rs:main}}
```

## Usage
### Init
```rust,ignore
const MOD: u32 = 1000000007;
let m: Mint<MOD> = 1000000008.into();
let m = Mint::<MOD>::from(1000000008);
assert!(m.val == 1);
```

### inv
```rust,ignore
assert!(m * m.inv() == 1);
```

### factor
```rust,editable,ignore
m.factor(); // m! = 1*2*3*...*m
```

### ncr, npr
```rust,editable,ignore
n.ncr(r);
n.npr(r);
```

### pow
```rust,editable,ignore
m.pow(x); // m^x
```

### basic operations
```rust,editable,ignore
let mut m: Mint<MOD> = 3.into();
m += 3.into();
m -= 2.into();
m /= 3.into();
m *= 3.into();
```