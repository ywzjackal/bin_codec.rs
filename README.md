# bin_codec.rs
binary codec macro for rust

Welcome any suggest and pull request

**Only support struct codec for now!**

**And not stable or safe for use !**

# Field Attributes
* **bits**: bit size defination
* **is_some**: for `Option` field
* **count**: for `Vec` field
* **has_next**: for `Vec` field

## `#[bin(bits({num}))]`
* effect in encode mode and decode mode
```rust
use bin_codec::*;
use bin_codec_derive::{BinEncodeBe, BinDecodeBe};
#[test]
fn test_has_bit_field() {
    #[derive(BinEncodeBe)]
    struct Struct<T> where T: Encode {
        #[bin(bits(24))]
        a_field: T,
        #[bin(bits(16))]
        b_field: T,
    }

    let s = Struct::<_> {
        a_field: 0x345678_i32,
        b_field: 0x3344,
    };

    let mut target = [0u8; 5];
    s.encode(&mut target, 0, &mut ()).unwrap();
    assert_eq!(&[0x34, 0x56, 0x78, 0x33, 0x44], &target[..]);

    let mut target = [0u8; 5];
    s.encode(&mut target, 0, &mut()).unwrap();
    println!("{:#02X?}", target);
    assert_eq!(&[0x78, 0x56, 0x34, 0x44, 0x33], &target[..]);
}
```
## `#[bin(is_some({code})]`
* effect in decode mode
* effect with Option field
### Option field example:
```rust
use bin_codec::*;
use bin_codec_derive::{BinEncodeBe, BinDecodeBe};
#[test]
fn test_option() {
    #[derive(BinEncodeBe, BinDecodeBe)]
    struct Struct {
        a_field: i32,
        #[bin(is_some("a_field == 0"))]
        b_field: Option<i32>,
    }
    let target = [0x12,0x34,0x56,0x78];
    let (s, size) = Struct::decode(&target, 0, &mut Context::default()).unwrap();
    assert_eq!(size, 32);
    assert_eq!(s.a_field, 0x12345678);
    assert_eq!(None, s.b_field);
    //
    let target = [0,0,0,0,0x12,0x34,0x56,0x78];
    let (s, size) = Struct::decode(&target, 0, &mut Context::default()).unwrap();
    assert_eq!(size, 64);
    assert_eq!(s.a_field, 0);
    assert_eq!(Some(0x12345678), s.b_field);
}
```
### Vec field example:
```rust
use bin_codec::*;
use bin_codec_derive::{BinEncodeBe, BinDecodeBe};
#[test]
fn test_vec() {
    #[derive(BinDecodeBe, Debug)]
    #[bin(has_next(value != 0))]
    struct Item {
        value: u8,
    }
    #[derive(BinDecodeBe, Debug)]
    struct Struct {
        a_field: u8,
        #[bin(is_some(a_field == 0))]
        b_field: Vec<Item>,
    }
    let (s, size) = Struct::decode(&[0, 1, 2, 0, 1], 0, &mut Context::default()).unwrap();
    assert_eq!(0, s.a_field);
    assert_eq!(3, s.b_field.len());
}
```
## `#[bin(count({code}))]`
* effect in decode mode
* effect with Vec field
```rust
use bin_codec::*;
use bin_codec_derive::{BinEncodeBe, BinDecodeBe};
#[test]
fn test_bits_on_vec() {
    #[derive(BinDecodeBe, BinEncodeBe)]
    struct S {
        count: u8,
        #[bin(bits(16))]
        #[bin(count("count as usize"))]
        values: Vec<u32>,
    }

    let (s, size) = S::decode(&[3,1,2,3,4,5,6], 0, &mut Context::default()).unwrap();
    assert_eq!(size, 56);
    assert_eq!(s.count, 3);
    assert_eq!(&[0x0102, 0x0304, 0x0506], s.values.as_slice());
    //
    let mut target = [0u8; 7];
    let size = s.encode(&mut target, 0, &mut Context::default()).unwrap();
    assert_eq!(size, 56);
    assert_eq!(&[3,1,2,3,4,5,6], &target);
    //
    let (s, size) = S::decode(&[3,1,2,3,4,5,6], 0, &mut Context::default()).unwrap();
    assert_eq!(size, 56);
    assert_eq!(s.count, 3);
    assert_eq!(&[0x201, 0x403, 0x605], s.values.as_slice());
    //
    let mut target = [0u8; 7];
    let size = s.encode(&mut target, 0, &mut Context::default()).unwrap();
    assert_eq!(size, 56);
    assert_eq!(&[3,1,2,3,4,5,6], &target);
}
```
# License
This project is licensed under either of

* Apache License, Version 2.0, (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)

at your option.
