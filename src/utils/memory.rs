use libc;
use std::mem;

use super::error::{Result, SparserError};
const ALIGNMENT: usize = 64;

#[cfg(windows)]
#[link(name = "msvcrt")]
extern "C" {
    fn _aligned_malloc(size: libc::size_t, alignment: libc::size_t) -> libc::size_t;
    fn _aligned_free(prt: *const u8);
}

#[cfg(windows)]
pub fn allocate_aligned(size: i64) -> Result<*mut u8> {
    let page = unsafe { _aligned_malloc(size as libc::size_t, ALIGNMENT as libc::size_t) };
    match page {
        0 => Err(SparserError::MemoryError(
            "Failed to allocate memory".to_string(),
        )),
        _ => Ok(unsafe { mem::transmute::<libc::size_t, *mut u8>(page) }),
    }
}

#[cfg(not(windows))]
pub fn allocate_aligned(size: i64) -> Result<*mut u8> {
    unsafe {
        let mut page: *mut libc::c_void = mem::uninitialized();
        let result = libc::posix_memalign(&mut page, ALIGNMENT, size as usize);
        match result {
            0 => Ok(mem::transmute::<*mut libc::c_void, *mut u8>(page)),
            _ => Err(SparserError::MemoryError(
                "Failed to allocate memory".to_string(),
            )),
        }
    }
}

#[cfg(windows)]
pub fn free_aligned(p: *const u8) {
    unsafe {
        _aligned_free(p);
    }
}

#[cfg(not(windows))]
pub fn free_aligned(p: *const u8) {
    unsafe {
        libc::free(mem::transmute::<*const u8, *mut libc::c_void>(p));
    }
}

pub unsafe fn memcpy(dst: *mut u8, src: *const u8, len: usize) {
    let src = mem::transmute::<*const u8, *const libc::c_void>(src);
    let dst = mem::transmute::<*mut u8, *mut libc::c_void>(dst);
    libc::memcpy(dst, src, len);
}

extern "C" {
    #[inline]
    pub fn memcmp(p1: *const u8, p2: *const u8, len: usize) -> i32;
}

/// Check if the pointer `p` is aligned to offset `a`.
pub fn is_aligned<T>(p: *const T, a: usize) -> bool {
    let a_minus_one = a.wrapping_sub(1);
    let pmoda = p as usize & a_minus_one;
    pmoda == 0
}
