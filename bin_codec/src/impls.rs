use crate::*;
use std::mem::size_of;
macro_rules! check_eob {
    ($target:expr, $offset:expr, $bits: expr) => {
        if $target < $offset || $target - $offset < $bits {
            return Err(Error::UnexpectEndOfBuff);
        }
    };
}
macro_rules! impl_number {
    (
        $($ty:ty),*
    ) => {
        $(
            impl EncodeBe for $ty {
                #[inline(always)]
                fn encode_offset<T>(&self, target: &mut [u8], _ctx: &mut T, offset: &mut usize, bits: usize) {
                    let v = self.to_be();
                    let size = size_of::<$ty>() * 8;
                    unsafe {
                        crate::utils::bit_copy(target.as_mut_ptr(), offset, &v as *const $ty as *const u8, size - bits, bits);
                        // let target_ptr = target.as_mut_ptr();
                        // let v = &v as *const $ty as *const u8;
                        // let mut v_start = size - bits;
                        // bit_copy!(target_ptr, offset, v, v_start, bits, u8);
                    }
                }

                #[inline(always)]
                fn bits_with_user_define(&self, bits: Option<usize>) -> usize { bits.unwrap_or(size_of::<$ty>() * 8) }
            }

            impl EncodeLe for $ty {
                #[inline(always)]
                fn encode_offset<T>(&self, target: &mut [u8], _ctx: &mut T, offset: &mut usize, bits: usize) {
                    // let size = size_of::<$ty>() * 8;
                    let v = self.to_le();
                    unsafe {
                        crate::utils::bit_copy(target.as_mut_ptr(), offset, &v as *const $ty as *const u8, 0, bits);
                    }
                }

                #[inline(always)]
                fn bits_with_user_define(&self, bits: Option<usize>) -> usize { bits.unwrap_or(size_of::<$ty>() * 8) }
            }

            impl DecodeBe for $ty {
                type Context = ();
                #[inline(always)]
                fn decode_offset(data: &[u8], offset: &mut usize, _: &mut ShouldDecode, _: &mut Self::Context, bits: usize) -> Result<Self> {
                    let size = size_of::<$ty>() * 8;
                    let mut v: $ty = 0;
                    println!(">> {} {} {}", data.len(), *offset, bits);
                    check_eob!(data.len() * 8, *offset, std::mem::size_of::<$ty>());
                    unsafe {
                        crate::utils::bit_copy(&mut v as *mut $ty as *mut u8, &mut (size - bits), data.as_ptr(), *offset, bits);
                    }
                    *offset += bits;
                    Ok(v.to_be())
                }
                #[inline(always)]
                fn default_bits() -> usize {
                    size_of::<$ty>() * 8
                }
            }

            impl DecodeLe for $ty {
                type Context = ();
                #[inline(always)]
                fn decode_offset(data: &[u8], offset: &mut usize, _: &mut ShouldDecode, _: &mut Self::Context, bits: usize) -> Result<Self> {
                    check_eob!(data.len() * 8, *offset, std::mem::size_of::<$ty>());
                    let mut v: $ty = 0;
                    unsafe {
                        crate::utils::bit_copy(&mut v as *mut $ty as *mut u8, &mut 0, data.as_ptr(), *offset, bits);
                    }
                    *offset += bits;
                    Ok(v.to_le())
                }
                #[inline(always)]
                fn default_bits() -> usize {
                    size_of::<$ty>() * 8
                }
            }
        )*
    }
}

macro_rules! impl_float {
    (
        $($ty:ty),*
    ) => {
        $(
            impl EncodeBe for $ty {
                #[inline(always)]
                fn encode_offset<T>(&self, target: &mut [u8], _ctx: &mut T, offset: &mut usize, _bits: usize) {
                    let size = size_of::<$ty>() * 8;
                    unsafe {
                        crate::utils::bit_copy(target.as_mut_ptr(), offset, self as *const $ty as *const u8, 0, size);
                    }
                }

                #[inline(always)]
                fn bits_with_user_define(&self, bits: Option<usize>) -> usize { bits.unwrap_or(size_of::<$ty>() * 8) }
            }

            impl EncodeLe for $ty {
                #[inline(always)]
                fn encode_offset<T>(&self, target: &mut [u8], _ctx: &mut T, offset: &mut usize, _bits: usize) {
                    let size = size_of::<$ty>() * 8;
                    unsafe {
                        crate::utils::bit_copy(target.as_mut_ptr(), offset, self as *const $ty as *const u8, 0, size);
                    }
                }

                #[inline(always)]
                fn bits_with_user_define(&self, bits: Option<usize>) -> usize { bits.unwrap_or(size_of::<$ty>() * 8) }
            }

            impl DecodeBe for $ty {
                type Context = ();
                #[inline(always)]
                fn decode_offset(data: &[u8], offset: &mut usize, _: &mut ShouldDecode, _: &mut Self::Context, _bits: usize) -> Result<Self> {
                    check_eob!(data.len() * 8, *offset, std::mem::size_of::<$ty>());
                    let size = size_of::<$ty>() * 8;
                    let mut v: $ty = 0.;
                    unsafe {
                        crate::utils::bit_copy(&mut v as *mut $ty as *mut u8, &mut 0, data.as_ptr(), *offset, size);
                    }
                    *offset += size;
                    Ok(v)
                }
                #[inline(always)]
                fn default_bits() -> usize {
                    size_of::<$ty>() * 8
                }
            }

            impl DecodeLe for $ty {
                type Context = ();
                #[inline(always)]
                fn decode_offset(data: &[u8], offset: &mut usize, _: &mut ShouldDecode, _: &mut Self::Context, _bits: usize) -> Result<Self> {
                    check_eob!(data.len() * 8, *offset, std::mem::size_of::<$ty>());
                    let size = size_of::<$ty>() * 8;
                    let mut v: $ty = 0.;
                    unsafe {
                        crate::utils::bit_copy(&mut v as *mut $ty as *mut u8, &mut 0, data.as_ptr(), *offset, size);
                    }
                    *offset += size;
                    Ok(v)
                }
                #[inline(always)]
                fn default_bits() -> usize {
                    size_of::<$ty>() * 8
                }
            }
        )*
    }
}

