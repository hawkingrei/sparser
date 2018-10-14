#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

#[inline]
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

#[test]
fn test_ffs() {
    assert!(ffs(1) == 1);
    assert!(ffs(16) == 5);
    assert!(ffs(64) == 7);
}

/** Search for an 8-bit search string.
 *
 * @param reg the register filled with the search value
 * @param base the data to search. Should be at least 32 bytes long.
 *
 * @return the number of matches found.
 */
#[inline]
pub fn search_epi8(reg: __m256i, base: __m256i) -> u32 {
    let mut count = 0;
    unsafe {
        let mut mask = _mm256_movemask_epi8(_mm256_cmpeq_epi8(reg, base));
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
#[inline]
pub fn search_epi16(reg: __m256i, base: __m256i) -> u32 {
    let mut count = 0;
    unsafe {
        let mut mask = _mm256_movemask_epi8(_mm256_cmpeq_epi16(reg, base));
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
#[inline]
pub fn search_epi32(reg: __m256i, base: __m256i) -> u32 {
    let mut count = 0;
    unsafe {
        let mut mask = _mm256_movemask_epi8(_mm256_cmpeq_epi32(reg, base));
        mask = mask & 0x11111111;
        while (mask != 0) {
            let index = ffs(mask) - 1;
            mask &= !(1 << index);
            count = count + 1;
        }
    }
    return count;
}

#[cfg(test)]
mod test {
    use sparser_kernels::search_epi16;
    use sparser_kernels::search_epi32;
    use sparser_kernels::search_epi8;
    use std::arch::x86_64::__m256i;
    use std::arch::x86_64::_mm256_loadu_si256;

    #[test]
    fn test_search_epi32() {
        unsafe {
            let load_bytes: [u8; 32] = [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32,
            ];
            let lb_ptr = load_bytes.as_ptr();
            let req: __m256i = _mm256_loadu_si256(lb_ptr as *const __m256i);
            let base: [u8; 32] = [
                9, 10, 11, 12, 9, 10, 11, 12, 9, 10, 11, 12, 13, 14, 15, 16, 9, 10, 11, 12, 9, 10,
                11, 12, 9, 10, 11, 12, 9, 10, 11, 12,
            ];
            let base_req: __m256i = _mm256_loadu_si256(base.as_ptr() as *const __m256i);
            let result = search_epi32(req, base_req);
            assert!(result == 2);
        }
    }

    #[test]
    fn test_search_epi8() {
        unsafe {
            let load_bytes: [u8; 32] = [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32,
            ];
            let lb_ptr = load_bytes.as_ptr();
            let req: __m256i = _mm256_loadu_si256(lb_ptr as *const __m256i);
            let base: [u8; 32] = [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 32,
            ];
            let base_req: __m256i = _mm256_loadu_si256(base.as_ptr() as *const __m256i);
            let result = search_epi8(req, base_req);
            assert!(result == 3);
        }
    }

    #[test]
    fn test_search_epi16() {
        unsafe {
            let load_bytes: [u8; 32] = [
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
                24, 25, 26, 27, 28, 29, 30, 31, 32,
            ];
            let lb_ptr = load_bytes.as_ptr();
            let req: __m256i = _mm256_loadu_si256(lb_ptr as *const __m256i);
            let base: [u8; 32] = [
                1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 11, 12, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 32, 32,
            ];
            let base_req: __m256i = _mm256_loadu_si256(base.as_ptr() as *const __m256i);
            let result = search_epi16(req, base_req);
            assert!(result == 2);
        }
    }
}
