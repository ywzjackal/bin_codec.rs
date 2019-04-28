use bin_codec::*;
use bin_codec_derive::{BinEncodeBe, BinDecodeBe, BinEncodeLe};
#[test]
fn test_no_bit_field() {
    #[derive(BinEncodeBe, BinDecodeBe)]
    struct Struct {
        a_field: i32,
        b_field: Option<i32>,
    }

    let s = Struct {
        a_field: 0x12345678_i32,
        b_field: Some(0x11223344),
    };

    let mut target = [0u8; 8];
    s.encode(&mut target, &mut ());
    assert_eq!(&[0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33, 0x44], &target[..]);

    let s = Struct {
        a_field: 0x12345678_i32,
        b_field: None,
    };

    let mut target = [0u8; 4];
    s.encode(&mut target, &mut ());
    assert_eq!(&[0x12, 0x34, 0x56, 0x78], &target[..]);
}

#[test]
fn test_has_bit_field() {
    #[derive(BinEncodeBe, BinEncodeLe)]
    struct Struct<T> where T: EncodeBe + EncodeLe {
        #[bin(bits=24)]
        a_field: T,
        #[bin(bits=16)]
        b_field: T,
    }

    let s = Struct::<_> {
        a_field: 0x345678_i32,
        b_field: 0x3344,
    };

    let mut target = [0u8; 5];
    EncodeBe::encode(&s, &mut target, &mut ());
    assert_eq!(&[0x34, 0x56, 0x78, 0x33, 0x44], &target[..]);

    let mut target = [0u8; 5];
    EncodeLe::encode(&s, &mut target, &mut ());
    println!("{:#02X?}", target);
    assert_eq!(&[0x78, 0x56, 0x34, 0x44, 0x33], &target[..]);
}

#[test]
fn test_vec_field() {
    #[derive(BinEncodeBe, BinEncodeLe, Clone)]
    struct Struct<T> where T: EncodeBe + EncodeLe {
        #[bin(bits=24)]
        a_field: T,
        #[bin(bits=16)]
        b_field: T,
    }

    let s = Struct::<_> {
        a_field: 0x345678_i32,
        b_field: 0x3344,
    };

    #[derive(BinEncodeBe, BinEncodeLe)]
    struct StructVec<T> where T: EncodeBe + EncodeLe {
        vec: Vec<Struct<T>>,
    }

    let vec = StructVec {
        vec: vec![s.clone(), s]
    };

    let mut target = [0u8; 10];
    EncodeBe::encode(&vec, &mut target, &mut ());
    assert_eq!(&[0x34, 0x56, 0x78, 0x33, 0x44, 0x34, 0x56, 0x78, 0x33, 0x44], &target[..]);

    let mut target = [0u8; 10];
    EncodeLe::encode(&vec, &mut target, &mut ());
    assert_eq!(&[0x78, 0x56, 0x34, 0x44, 0x33, 0x78, 0x56, 0x34, 0x44, 0x33], &target[..]);
}