use crate::u128_low_high::LowHigh;
use core::mem::MaybeUninit;
use std::ffi::c_char;

extern "C" {
    fn CityHash64(s: *const c_char, len: usize) -> u64;

    fn CityHash128(
        buf: *const c_char,
        len: usize,
        hash_low_128_half: *mut u64,
        hash_high_128_half: *mut u64,
    );
}

// See
// https://github.com/HUD-Software/cityhash-sys/blob/master/src/cityhash_portable.rs
// #[inline]
// #[must_use]
pub fn city_hash_64(bytes: &[u8]) -> u64 {
    unsafe { CityHash64(bytes.as_ptr() as *const i8, bytes.len()) }
}

// See
// https://github.com/HUD-Software/cityhash-sys/blob/master/src/cityhash_portable.rs
#[inline]
#[must_use]
pub fn city_hash_128(bytes: &[u8]) -> u128 {
    unsafe {
        let mut low_128_half = MaybeUninit::<u64>::uninit();
        let mut high_128_half = MaybeUninit::<u64>::uninit();
        CityHash128(
            bytes.as_ptr() as *const i8,
            bytes.len(),
            low_128_half.as_mut_ptr(),
            high_128_half.as_mut_ptr(),
        );
        u128::from_halfs(high_128_half.assume_init(), low_128_half.assume_init())
    }
}
