use bin_codec::*;
use bin_codec_derive::{BinEncode, BinDecode};
#[test]
fn test_unnamed() {
    #[derive(BinDecode, BinEncode)]
    struct Struct(u8, u8);
    let (s, size) = Struct::decode_be(&[0,1], 0, &mut Context::default()).unwrap();
    assert_eq!(s.0, 0);
    assert_eq!(s.1, 1);
    assert_eq!(16, size);
    //
    let mut target = [0u8; 2];
    let size = s.encode_be(&mut target, 0, &mut Context::default()).unwrap();
    assert_eq!(&[0,1], &target);
    assert_eq!(size, 16);
}