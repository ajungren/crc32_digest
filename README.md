# `crc32_digest`

[![Build Status](https://travis-ci.org/ajungren/crc32_digest.svg?branch=master)](https://travis-ci.org/ajungren/crc32_digest)
[![Crate](https://img.shields.io/crates/v/crc32_digest.svg)](https://crates.io/crates/crc32_digest)
[![API](https://docs.rs/crc32_digest/badge.svg)](https://docs.rs/crc32_digest/)
[![Minimum rustc version](https://img.shields.io/badge/rustc-1.32+-lightgray.svg)](https://github.com/ajungren/crc32_digest#requirements)

An implementation of the [`digest`][crate:digest] crate's [`Digest`] and [`DynDigest`] traits using
[`crc32fast`][crate:crc32fast].

If [`digest`][crate:digest] is built with the `std` feature enabled, `Crc32` will implement [`Write`] as well.

Internally, the `Crc32` struct provided by this crate implements the [`FixedOutput`], [`Input`], and [`Reset`] traits. A
blanket `impl` of [`Digest`] and [`DynDigest`] is provided by [`digest`][crate:digest] for types implementing those
traits (along with `Clone` and `Default`).

## Requirements

Rust 1.32 or newer is required for `u32::to_be_bytes`.

[`Write`] support requires the `std` feature of `digest` to be enabled.

## Usage

```rust
use crc32_digest::Crc32;
use digest::Digest;

fn main() {
    let mut crc32 = Crc32::new();
    crc32.input(b"hello, world");
    let result = crc32.result();
    
    // Get checksum as a byte slice
    assert_eq!(result.as_slice(), &[0xff, 0xab, 0x72, 0x3a]);
    // Format checksum as a lowercase hexadecimal string
    assert_eq!(format!("{:x}", result), "ffab723a");
}
```

Alternatively, `Crc32::from_state(state: u32)` can be used to create a new `Crc32` instance with a specific initial
state.

[crate:crc32fast]: https://crates.io/crates/crc32fast
[crate:digest]: https://crates.io/crates/digest
[`Digest`]: https://docs.rs/digest/latest/digest/trait.Digest.html
[`DynDigest`]: https://docs.rs/digest/latest/digest/trait.DynDigest.html
[`FixedOutput`]: https://docs.rs/digest/latest/digest/trait.FixedOutput.html
[`Input`]: https://docs.rs/digest/latest/digest/trait.Input.html
[`Reset`]: https://docs.rs/digest/latest/digest/trait.Reset.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
