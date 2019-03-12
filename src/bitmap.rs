const WORDSZ: usize = 64;

use std::arch::x86_64::_popcnt64;

#[derive(Copy, Clone)]
pub struct Bitmap {
    bits: [i64; 1024],
    count: usize,
    words: usize,
}

impl Default for Bitmap {
    #[inline(always)]
    fn default() -> Self {
        let mut words = (1024 / WORDSZ) + 1;
        Bitmap {
            bits: [0; 1024],
            count: 0,
            words: words,
        }
    }
}

impl Bitmap {
    pub fn reset(&mut self) {
        self.bits = [0; 1024];
        self.count = 0;
    }

    pub fn set(&mut self, index: usize) {
        let word = index / WORDSZ;
        let shift = word % WORDSZ;
        self.bits[word] |= (0x1 << shift);
        self.count += 1;
    }

    pub fn unset(&mut self, index: usize) {
        let word = index / WORDSZ;
        let shift = word % WORDSZ;
        self.bits[word] &= !(0x1 << shift);
        self.count -= 1;
    }

    pub fn and(&self, bm: Bitmap) -> Self {
        let mut result: Bitmap = Default::default();
        unsafe {
            for i in 0..self.words {
                result.bits[i] = self.bits[i] & bm.bits[i];
                result.count += _popcnt64(result.bits[i]) as usize;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitmap_length() {
        assert_eq!(64, Bitmap::new(63 * 8).len());
        assert_eq!(64, Bitmap::new(64 * 8).len());
        assert_eq!(128, Bitmap::new(65 * 8).len());
    }

    #[test]
    fn test_bitmap_is_set() {
        let mut bitmap = Bitmap::from(Buffer::from([0b01001010]));
        assert_eq!(false, bitmap.is_set(0));
        assert_eq!(true, bitmap.is_set(1));
        assert_eq!(false, bitmap.is_set(2));
        assert_eq!(true, bitmap.is_set(3));
        assert_eq!(false, bitmap.is_set(4));
        assert_eq!(false, bitmap.is_set(5));
        assert_eq!(true, bitmap.is_set(6));
        assert_eq!(false, bitmap.is_set(7));
        unsafe {
            bitmap.set(7);
            assert_eq!(true, bitmap.is_set(7));
            bitmap.unset(7);
            assert_eq!(false, bitmap.is_set(7));
        }
    }
}
