#[macro_use]
extern crate criterion;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

use std::io::Cursor;
use bin_codec::*;
use bin_codec_derive::{BinEncodeBe};
use criterion::Criterion;
use bincode::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, BinEncodeBe, Default)]
struct Entity {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, BinEncodeBe, Default)]
struct World {
    vec: Vec<Entity>,
}

fn bench_ts_encode(c: &mut Criterion) {
    let world = World{ 
        vec: vec![
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }
        ]
    };
    let _shouldbe = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    c.bench_function("compare-vec-bin_codec", move |b| b.iter(|| {
        let mut target = [0u8; 48 * 8];
        let mut ctx = ();
        world.encode(&mut target, &mut ctx);
        // assert_eq!(target[1], 0x02);
    }));

    let world = World{ 
        vec: vec![
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 },
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }, 
            Entity { x: 0x01020304, y: 0x05060708 }
        ]
    };
    c.bench_function("compare-vec-bincode", move |b| b.iter(|| {
        let target: Vec<u8> = serialize(&world).unwrap();
        // assert_eq!(target[8], 0x04);
    }));

}

criterion_group!(benches, bench_ts_encode);
criterion_main!(benches);