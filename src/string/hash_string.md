# Hash string

## Template

```rust showLineNumbers
{{#include ../../code/src/string/hashstr.rs:hash-function}}
```

## Usage

### Init
```rust showLineNumbers
Hash::init(250000);
let str_hash = Hash::new(String::from("aabaa").as_bytes());
let hash = Hash::new(&[1, 2, 4]);
```
### Get hash value
```rust showLineNumbers
Hash::once(&[1,2,3]);
hash.substr(start, end);  // hash value of s[start..=end]
hash.rolling(start, end); // same as substr but end might be less than start
```

### Common prefix
```rust showLineNumbers
hash.common_prefix(start1, start2);
```

### Palindrome
```rust showLineNumbers
Hash::is_palindrome(hash_value);
hash.is_substr_palindrome(start, end);
```

### Reversed hash
```rust showLineNumbers
Hash::reversed(hash_value);
```

### Merge
`merge(h1, len1, h2, len2)` returns the hash of `A + B`.
```rust showLineNumbers
Hash::merge(hashA, lenA, hashB, lenB);
```