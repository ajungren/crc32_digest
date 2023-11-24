# `crc32_digest`

[![Build and Test](https://github.com/lookbusy1344/crc32_digest/actions/workflows/rust.yml/badge.svg)](https://github.com/lookbusy1344/crc32_digest/actions/workflows/rust.yml)

A fork of https://github.com/ajungren/crc32_digest to support the latest version of Digest, and Rust 1.74 (November 2023)

An implementation of the [`digest`][crate:digest] crate's [`Digest`] and [`DynDigest`] traits using
[`crc32fast`][crate:crc32fast].

If [`digest`][crate:digest] is built with the `std` feature enabled, `Crc32` will implement [`Write`] as well.

Internally, the `Crc32` struct provided by this crate implements the [`FixedOutput`], [`Input`], and [`Reset`] traits. A
blanket `impl` of [`Digest`] and [`DynDigest`] is provided by [`digest`][crate:digest] for types implementing those
traits (along with `Clone` and `Default`).

## Requirements

Updated for Rust 1.74 and Digest 0.10.7

[`Write`] support requires the `std` feature of `digest` to be enabled.

## Usage

```rust
use crc32_digest::Crc32;
use digest::Digest;

fn main() {
    let mut crc32 = Crc32::new();
    crc32.update(b"hello, world");
    let result = crc32.finalize();

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
