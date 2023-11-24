extern crate crc32_digest;
extern crate digest;

use crc32_digest::Crc32;
use digest::Digest;

macro_rules! test {
    ($name:ident, $input:expr, $hash:expr) => {
        #[test]
        fn $name() {
            let mut crc32 = Crc32::new();
            crc32.update($input);
            let checksum = crc32.finalize();
            assert_eq!(format!("{:x}", checksum), $hash);
        }
    };
}

// Test cases are borrowed from:
// https://github.com/srijs/rust-crc32fast/blob/master/src/baseline.rs
test!(hash_empty, [0x00; 0], "00000000");
test!(hash_ones, [0xff; 32], "ff6cab0b");
test!(hash_zeroes, [0x00; 32], "190a55ad");
test!(hash_ascending, b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F", "91267e8a");

#[test]
fn reset() {
    let mut crc32 = Crc32::new();
    crc32.update(b"hello, world");
    crc32.reset();
    let checksum = crc32.finalize();
    assert_eq!(format!("{:x}", checksum), "00000000");
}
