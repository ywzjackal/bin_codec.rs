#[macro_use]
extern crate criterion;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bin_codec::*;
use bin_codec_derive::{BinEncodeBe};
use criterion::Criterion;
use bincode::*;

#[derive(Serialize, BinEncodeBe, Default)]
struct Entity {
    _0: i128,
    _1: i128,
    _2: i128,
    _3: i128,
    _4: i128,
    _5: i128,
    _6: i128,
    _7: i128,
    _8: i128,
    _9: i128,
    _10: i128,
    _11: i128,
    _12: i128,
    _13: i128,
    _14: i128,
    // _15: i128,
    // _16: i128,
}

#[derive(Serialize, BinEncodeBe, Default)]
struct EntityBig {
    _0: i128,
    _1: i128,
    _2: i128,
    _3: i128,
    _4: i128,
    _5: i128,
    _6: i128,
    _7: i128,
    _8: i128,
    _9: i128,
    _10: i128,
    _11: i128,
    _12: i128,
    _13: i128,
    _14: i128,
    _15: i128,
    _16: i128,
    _17: i128,
    _18: i128,
    _19: i128,
    _20: i128,
    _21: i128,
    _22: i128,
    _23: i128,
    _24: i128,
    _25: i128,
    _26: i128,
    _27: i128,
    _28: i128,
    _29: i128,
    _30: i128,
    _31: i128,
    _32: i128,
    _33: i128,
    _34: i128,
    _35: i128,
    _36: i128,
    _37: i128,
    _38: i128,
    _39: i128,
    _40: i128,
    _41: i128,
}

fn bench_ts_encode(c: &mut Criterion) {
    let entry = Entity::default();
    println!("Entity Size: {} bytes", std::mem::size_of::<Entity>());
    c.bench_function("compare-bin_codec", move |b| b.iter(|| {
        let mut target = [0u8; std::mem::size_of::<Entity>()];
        let mut ctx = ();
        entry.encode(&mut target, &mut ctx);
    }));

    let entry = Entity::default();
    c.bench_function("compare-bincode", move |b| b.iter(|| {
        let _: Vec<u8> = serialize(&entry).unwrap();
    }));

    let entry = EntityBig::default();
    println!("Entity Size: {} bytes", std::mem::size_of::<EntityBig>());
    c.bench_function("compare-big-bin_codec", move |b| b.iter(|| {
        let mut target = [0u8; std::mem::size_of::<EntityBig>()];
        let mut ctx = ();
        entry.encode(&mut target, &mut ctx);
    }));

    let entry = EntityBig::default();
    c.bench_function("compare-big-bincode", move |b| b.iter(|| {
        let _: Vec<u8> = serialize(&entry).unwrap();
    }));

}

criterion_group!(benches, bench_ts_encode);
criterion_main!(benches);