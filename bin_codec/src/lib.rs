extern crate proc_macro;
mod error;
mod utils;
mod traits;
mod impls;

pub use self::error::*;
pub use self::traits::*;
pub use self::impls::*;
pub use self::utils::*;

// pub fn encode_to_slice_be<T: Encode>(src: &T, target: &mut [u8]) -> Result<usize> {
//     src.encode_be(target, 0, &mut Context::default()).map(|bits| (bits + 7) / 8)
// }

// pub fn encode_to_slice_le<T: Encode>(src: &T, target: &mut [u8]) -> Result<usize> {
//     src.encode_le(target, 0, &mut Context::default()).map(|bits| (bits + 7) / 8)
// }

// pub fn decode_from_slice_be<T: Decode>(src: &[u8]) -> Result<(T, usize)> {
//     T::decode_be(src, 0, &mut Context::default())
// }

// pub fn decode_from_slice_le<T: Decode>(src: &[u8]) -> Result<(T, usize)> {
//     T::decode_le(src, 0, &mut Context::default())
// }