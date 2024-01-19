# fi2b

![Rust](https://github.com/2A5F/fi2b/workflows/Rust/badge.svg)
[![version](https://img.shields.io/crates/v/fi2b)](https://crates.io/crates/fi2b)
[![documentation](https://docs.rs/fi2b/badge.svg)](https://docs.rs/fi2b)
![LICENSE](https://img.shields.io/crates/l/fi2b)

Convert floating point and integer numbers into bytes and put them into an array

```rust
let a: &[u8] = &fi2b![1, 2, 1.0, -1.0];
assert_eq!(
    a,
    [1u8, 0u8, 0u8, 0u8, 2u8, 0u8, 0u8, 0u8, 0u8, 0u8, 128u8, 63u8, 0u8, 0u8, 128u8, 191u8]
)
```
