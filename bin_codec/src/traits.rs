use crate::error;
pub trait EncodeBe {
    fn encode<T>(&self, target: &mut [u8], ctx: &mut T) {
        assert!(target.len() >= (self.bits() + 7) / 8);
        Self::encode_offset(self, target, ctx, &mut 0, self.bits());
    }
    fn encode_offset<T>(&self, target: &mut [u8], ctx: &mut T, offset: &mut usize, bits: usize);
    fn bits(&self) -> usize;
}

pub trait EncodeLe {
    fn encode<T>(&self, target: &mut [u8], ctx: &mut T) {
        assert!(target.len() >= (self.bits() + 7) / 8);
        Self::encode_offset(self, target, ctx, &mut 0, self.bits());
    }
    fn encode_offset<T>(&self, target: &mut [u8], ctx: &mut T, offset: &mut usize, bits: usize);
    fn bits(&self) -> usize;
}

pub enum ShouldDecode {
    HasNext(bool),
    Count(usize),
    IsSome(bool),
    None,
}

pub trait DecodeBe: Sized {
    fn decode<T>(data: &[u8], ctx: &mut T) -> error::Result<(Self, usize)> {
        let mut offset = 0;
        Ok((Self::decode_offset(data, &mut offset, &mut ShouldDecode::None, ctx, 0)?, offset))
    }
    fn decode_offset<T>(data: &[u8], offset: &mut usize, sd: &mut ShouldDecode, ctx: &mut T, bits: usize) -> error::Result<Self>;
}

pub trait DecodeLe: Sized {
    fn decode<T>(data: &[u8], ctx: &mut T) -> error::Result<(Self, usize)> {
        let mut offset = 0;
        Ok((Self::decode_offset(data, &mut offset, &mut ShouldDecode::None, ctx, 0)?, offset))
    }
    fn decode_offset<T>(data: &[u8], offset: &mut usize, sd: &mut ShouldDecode, ctx: &mut T, bits: usize) -> error::Result<Self>;
}