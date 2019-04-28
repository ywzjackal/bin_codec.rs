use bin_codec::*;
use bin_codec_derive::{BinDecodeBe};
#[test]
fn test_is_some() {
    #[derive(BinDecodeBe)]
    struct Struct {
        a_field: i32,
        #[bin(is_some("a_field == 0"))]
        b_field: Option<i32>,
    }
    let target = [0x12, 0x34, 0x56, 0x78];
    let (s, size) = Struct::decode(&target, &mut ()).unwrap();
    assert_eq!(size, 32);
    assert_eq!(s.a_field, 0x12345678);
    assert_eq!(None, s.b_field);
    //
    let target = [0, 0, 0, 0, 0x12, 0x34, 0x56, 0x78];
    let (s, size) = Struct::decode(&target, &mut ()).unwrap();
    assert_eq!(size, 64);
    assert_eq!(s.a_field, 0);
    assert_eq!(Some(0x12345678), s.b_field);
}
