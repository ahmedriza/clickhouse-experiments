use std::ffi::c_char;

extern "C" {
    fn CityHash64(s: *const c_char, len: usize) -> u64;
}

// #[inline]
// #[must_use]
pub fn city_hash_64(bytes: &[u8]) -> u64 {
    unsafe { CityHash64(bytes.as_ptr() as *const i8, bytes.len()) }
}
