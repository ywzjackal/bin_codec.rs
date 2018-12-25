use bin_codec::*;
use bin_codec_derive::BinEncode;
#[test]
fn test_attr_value_int() {
    #[derive(BinEncode)]
    struct Struct<T> where T: Encode {
        #[bin(bits=4, value=0xff)]
        a_field: T,
    }

    let s = Struct::<_> {
        a_field: 0x12345678_i32,
    };

    let mut target = [0u8; 1];
    s.encode_be(&mut target, 0, &mut Context::default()).unwrap();
    assert_eq!(&[0xf0], &target[..], "{:#02X?} != {:#02X?}", &[0xf0], &target[..]);
}
#[test]
fn test_attr_value_expr() {
    fn get_u8() -> u8 {
        0xa0
    }
    #[derive(BinEncode)]
    struct Struct {
        #[bin(value="get_u8()")]
        a_field: u8,
        #[bin(value="self.a_field + 1")]
        b_field: u8,
    }

    let s = Struct {
        a_field: 1,
        b_field: 0,
    };

    let mut target = [0u8; 2];;
    s.encode_be(&mut target, 0, &mut Context::default()).unwrap();
    assert_eq!(&[0xA0, 2], &target[..]);
}