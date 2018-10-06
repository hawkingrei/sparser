#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

fn ffs(x: i32) -> i32 {
    let mut r: i32 = 1;
    let mut val = x;
    if (val == 0) {
        return val;
    }
    if ((val & 0xffff) == 0) {
        val >>= 16;
        r += 16;
    }
    if ((val & 0xff) == 0) {
        val >>= 8;
        r += 8;
    }
    if ((val & 0xf) == 0) {
        val >>= 4;
        r += 4;
    }
    if ((val & 3) == 0) {
        val >>= 2;
        r += 2;
    }
    if ((val & 1) == 0) {
        val >>= 1;
        r += 1;
    }
    return r;
}

/** Search for an 8-bit search string.
 *
 * @param reg the register filled with the search value
 * @param base the data to search. Should be at least 32 bytes long.
 *
 * @return the number of matches found.
 */
pub fn search_epi8(reg: __m256i, base: Vec<char>) -> u32 {
    let mut count = 0;
    unsafe {
        let val = _mm256_loadu_si256(base.as_ptr() as *const __m256i);
        let mut mask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(reg, val));
        while (mask != 0) {
            let index = ffs(mask) - 1;
            mask &= !(1 << index);
            count = count + 1;
        }
    }
    return count;
}

/** Search for an 16-bit search string.
 *
 * @param reg the register filled with the search value
 * @param base the data to search. Should be at least 32 bytes long.
 *
 * @return the number of matches found.
 */
pub fn search_epi16(reg: __m256i, base: Vec<char>) -> u32 {
    let mut count = 0;
    unsafe {
        let val = _mm256_loadu_si256(base.as_ptr() as *const __m256i);
        let mut mask = _mm256_movemask_epi8(_mm256_cmpeq_epi16(reg, val));
        mask &= 0x55555555;
        while (mask != 0) {
            let index = ffs(mask) - 1;
            mask &= !(1 << index);
            count = count + 1;
        }
    }
    return count;
}

/** Search for an 32-bit search string.
 *
 * @param reg the register filled with the search value
 * @param base the data to search. Should be at least 32 bytes long.
 *
 * @return the number of matches found.
 */
pub fn search_epi32(reg: __m256i, base: Vec<char>) -> u32 {
    let mut count = 0;
    unsafe {
        let val = _mm256_loadu_si256(base.as_ptr() as *const __m256i);
        let mut mask = _mm256_movemask_epi8(_mm256_cmpeq_epi32(reg, val));
        mask &= 0x11111111;
        while (mask != 0) {
            let index = ffs(mask) - 1;
            mask &= !(1 << index);
            count = count + 1;
        }
    }
    return count;
}
