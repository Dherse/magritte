use std::ffi::CStr;

use const_fnv1a_hash::fnv1a_hash_64;

pub const fn const_hash(value: &CStr) -> u64 {
    fnv1a_hash_64(value.to_bytes(), None)
}

pub const fn const_hash_str(value: &str) -> u64 {
    fnv1a_hash_64(value.as_bytes(), None)
}
