use bin_codec::*;
use bin_codec_derive::{BinEncodeBe, BinDecodeBe};
#[test]
fn test_unnamed() {
    #[derive(BinDecodeBe, BinEncodeBe)]
    struct Struct(u8, u8);
    let (s, size) = Struct::decode(&[0,1], &mut ()).unwrap();
    assert_eq!(s.0, 0);
    assert_eq!(s.1, 1);
    assert_eq!(16, size);
    //
    let mut target = [0u8; 2];
    s.encode(&mut target, &mut ());
    assert_eq!(&[0,1], &target);
    assert_eq!(s.bits(), 16);
}

#[test]
fn test_unnamed_bits() {
    #[derive(BinDecodeBe, BinEncodeBe)]
    struct Struct(#[bin(bits(4))]u8, #[bin(bits(4))]u8);
    let (s, size) = Struct::decode(&[0x12,34], &mut ()).unwrap();
    assert_eq!(s.0, 1);
    assert_eq!(s.1, 2);
    assert_eq!(8, size);
    //
    let mut target = [0u8; 2];
    s.encode(&mut target, &mut ());
    assert_eq!(&[0x12,0], &target);
    assert_eq!(s.bits(), 8); 
}