impl_number!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
impl_float!(f32, f64);

impl EncodeBe for bool {
    #[inline(always)]
    fn encode_offset<T>(&self, target: &mut [u8], _ctx: &mut T, offset: &mut usize, bits: usize) {
        // assert_eq!(bits, 1, "`bits` of bool type must be 1");
        unsafe {
            let v = self;
            crate::utils::bit_copy(
                target.as_mut_ptr(),
                offset,
                v as *const bool as *const u8,
                7,
                1,
            );
        }
    }
    fn bits_with_user_define(&self, _bits: Option<usize>) -> usize {
        1
    }
}

impl EncodeLe for bool {
    #[inline(always)]
    fn encode_offset<T>(&self, target: &mut [u8], _ctx: &mut T, offset: &mut usize, bits: usize) {
        // assert_eq!(bits, 1, "`bits` of bool type must be 1");
        unsafe {
            let v = self;
            crate::utils::bit_copy(
                target.as_mut_ptr(),
                offset,
                v as *const bool as *const u8,
                7,
                1,
            );
        }
    }
    fn bits_with_user_define(&self, _bits: Option<usize>) -> usize {
        1
    }
}

impl DecodeBe for bool {
    type Context = ();
    #[inline(always)]
    fn decode_offset(
        data: &[u8],
        offset: &mut usize,
        _: &mut ShouldDecode,
        _: &mut Self::Context,
        _bits: usize
    ) -> Result<Self> {
        check_eob!(data.len() * 8, *offset, 1);
        let size = 1;
        let mut v = false;
        unsafe {
            crate::utils::bit_copy(
                &mut v as *mut bool as *mut u8,
                &mut 7,
                data.as_ptr(),
                *offset,
                size,
            );
        }
        *offset += size;
        Ok(v)
    }
    #[inline(always)]
    fn default_bits() -> usize {
        1
    }
}

impl DecodeLe for bool {
    type Context = ();
    #[inline(always)]
    fn decode_offset(
        data: &[u8],
        offset: &mut usize,
        _: &mut ShouldDecode,
        _: &mut Self::Context,
        _bits: usize
    ) -> Result<Self> {
        check_eob!(data.len() * 8, *offset, 1);
        let size = 1;
        let mut v = false;
        unsafe {
            crate::utils::bit_copy(
                &mut v as *mut bool as *mut u8,
                &mut 7,
                data.as_ptr(),
                *offset,
                size,
            );
        }
        *offset += size;
        Ok(v)
    }
    #[inline(always)]
    fn default_bits() -> usize {
        1
    }
}

impl<E: EncodeBe> EncodeBe for Vec<E> {
    #[inline(always)]
    fn encode_offset<T>(&self, target: &mut [u8], ctx: &mut T, offset: &mut usize, bits: usize) {
        for i in self {
            i.encode_offset(target, ctx, offset, bits);
        }
    }
    #[inline(always)]
    fn bits_with_user_define(&self, bits: Option<usize>) -> usize {
        self.iter().map(|i| i.bits_with_user_define(bits)).sum()
    }
}

impl<E: EncodeLe> EncodeLe for Vec<E> {
    #[inline(always)]
    fn encode_offset<T>(&self, target: &mut [u8], ctx: &mut T, offset: &mut usize, bits: usize) {
        for i in self {
            i.encode_offset(target, ctx, offset, bits);
        }
    }
    #[inline(always)]
    fn bits_with_user_define(&self, bits: Option<usize>) -> usize {
        self.iter().map(|i| i.bits_with_user_define(bits)).sum()
    }
}

impl<E: EncodeBe> EncodeBe for Option<E> {
    #[inline(always)]
    fn encode_offset<T>(&self, target: &mut [u8], ctx: &mut T, offset: &mut usize, bits: usize) {
        for i in self {
            i.encode_offset(target, ctx, offset, bits);
        }
    }
    #[inline(always)]
    fn bits_with_user_define(&self, bits: Option<usize>) -> usize {
        self.iter().map(|i| i.bits_with_user_define(bits)).sum()
    }
}

