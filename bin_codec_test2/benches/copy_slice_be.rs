#[macro_use]
extern crate criterion;
use bin_codec::*;
use criterion::*;

fn bench_bit_copy_slice_be(c: &mut Criterion) {
    let mut target = [0xfc; 20];
    let src = [
        0xff, 0x01, 0xff, 0x02, 0xff, 0x03, 0xff, 0x04, 0xff, 0x05, 0xff, 0x06, 0xff, 0x07, 0xff,
        0x08, 0xff, 0xff,
    ];
    c.bench(
        "copy_slice_be",
        Benchmark::new("inline", move |b| {
            b.iter(|| unsafe {
                bit_copy(target.as_mut_ptr(), &mut 6, src.as_ptr(), 1, 16 * 8 + 1);
            })
        })
        .with_function("macro", move |b| {
            b.iter(|| unsafe {
                use bin_codec::*;
                let offset = &mut 6;
                let target = target.as_mut_ptr();
                let src = src.as_ptr();
                let mut  src_start = 1;
                let bits = 16 * 8 + 1;
                bin_codec::bit_copy!(target, offset, src, src_start, bits, u128, u8);
            })
        }),
    );
}

criterion_group!(benches, bench_bit_copy_slice_be);
criterion_main!(benches);
