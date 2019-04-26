use bin_codec::*;
use bin_codec_derive::{BinEncode, BinDecode};
#[test]
fn test_count() {
    #[derive(BinDecode)]
    struct Struct {
        a_field: u8,
        #[bin(count(a_field as usize))]
        b_field: Vec<u8>,
    }
    let target = [0];
    let (s, size) = Struct::decode_be(&target, 0, &mut Context::default()).unwrap();
    assert_eq!(size, 8);
    assert_eq!(s.a_field, 0);
    assert_eq!(true, s.b_field.is_empty());
    //
    let target = [2, 0, 1];
    let (s, size) = Struct::decode_be(&target, 0, &mut Context::default()).unwrap();
    assert_eq!(size, 24);
    assert_eq!(s.a_field, 2);
    assert_eq!(&[0, 1], s.b_field.as_slice());
}

#[test]
fn test_has_next() {
    #[derive(BinDecode, Debug)]
    #[bin(has_next=(value != 0))]
    struct Item {
        value: u8,
    }
    #[derive(BinDecode, Debug)]
    struct Struct {
        a_field: u8,
        #[bin(is_some(a_field == 0))]
        b_field: Vec<Item>,
    }
    let (s, _size) = Struct::decode_be(&[0, 1, 2, 0, 1], 0, &mut Context::default()).unwrap();
    assert_eq!(0, s.a_field);
    assert_eq!(3, s.b_field.len());
}

#[test]
fn test_bits_on_vec() {
    #[derive(BinDecode, BinEncode)]
    struct S {
        count: u8,
        #[bin(bits=16, count=count as usize)]
        values: Vec<u32>,
    }

    let (s, size) = S::decode_be(&[3,1,2,3,4,5,6], 0, &mut Context::default()).unwrap();
    assert_eq!(size, 56);
    assert_eq!(s.count, 3);
    assert_eq!(&[0x0102, 0x0304, 0x0506], s.values.as_slice());
    //
    let mut target = [0u8; 7];
    let size = s.encode_be(&mut target, 0, &mut Context::default()).unwrap();
    assert_eq!(size, 56);
    assert_eq!(&[3,1,2,3,4,5,6], &target);
    //
    let (s, size) = S::decode_le(&[3,1,2,3,4,5,6], 0, &mut Context::default()).unwrap();
    assert_eq!(size, 56);
    assert_eq!(s.count, 3);
    assert_eq!(&[0x201, 0x403, 0x605], s.values.as_slice());
    //
    let mut target = [0u8; 7];
    let size = s.encode_le(&mut target, 0, &mut Context::default()).unwrap();
    assert_eq!(size, 56);
    assert_eq!(&[3,1,2,3,4,5,6], &target);
}