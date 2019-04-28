#[macro_use]
extern crate criterion;
use bin_codec::*;
use bin_codec_derive::{BinEncodeBe, BinDecodeBe};
use criterion::Criterion;

#[derive(BinEncodeBe, BinDecodeBe)]
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

fn bench_ts_encode(c: &mut Criterion) {
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
    c.bench_function("bench_ts_encode", move |b| b.iter(|| {
        let mut target = [0u8; 4];
        let mut ctx = ();
        ts.encode(&mut target, &mut ctx);
        // assert_eq!(target[1], 0x5a);
    }));
    let target = [0x47, 0b0100_0000 | 0x1A, 0xBC, 0b1000_0000 | 0xA];
    c.bench_function("bench_ts_decode", move |b| b.iter(|| {
        let (_rt, _bits) = TsPack::decode(&target, &mut ()).unwrap();
        // assert_eq!(_rt.pid, 0x1abc);
    }));
}

criterion_group!(benches, bench_ts_encode);
criterion_main!(benches);