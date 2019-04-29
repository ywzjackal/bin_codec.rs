#[macro_use]
extern crate criterion;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bin_codec::*;
use bin_codec_derive::BinEncodeBe;
use bincode::*;
use criterion::Criterion;

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
    c.bench_function("compare-bin_codec", move |b| {
        b.iter(|| {
            let mut target = [0u8; std::mem::size_of::<Entity>()];
            let mut ctx = ();
            entry.encode(&mut target, &mut ctx);
        })
    });

    let entry = Entity::default();
    c.bench_function("compare-bincode", move |b| {
        b.iter(|| {
            let _: Vec<u8> = serialize(&entry).unwrap();
        })
    });

    let entry = Entity::default();
    c.bench_function("compare-raw", move |b| {
        b.iter(|| {
            unsafe {
                let mut target = [0u8; std::mem::size_of::<Entity>()];
                *(target.as_mut_ptr().offset(0 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._0.to_be();
                *(target.as_mut_ptr().offset(1 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._1.to_be();
                *(target.as_mut_ptr().offset(2 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._2.to_be();
                *(target.as_mut_ptr().offset(3 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._3.to_be();
                *(target.as_mut_ptr().offset(4 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._4.to_be();
                *(target.as_mut_ptr().offset(5 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._5.to_be();
                *(target.as_mut_ptr().offset(6 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._6.to_be();
                *(target.as_mut_ptr().offset(7 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._7.to_be();
                *(target.as_mut_ptr().offset(8 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._8.to_be();
                *(target.as_mut_ptr().offset(9 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._9.to_be();
                *(target.as_mut_ptr().offset(10 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._10.to_be();
                *(target.as_mut_ptr().offset(11 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._11.to_be();
                *(target.as_mut_ptr().offset(12 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._12.to_be();
                *(target.as_mut_ptr().offset(13 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._13.to_be();
                *(target.as_mut_ptr().offset(14 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._14.to_be();
            }
        })
    });

    let entry = EntityBig::default();
    // println!("Entity Size: {} bytes", std::mem::size_of::<EntityBig>());
    c.bench_function("compare-big-bin_codec", move |b| {
        b.iter(|| {
            let mut target = [0u8; std::mem::size_of::<EntityBig>()];
            let mut ctx = ();
            entry.encode(&mut target, &mut ctx);
        })
    });

    let entry = EntityBig::default();
    c.bench_function("compare-big-bincode", move |b| {
        b.iter(|| {
            let _: Vec<u8> = serialize(&entry).unwrap();
        })
    });

    let entry = EntityBig::default();
    c.bench_function("compare-big-raw", move |b| {
        b.iter(|| {
            unsafe {
                let mut target = [0u8; std::mem::size_of::<Entity>()];
                *(target.as_mut_ptr().offset(0 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._0.to_be();
                *(target.as_mut_ptr().offset(1 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._1.to_be();
                *(target.as_mut_ptr().offset(2 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._2.to_be();
                *(target.as_mut_ptr().offset(3 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._3.to_be();
                *(target.as_mut_ptr().offset(4 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._4.to_be();
                *(target.as_mut_ptr().offset(5 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._5.to_be();
                *(target.as_mut_ptr().offset(6 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._6.to_be();
                *(target.as_mut_ptr().offset(7 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._7.to_be();
                *(target.as_mut_ptr().offset(8 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._8.to_be();
                *(target.as_mut_ptr().offset(9 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._9.to_be();
                *(target.as_mut_ptr().offset(10 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._10.to_be();
                *(target.as_mut_ptr().offset(11 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._11.to_be();
                *(target.as_mut_ptr().offset(12 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._12.to_be();
                *(target.as_mut_ptr().offset(13 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._13.to_be();
                *(target.as_mut_ptr().offset(14 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._14.to_be();
                *(target.as_mut_ptr().offset(15 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._15.to_be();
                *(target.as_mut_ptr().offset(16 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._16.to_be();
                *(target.as_mut_ptr().offset(17 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._17.to_be();
                *(target.as_mut_ptr().offset(18 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._18.to_be();
                *(target.as_mut_ptr().offset(19 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._19.to_be();
                *(target.as_mut_ptr().offset(20 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._20.to_be();
                *(target.as_mut_ptr().offset(21 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._21.to_be();
                *(target.as_mut_ptr().offset(22 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._22.to_be();
                *(target.as_mut_ptr().offset(23 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._23.to_be();
                *(target.as_mut_ptr().offset(24 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._24.to_be();
                *(target.as_mut_ptr().offset(25 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._25.to_be();
                *(target.as_mut_ptr().offset(26 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._26.to_be();
                *(target.as_mut_ptr().offset(27 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._27.to_be();
                *(target.as_mut_ptr().offset(28 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._28.to_be();
                *(target.as_mut_ptr().offset(29 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._29.to_be();
                *(target.as_mut_ptr().offset(30 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._30.to_be();
                *(target.as_mut_ptr().offset(31 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._31.to_be();
                *(target.as_mut_ptr().offset(32 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._32.to_be();
                *(target.as_mut_ptr().offset(33 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._33.to_be();
                *(target.as_mut_ptr().offset(34 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._34.to_be();
                *(target.as_mut_ptr().offset(35 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._35.to_be();
                *(target.as_mut_ptr().offset(36 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._36.to_be();
                *(target.as_mut_ptr().offset(37 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._37.to_be();
                *(target.as_mut_ptr().offset(38 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._38.to_be();
                *(target.as_mut_ptr().offset(39 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._39.to_be();
                *(target.as_mut_ptr().offset(40 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._40.to_be();
                *(target.as_mut_ptr().offset(41 * std::mem::size_of::<i128>() as isize) as *mut i128) = entry._41.to_be();
            }
        })
    });
}

criterion_group!(benches, bench_ts_encode);
criterion_main!(benches);