impl<E: EncodeLe> EncodeLe for Option<E> {
    #[inline(always)]
    fn encode_offset<T>(&self, target: &mut [u8], ctx: &mut T, offset: &mut usize, bits: usize) {
        for i in self {
            i.encode_offset(target, ctx, offset, bits);
        }
    }
    #[inline(always)]
    fn bits_with_user_define(&self, bits: Option<usize>) -> usize {
        self.iter().map(|i| i.bits_with_user_define(bits)).sum()
    }
}

impl<E: DecodeBe> DecodeBe for Vec<E> {
    type Context = E::Context;
    #[inline(always)]
    fn decode_offset(
        data: &[u8],
        offset: &mut usize,
        sd: &mut ShouldDecode,
        ctx: &mut Self::Context,
        bits: usize,
    ) -> Result<Self> {
        match sd {
            ShouldDecode::Count(count) => {
                let mut rt = Vec::with_capacity(*count);
                for _ in 0..*count {
                    let i = DecodeBe::decode_offset(data, offset, sd, ctx, bits)?;
                    rt.push(i);
                }
                Ok(rt)
            }
            ShouldDecode::HasNext(_) => {
                let mut rt = Vec::new();
                while let ShouldDecode::HasNext(true) = sd {
                    let i = DecodeBe::decode_offset(data, offset, sd, ctx, bits)?;
                    rt.push(i);
                }
                Ok(rt)
            }
            _ => panic!("must set `count` or `has_next` on `Vec` field"),
        }
    }
    #[inline(always)]
    fn default_bits() -> usize {
        E::default_bits()
    }
}

impl<E: DecodeLe> DecodeLe for Vec<E> {
    type Context = E::Context;
    #[inline(always)]
    fn decode_offset(
        data: &[u8],
        offset: &mut usize,
        sd: &mut ShouldDecode,
        ctx: &mut Self::Context,
        bits: usize,
    ) -> Result<Self> {
        match sd {
            ShouldDecode::Count(count) => {
                let mut rt = Vec::with_capacity(*count);
                for _ in 0..*count {
                    let i = DecodeLe::decode_offset(data, offset, sd, ctx, bits)?;
                    rt.push(i);
                }
                Ok(rt)
            }
            ShouldDecode::HasNext(_) => {
                let mut rt = Vec::new();
                while let ShouldDecode::HasNext(true) = sd {
                    let i = DecodeLe::decode_offset(data, offset, sd, ctx, bits)?;
                    rt.push(i);
                }
                Ok(rt)
            }
            _ => panic!("must set `count` or `has_next` on `Vec` field"),
        }
    }
    #[inline(always)]
    fn default_bits() -> usize {
        E::default_bits()
    }
}

impl<E: DecodeBe> DecodeBe for Option<E> {
    type Context = E::Context;
    #[inline(always)]
    fn decode_offset(
        data: &[u8],
        offset: &mut usize,
        sd: &mut ShouldDecode,
        ctx: &mut Self::Context,
        bits: usize,
    ) -> Result<Self> {
        match sd {
            ShouldDecode::IsSome(true) => Ok(Some(DecodeBe::decode_offset(data, offset, sd, ctx, bits)?)),
            ShouldDecode::IsSome(false) => Ok(None),
            _ => panic!("must set `is_some` on `Option` field"),
        }
    }
    #[inline(always)]
    fn default_bits() -> usize {
        E::default_bits()
    }
}

impl<E: DecodeLe> DecodeLe for Option<E> {
    type Context = E::Context;
    #[inline(always)]
    fn decode_offset(
        data: &[u8],
        offset: &mut usize,
        sd: &mut ShouldDecode,
        ctx: &mut Self::Context,
        bits: usize,
    ) -> Result<Self> {
        match sd {
            ShouldDecode::IsSome(true) => Ok(Some(DecodeLe::decode_offset(data, offset, sd, ctx, bits)?)),
            ShouldDecode::IsSome(false) => Ok(None),
            _ => panic!("must set `is_some` on `Option` field"),
        }
    }
    #[inline(always)]
    fn default_bits() -> usize {
        E::default_bits()
    }
}

#[test]
fn test_encode_le_number() {
    let mut data = [0u8; 100];
    let mut offset = 0;
    EncodeLe::encode_offset(&0x11223344i32, &mut data, &mut (), &mut offset, 24);
    assert_eq!(&data[..3], &[0x44, 0x33, 0x22], "{:X?}", &data[..3]);
    assert_eq!(offset, 24);
}

#[test]
fn test_decode_le_number() {
    let data = [0x44, 0x33, 0x22];
    let mut offset = 0;
    let value: i32 = DecodeLe::decode_offset(&data, &mut offset, &mut ShouldDecode::None, &mut (), 24).unwrap();
    assert_eq!(value, 0x223344);
    assert_eq!(offset, 24);
}
