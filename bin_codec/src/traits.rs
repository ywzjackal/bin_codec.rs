use crate::error;
pub trait EncodeBe {
    fn encode<T>(&self, target: &mut [u8], ctx: &mut T) {
        assert!(target.len() >= (self.bits() + 7) / 8);
        Self::encode_offset(self, target, ctx, &mut 0, self.bits());
    }
    fn encode_offset<T>(&self, target: &mut [u8], ctx: &mut T, offset: &mut usize, bits: usize);
    fn bits(&self) -> usize {
        Self::bits_with_user_define(self, None)
    }
    fn bits_with_user_define(&self, bits: Option<usize>) -> usize;
}

pub trait EncodeLe {
    fn encode<T>(&self, target: &mut [u8], ctx: &mut T) {
        assert!(target.len() >= (self.bits() + 7) / 8);
        Self::encode_offset(self, target, ctx, &mut 0, self.bits());
    }
    fn encode_offset<T>(&self, target: &mut [u8], ctx: &mut T, offset: &mut usize, bits: usize);
    fn bits(&self) -> usize {
        Self::bits_with_user_define(self, None)
    }
    fn bits_with_user_define(&self, bits: Option<usize>) -> usize;
}

pub enum ShouldDecode {
    HasNext(bool),
    Count(usize),
    IsSome(bool),
    None,
}

pub trait DecodeBe: Sized {
    type Context;
    fn decode(data: &[u8], ctx: &mut Self::Context) -> error::Result<(Self, usize)> {
        let mut offset = 0;
        Ok((Self::decode_offset(data, &mut offset, &mut ShouldDecode::None, ctx, 0)?, offset))
    }
    fn decode_offset(data: &[u8], offset: &mut usize, sd: &mut ShouldDecode, ctx: &mut Self::Context, bits: usize) -> error::Result<Self>;
    fn default_bits() -> usize;
}

pub trait DecodeLe: Sized {
    type Context;
    fn decode(data: &[u8], ctx: &mut Self::Context) -> error::Result<(Self, usize)> {
        let mut offset = 0;
        Ok((Self::decode_offset(data, &mut offset, &mut ShouldDecode::None, ctx, 0)?, offset))
    }
    fn decode_offset(data: &[u8], offset: &mut usize, sd: &mut ShouldDecode, ctx: &mut Self::Context, bits: usize) -> error::Result<Self>;
    fn default_bits() -> usize;
}