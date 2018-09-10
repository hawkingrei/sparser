use std::sync::Arc;
use libc;

use utils::memory;

const WORDSZ: usize = 64;

#[derive(PartialEq, Debug)]
pub struct Bitmap {
    bits: Arc<BitData>,
    capacity: usize,
    words: usize,
}

#[derive(Debug)]
struct BitData {
    /// The raw pointer into the buffer bytes
    ptr: *const u8,

    /// The length of the buffer
    count: usize,
}

impl Bitmap {
    /*
    pub fn new(num_bits: usize) -> Self {
        let num_bytes = num_bits / 8 + if num_bits % 8 > 0 { 1 } else { 0 };
        let r = num_bytes % 64;
        let len = if r == 0 {
            num_bytes
        } else {
            num_bytes + 64 - r
        };
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(255); // 1 is not null
        }

    }
    */
}

impl PartialEq for BitData {
    fn eq(&self, other: &BitData) -> bool {
        if self.count != other.count {
            return false;
        }
        unsafe { libc::memcmp(self.ptr as *const libc::c_void, other.ptr as *const libc::c_void, self.count as usize) == 0 }
    }
}

/// Release the underlying memory when the current buffer goes out of scope
impl Drop for BitData {
    fn drop(&mut self) {
        memory::free_aligned(self.ptr);
    }
}