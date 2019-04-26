use bin_codec::*;
use bin_codec_derive2::{BinEncodeBe};

#[derive(BinEncodeBe)]
pub struct TsPack {
    // 00 : 0
    pub sync_code: u8,
    // 01 : 7
    pub translation_error: bool,
    // 01 : 6
    pub first_payload: bool,
    // 01 : 5
    pub high_priority: bool,
    // 01 : 4
    #[bin(bits(13))]
    pub pid: u16,
    pub first: bool,
    pub last: bool,
    #[bin(skip(2))]
    #[bin(bits(4))]
    pub seq: i8,
}

#[derive(BinEncodeBe)]
pub struct Wrap {
    #[bin(count(1))]
    pub wrap: Vec<TsPack>,
}
#[test]
fn test_wrap_vec() {
    let ts = Wrap { 
        wrap: vec![
            TsPack {
                sync_code: 0x47,
                translation_error: false,
                first_payload: true,
                high_priority: false,
                pid: 0x1ABC,
                first: true,
                last: false,
                seq: 0xA,
            }
        ],
    };
    let mut target = [0u8; 4];
    ts.encode(&mut target, &mut ());
    let shouldbe = [0x47, 0b0100_0000 | 0x1A, 0xBC, 0b1000_0000 | 0xA];
    assert_eq!(target, shouldbe, "\r\n{:02X?} \r\n {:02X?}", target, shouldbe);
}