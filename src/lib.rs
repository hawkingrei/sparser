extern crate libc;
extern crate rand;
pub mod bitmap;
pub mod sparser_kernels;
pub mod utils;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
