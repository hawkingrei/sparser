#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;
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
        let mask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(reg, val));
        while (mask != 0) {
            break;
        }
    }
    return 0;
}
