use crate::error;
use crate::Context;

/// Encode trait
pub trait Encode {
//    fn bit_size(&self) -> usize;
//    fn byte_size(&self) -> usize {
//        (self.bit_size() + 7) / 8
//    }
    fn encode_be(&self, target: &mut [u8], target_start: usize, ctx: &mut Context) -> error::Result<usize>;
    fn encode_le(&self, target: &mut [u8], target_start: usize, ctx: &mut Context) -> error::Result<usize>;
}

/// Decode trait
pub trait Decode: Sized {
//    fn bit_size() -> usize;
//    fn byte_size() -> usize {
//        (Self::bit_size() + 7) / 8
//    }
    fn decode_le(data: &[u8], data_start_bit: usize, ctx: &mut Context) -> error::Result<(Self, usize)>;
    fn decode_be(data: &[u8], data_start_bit: usize, ctx: &mut Context) -> error::Result<(Self, usize)>;
}