use crate::{Encode, Decode, Result, Context, Error};
use std::mem::size_of;
macro_rules! check_eob {
    ($real:expr, $need:expr, $offset: expr) => {
        if $real < $need {
            return Err(Error::UnexpectEob{ offset: $offset, expected: $need, real: $real });
        }
    }
}
macro_rules! impl_number {
    (
        $($ty:ty),*
    ) => {
        $(
            impl Encode for $ty {
//                fn bit_size(&self) -> usize { size_of::<Self>() * 8 }
                fn encode_be(&self, target: &mut [u8], target_start: usize, ctx: &mut Context) -> Result<usize> {
                    let origin_bit_size = size_of::<$ty>() * 8;
                    let bit_size = ctx.bit_size().unwrap_or(origin_bit_size);
                    check_eob!(target.len() * 8, target_start + bit_size, target_start); //, "encode target space too small:{} > ({} + {}) / 8", target.len(), target_start, bit_size);
                    let v = self.to_be();
                    unsafe {
                        crate::utils::bit_copy(target.as_mut_ptr(), target_start, &v as *const $ty as *const u8, origin_bit_size - bit_size, origin_bit_size);
                    }
                    Ok(bit_size)
                }
                fn encode_le(&self, target: &mut [u8], target_start: usize, ctx: &mut Context) -> Result<usize> {
                    let origin_bit_size = size_of::<$ty>() * 8;
                    let bit_size = ctx.bit_size().unwrap_or(origin_bit_size);
                    check_eob!(target.len() * 8, target_start + bit_size, target_start);//, "encode target space too small:{} > ({} + {}) / 8", target.len(), target_start, bit_size);
                    let v = self.to_le();
                    unsafe {
                        crate::utils::bit_copy(target.as_mut_ptr(), target_start, &v as *const $ty as *const u8, 0, bit_size);
                    }
                    Ok(bit_size)
                }
            }

            impl Decode for $ty {
//                fn bit_size() -> usize { size_of::<Self>() * 8 }
                fn decode_be(data: &[u8], data_start_bit: usize, ctx: &mut Context) -> Result<(Self, usize)> {
                    let bit_size = ctx.bit_size().unwrap_or(size_of::<$ty>() * 8);
                    check_eob!(data.len() * 8, data_start_bit + bit_size, data_start_bit);//, "decode data space too small:{} > {}", data.len(), (data_start_bit + bit_size) / 8);
                    let mut v: $ty = 0;
                    unsafe {
                        crate::utils::bit_copy(&mut v as *mut $ty as *mut u8, size_of::<$ty>() * 8 - bit_size, data.as_ptr(), data_start_bit, data_start_bit + bit_size);
                    }
                    Ok((v.to_be(), bit_size))
                }

                fn decode_le(data: &[u8], data_start_bit: usize, ctx: &mut Context) -> Result<(Self, usize)> {
                    let bit_size = ctx.bit_size().unwrap_or(size_of::<$ty>() * 8);
                    check_eob!(data.len() * 8, data_start_bit + bit_size, data_start_bit);//, "decode data space too small:{} > {}", data.len(), (data_start_bit + bit_size) / 8);
                    let mut v: $ty = 0;
                    unsafe {
                        crate::utils::bit_copy(&mut v as *mut $ty as *mut u8, 0, data.as_ptr(), data_start_bit, data_start_bit + bit_size);
                    }
                    Ok((v.to_le(), bit_size))
                }
            }
        )*
    }
}

impl_number!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

impl Encode for bool {

    fn encode_be(&self, target: &mut [u8], target_start: usize, ctx: &mut Context) -> Result<usize> {
        if *self {
            1u8.encode_be(target, target_start, ctx)
        } else {
            0u8.encode_be(target, target_start, ctx)
        }
    }
    fn encode_le(&self, target: &mut [u8], target_start: usize, ctx: &mut Context) -> Result<usize> {
        if *self {
            1u8.encode_le(target, target_start, ctx)
        } else {
            0u8.encode_le(target, target_start, ctx)
        }
    }
}

impl Decode for bool {

    fn decode_be(data: &[u8], data_start: usize, ctx: &mut Context) -> Result<(Self, usize)> {
        i32::decode_be(data, data_start, ctx).map(|(v, i)| (v != 0, i))
    }

    fn decode_le(data: &[u8], data_start: usize, ctx: &mut Context) -> Result<(Self, usize)> {
        i32::decode_le(data, data_start, ctx).map(|(v, i)| (v != 0, i))
    }
}

impl<E: Encode> Encode for Box<E> {

    fn encode_be(&self, target: &mut [u8], target_start: usize, ctx: &mut Context) -> Result<usize> {
        self.as_ref().encode_be(target, target_start, ctx)
    }
    fn encode_le(&self, target: &mut [u8], target_start: usize, ctx: &mut Context) -> Result<usize> {
        self.as_ref().encode_le(target, target_start, ctx)
    }
}

impl<D: Decode> Decode for Box<D> {

    fn decode_be(data: &[u8], data_start: usize, ctx: &mut Context) -> Result<(Self, usize)> {
        D::decode_be(data, data_start, ctx).map(|(v, i)| (Box::new(v), i))
    }

    fn decode_le(data: &[u8], data_start: usize, ctx: &mut Context) -> Result<(Self, usize)> {
        D::decode_le(data, data_start, ctx).map(|(v, i)| (Box::new(v), i))
    }
}

impl<E: Encode> Encode for Option<E> {

    fn encode_be(&self, target: &mut [u8], target_start: usize, ctx: &mut Context) -> Result<usize> {
        self.as_ref().map(|s| s.encode_be(target, target_start, ctx)).unwrap_or(Ok(0))
    }
    fn encode_le(&self, target: &mut [u8], target_start: usize, ctx: &mut Context) -> Result<usize> {
        self.as_ref().map(|s| s.encode_le(target, target_start, ctx)).unwrap_or(Ok(0))
    }
}

