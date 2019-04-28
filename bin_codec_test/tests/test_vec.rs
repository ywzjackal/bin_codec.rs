use bin_codec::*;
use bin_codec_derive::{BinEncodeBe, BinDecodeBe, BinDecodeLe, BinEncodeLe};
#[test]
fn test_count() {
    #[derive(BinDecodeBe)]
    struct Struct {
        a_field: u8,
        #[bin(count("a_field as usize"))]
        b_field: Vec<u8>,
    }
    let target = [0];
    let (s, size) = Struct::decode(&target, &mut ()).unwrap();
    assert_eq!(size, 8);
    assert_eq!(s.a_field, 0);
    assert_eq!(true, s.b_field.is_empty());
    //
    let target = [2, 0, 1];
    let (s, size) = Struct::decode(&target, &mut ()).unwrap();
    assert_eq!(size, 24);
    assert_eq!(s.a_field, 2);
    assert_eq!(&[0, 1], s.b_field.as_slice());
}

#[test]
fn test_has_next() {
    #[derive(BinDecodeBe, Debug)]
    #[bin(context("bool"))]
    #[bin(has_next("*ctx"))]
    struct Item {
        #[bin(after_de("*ctx = value != 0;"))]
        value: u8,
    }
    #[derive(BinDecodeBe, Debug)]
    #[bin(context("bool"))]
    struct Struct {
        a_field: u8,
        #[bin(before_de("*ctx = a_field == 0;"))]
        #[bin(has_next("*ctx"))]
        #[bin(context("ctx"))]
        b_field: Vec<Item>,
    }
    let mut has_next = false;
    let (s, _size) = Struct::decode(&[0, 1, 2, 0, 1], &mut has_next).unwrap();
    assert_eq!(0, s.a_field);
    assert_eq!(3, s.b_field.len());
}

#[test]
fn test_bits_on_vec() {
    #[derive(BinDecodeBe, BinDecodeLe, BinEncodeLe, BinEncodeBe)]
    struct S {
        count: u8,
        #[bin(bits(16), count("count as usize"))]
        values: Vec<u32>,
    }

    let (s, size) = <S as DecodeBe>::decode(&[3,1,2,3,4,5,6], &mut ()).unwrap();
    assert_eq!(s.count, 3);
    assert_eq!(&[0x0102, 0x0304, 0x0506], s.values.as_slice());
    assert_eq!(size, 56);
    assert_eq!(EncodeBe::bits(&s), size);
    //
    let mut target = [0u8; 7];
    EncodeBe::encode(&s, &mut target, &mut ());
    assert_eq!(&[3,1,2,3,4,5,6], &target);
    //
    let (s, size) = <S as DecodeLe>::decode(&[3,1,2,3,4,5,6], &mut ()).unwrap();
    assert_eq!(size, 56);
    assert_eq!(s.count, 3);
    assert_eq!(&[0x201, 0x403, 0x605], s.values.as_slice());
    //
    let mut target = [0u8; 7];
    EncodeLe::encode(&s, &mut target, &mut ());
    assert_eq!(EncodeLe::bits(&s), 56);
    assert_eq!(&[3,1,2,3,4,5,6], &target);
}