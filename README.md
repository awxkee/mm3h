Murmur3Hasher

A Rust implementation of MurmurHash3 with platform-specific SIMD optimizations (NEON, AVX2, SSE2). This crate provides a Hasher compatible struct that can be used with Rust's hashing APIs.

```rust
use std::hash::{Hasher, Hash};
use murmur3_hasher::Murmur3Hasher;

fn main() {
    let mut hasher = Murmur3Hasher::new_with_seed(42);

    let data = b"Hash me!";
    hasher.write(data);

    let hash_value = hasher.finish();
    println!("Hash: {}", hash_value);
}
```

----

This project is licensed under either of

- BSD-3-Clause License (see [LICENSE](LICENSE.md))
- Apache License, Version 2.0 (see [LICENSE](LICENSE-APACHE.md))

at your option.