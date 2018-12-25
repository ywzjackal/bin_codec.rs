use bin_codec::*;
use bin_codec_derive::{BinEncode, BinDecode};
#[test]
fn test_no_bit_field() {
    #[derive(BinEncode, BinDecode)]
    struct Struct {
        a_field: i32,
        b_field: Option<i32>,
    }

    let s = Struct {
        a_field: 0x12345678_i32,
        b_field: Some(0x11223344),
    };

    let mut target = [0u8; 8];
    assert_eq!(64, s.encode_be(&mut target, 0, &mut Context::default()).unwrap());
    assert_eq!(&[0x12, 0x34, 0x56, 0x78, 0x11, 0x22, 0x33, 0x44], &target[..]);

    let s = Struct {
        a_field: 0x12345678_i32,
        b_field: None,
    };

    let mut target = [0u8; 4];
    assert_eq!(32, s.encode_be(&mut target, 0, &mut Context::default()).unwrap());
    assert_eq!(&[0x12, 0x34, 0x56, 0x78], &target[..]);
}

#[test]
fn test_has_bit_field() {
    #[derive(BinEncode)]
    struct Struct<T> where T: Encode {
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
    s.encode_be(&mut target, 0, &mut Context::default()).unwrap();
    assert_eq!(&[0x34, 0x56, 0x78, 0x33, 0x44], &target[..]);

    let mut target = [0u8; 5];
    s.encode_le(&mut target, 0, &mut Context::default()).unwrap();
    println!("{:#02X?}", target);
    assert_eq!(&[0x78, 0x56, 0x34, 0x44, 0x33], &target[..]);
}

#[test]
fn test_vec_field() {
    #[derive(BinEncode, Clone)]
    struct Struct<T> where T: Encode {
        #[bin(bits=24)]
        a_field: T,
        #[bin(bits=16)]
        b_field: T,
    }

    let s = Struct::<_> {
        a_field: 0x345678_i32,
        b_field: 0x3344,
    };

    #[derive(BinEncode)]
    struct StructVec<T> where T: Encode {
        vec: Vec<Struct<T>>,
    }

    let vec = StructVec {
        vec: vec![s.clone(), s]
    };

    let mut target = [0u8; 10];
    vec.encode_be(&mut target, 0, &mut Context::default()).unwrap();
    assert_eq!(&[0x34, 0x56, 0x78, 0x33, 0x44, 0x34, 0x56, 0x78, 0x33, 0x44], &target[..]);

    let mut target = [0u8; 10];
    vec.encode_le(&mut target, 0, &mut Context::default()).unwrap();
    assert_eq!(&[0x78, 0x56, 0x34, 0x44, 0x33, 0x78, 0x56, 0x34, 0x44, 0x33], &target[..]);
}