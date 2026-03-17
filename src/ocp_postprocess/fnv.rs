//! FNV-1 hash algorithm implementation.
//!
//! This module implements the Fowler-Noll-Vo (FNV-1) non-cryptographic hash function
//! in both 32-bit and 64-bit variants. FNV-1 is a fast hash function with good
//! distribution properties, suitable for hash tables and checksums.
//!
//! The implementation follows the FNV-1 specification using multiplication and XOR
//! operations. This is **not** suitable for cryptographic purposes.

pub(crate) fn fnv1_32(data: &[u8]) -> u32 {
    let mut hash = 0x811c9dc5u32;
    for byte in data {
        hash = hash.wrapping_mul(0x01000193);
        hash ^= u32::from(*byte);
    }
    hash
}

pub(crate) fn fnv1_64(data: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in data {
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(*byte);
    }
    hash
}
