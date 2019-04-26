use bin_codec::*;
use bin_codec_derive::{BinEncode, BinDecode};

#[derive(BinEncode)]
pub struct TsPack {
    // 00 : 0
    pub sync_code: u8,
    // 01 : 7
    #[bin(bits(1))]
    pub translation_error: bool,
    // 01 : 6
    #[bin(bits(1))]
    pub first_payload: bool,
    // 01 : 5
    #[bin(bits(1))]
    pub high_priority: bool,
    // 01 : 4
    #[bin(bits(13))]
    pub pid: u16,
    #[bin(bits(1))]
    pub first: bool,
    #[bin(bits(1))]
    pub last: bool,
    #[bin(skip(2))]
    #[bin(bits(4))]
    pub seq: i8,
}

#[test]
fn test_ts_encode() {
    let ts = TsPack {
        sync_code: 0x47,
        translation_error: false,
        first_payload: true,
        high_priority: false,
        pid: 0x1ABC,
        first: true,
        last: false,
        seq: 0xA,
    };
    let mut target = [0u8; 8];
    ts.encode_be(&mut target, 0, &mut Context::default()).unwrap();
    assert_eq!(target, [0x47, 0b0100_0000 | 0x1A, 0xBC, 0b1000_0000 | 0xA, 0 , 0 , 0 , 0]);
}