impl<D: Decode> Decode for Option<D> {

    fn decode_be(data: &[u8], data_start: usize, ctx: &mut Context) -> Result<(Self, usize)> {
        let is_some = ctx.is_some().expect("attribute `is_some` not set for `Option` field");
        if is_some {
            D::decode_be(data, data_start, ctx).map(|(d, i)| (Some(d), i))
        } else {
            Ok((None, 0))
        }
    }

    fn decode_le(data: &[u8], data_start: usize, ctx: &mut Context) -> Result<(Self, usize)> {
        let is_some = ctx.is_some().expect("attribute `is_some` not set for `Option` field");
        if is_some {
            D::decode_be(data, data_start, ctx).map(|(d, i)| (Some(d), i))
        } else {
            Ok((None, 0))
        }
    }
}

impl<E: Encode> Encode for Vec<E> {

    fn encode_be(&self, target: &mut [u8], mut target_start: usize, ctx: &mut Context) -> Result<usize> {
        let old_start = target_start;
        for e in self.iter() {
            target_start += e.encode_be(target, target_start, ctx)?;
        }
        Ok(target_start - old_start)
    }
    fn encode_le(&self, target: &mut [u8], mut target_start: usize, ctx: &mut Context) -> Result<usize> {
        let old_start = target_start;
        for e in self.iter() {
            target_start += e.encode_le(target, target_start, ctx)?;
        }
        Ok(target_start - old_start)
    }
}

impl<D: Decode> Decode for Vec<D> {

    fn decode_be(data: &[u8], mut data_start: usize, ctx: &mut Context) -> Result<(Self, usize)> {
        let data_start_old = data_start;
        match (ctx.count(), ctx.is_some()) {
            (Some(count), None) => {
                let mut rt: Vec<D> = Vec::with_capacity(count);
                for _ in 0..count {
                    println!("{:?}", ctx.bit_size());
                    let (e, size) = D::decode_be(data, data_start, ctx)?;
                    data_start += size;
                    rt.push(e);
                }
                Ok((rt, data_start - data_start_old))
            }
            (None, Some(mut has_next)) => {
                let mut rt: Vec<D> = Vec::new();
                while has_next {
                    ctx.set_has_next(None);
                    let (e, size) = D::decode_be(data, data_start, ctx)?;
                    data_start += size;
                    rt.push(e);
                    has_next = ctx.has_next().expect("attribute `has_next` must be set in vec element body");
                }
                Ok((rt, data_start - data_start_old))
            }
            (Some(_), Some(_)) => {
                panic!("attribute `count` and `is_some & has_next` ?")
            }
            (None, None) => {
                panic!("attribute `count` or `is_some & has_next` ?")
            }
        }
    }

    fn decode_le(data: &[u8], mut data_start: usize, ctx: &mut Context) -> Result<(Self, usize)> {
        let data_start_old = data_start;
        match (ctx.count(), ctx.has_next()) {
            (Some(count), None) => {
                let mut rt: Vec<D> = Vec::with_capacity(count);
                for _ in 0..count {
                    let (e, size) = D::decode_le(data, data_start, ctx)?;
                    data_start += size;
                    rt.push(e);
                }
                Ok((rt, data_start - data_start_old))
            }
            (None, Some(mut has_next)) => {
                let mut rt: Vec<D> = Vec::new();
                while has_next {
                    let (e, size) = D::decode_le(data, data_start, ctx)?;
                    data_start += size;
                    rt.push(e);
                    has_next = ctx.has_next().expect("attribute `has_next` must be set in vec element body");
                }
                Ok((rt, data_start - data_start_old))
            }
            (Some(_), Some(_)) => {
                panic!("attribute `count` and `has_next` ?")
            }
            (None, None) => {
                panic!("attribute `count` or `has_next` ?")
            }
        }
    }
}

#[test]
fn test_encode_number() {
    // test i32 be
    let mut target = [0u8; 4];
    0x12345678i32.encode_be(&mut target, 0, &mut Default::default()).unwrap();
    assert_eq!(&[0x12, 0x34, 0x56, 0x78], &target[..]);
    // test i32 le
    let mut target = [0u8; 4];
    0x12345678i32.encode_le(&mut target, 0, &mut Default::default()).unwrap();
    assert_eq!(&[0x78, 0x56, 0x34, 0x12], &target[..]);
}

#[test]
fn test_encode_box_number() {
    // test i32 be
    let mut target = [0u8; 4];
    0x12345678i32.encode_be(&mut target, 0, &mut Default::default()).unwrap();
    assert_eq!(&[0x12, 0x34, 0x56, 0x78], &target[..]);
    // test i32 le
    let mut target = [0u8; 4];
    Box::new(0x12345678i32).encode_le(&mut target, 0, &mut Default::default()).unwrap();
    assert_eq!(&[0x78, 0x56, 0x34, 0x12], &target[..]);
}

#[test]
fn test_decode_number() {
    let data = [0x12u8, 0x34, 0x56, 0x78];
    let mut ctx = Context::default();
    ctx.set_bit_size(Some(24));
    assert_eq!(0x12345678, i32::decode_be(&data[..], 0, &mut Default::default()).unwrap());
    assert_eq!(0x345678, i32::decode_be(&data[..], 8, &mut ctx).unwrap());
    assert_eq!(0x78563412, i32::decode_le(&data[..], 0, &mut Default::default()).unwrap());
    assert_eq!(0x785634, i32::decode_le(&data[..], 8, &mut ctx).unwrap());
}