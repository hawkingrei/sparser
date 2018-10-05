use utils::bit_util;
use utils::buffer::Buffer;

#[derive(PartialEq, Clone, Debug)]
pub struct Bitmap {
    bits: Buffer,
}

impl Bitmap {
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
        Bitmap {
            bits: Buffer::from(&v[..]),
        }
    }

    pub fn len(&self) -> usize {
        self.bits.len()
    }

    pub fn is_set(&self, i: i64) -> bool {
        bit_util::get_bit(self.bits.data(), i)
    }

    pub fn reset(&mut self) {
        let len = self.bits.len();
        let mut v = Vec::with_capacity(len);
        for _ in 0..len {
            v.push(255); // 1 is not null
        }
        self.bits = Buffer::from(&v[..]);
    }
}

impl From<Buffer> for Bitmap {
    fn from(buf: Buffer) -> Self {
        Self { bits: buf }
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
        let bitmap = Bitmap::from(Buffer::from([0b01001010]));
        assert_eq!(false, bitmap.is_set(0));
        assert_eq!(true, bitmap.is_set(1));
        assert_eq!(false, bitmap.is_set(2));
        assert_eq!(true, bitmap.is_set(3));
        assert_eq!(false, bitmap.is_set(4));
        assert_eq!(false, bitmap.is_set(5));
        assert_eq!(true, bitmap.is_set(6));
        assert_eq!(false, bitmap.is_set(7));
    }
}
