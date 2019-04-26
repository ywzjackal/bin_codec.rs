//const MASK_U8: [u8; 9] = [0, 0b1, 0b11, 0b111, 0b1111, 0b1_1111, 0b11_1111, 0b111_1111, 0b1111_1111];
const MASK_U8_REV: [u8; 9] = [
    0b1111_1111,
    0b111_1111,
    0b11_1111,
    0b1_1111,
    0b1111,
    0b111,
    0b11,
    0b1,
    0,
];
const MASK_U8_REV_BIT: [u8; 9] = [
    0, 0b10000000, 0b11000000, 0b11100000, 0b11110000, 0b11111000, 0b11111100, 0b11111110,
    0b11111111,
];
const CLEAR_U8: [u8; 9] = [
    0b1111_1111,
    0b1111_1110,
    0b1111_1100,
    0b1111_1000,
    0b1111_0000,
    0b1110_0000,
    0b1100_0000,
    0b1000_0000,
    0b0000_0000,
];

macro_rules! fast_copy_code {
    ($target: ident, $target_start: ident, $src: ident, $src_start: ident, $src_stop:ident, $($ty:ty),*) => {
        let target_byte = (*$target_start / 8) as isize;
        let target_start_remainder = *$target_start % 8;
        let src_start_remainder = $src_start % 8;
//        let total_bits = $src_stop - $src_start;
        let src_start_byte = ($src_start / 8) as isize;
        let src_stop_byte = ($src_stop - 1) / 8;
        let total_bytes = src_stop_byte + 1 - src_start_byte as usize;
//        println!("total bytes:{}, {} -> {}", total_bytes, src_stop_byte, src_start_byte);
        if target_start_remainder < src_start_remainder {
            // src << lsh
            let lsh = (src_start_remainder - target_start_remainder) % 8;
            $(
            if total_bytes >= std::mem::size_of::<$ty>() {
                let mut number: $ty = (*($src.offset(src_start_byte as isize) as *const $ty)).to_be() << lsh;
//                println!("num:{:b} = {:b} << {}", number, (*($src.offset(src_start_byte as isize) as *const $ty)).to_be(), lsh);
                let hb = (&mut number as *mut $ty as *mut u8).offset(std::mem::size_of::<$ty>() as isize - 1);
                *hb &= MASK_U8_REV[target_start_remainder];
                *hb |= (*$target.offset(target_byte)) & MASK_U8_REV_BIT[target_start_remainder];
//                println!("[{}] {:b} |= {:b}", target_byte, (*($target.offset(target_byte as isize) as *mut $ty)).to_be(), number);
                *($target.offset(target_byte) as *mut $ty) = number.to_be();
                let bits_copied = std::mem::size_of::<$ty>() * 8 - lsh - target_start_remainder;
                *$target_start += bits_copied;
                $src_start += bits_copied;
                continue;
            }
            )*
        } else {
            // src >> rsh
            let rsh = (target_start_remainder - src_start_remainder) % 8;
            $(
            if total_bytes >= std::mem::size_of::<$ty>() {
                let mut number: $ty = (*($src.offset(src_start_byte as isize) as *const $ty)).to_be() >> rsh;
//                println!("num:{:b} = {:b} >> {}", number, (*($src.offset(src_start_byte as isize) as *const $ty)).to_be(), rsh);
                let hb = (&mut number as *mut $ty as *mut u8).offset(std::mem::size_of::<$ty>() as isize - 1);
                *hb &= MASK_U8_REV[target_start_remainder];
                *hb |= (*$target.offset(target_byte)) & MASK_U8_REV_BIT[target_start_remainder];
//                println!("[{}] {:b} |= {:b}", target_byte, (*($target.offset(target_byte as isize) as *mut $ty)).to_be(), number);
                *($target.offset(target_byte as isize) as *mut $ty) = number.to_be();
                let bits_copied = std::mem::size_of::<$ty>() * 8 - target_start_remainder;
                *$target_start += bits_copied;
                $src_start += bits_copied;
                continue;
            }
            )*
        }
    }
}
#[inline]
pub unsafe fn bit_copy(
    target: *mut u8,
    target_start: &mut usize,
    src: *const u8,
    mut src_start: usize,
    bits: usize,
) {
    //    assert!(src_stop / 8 < src.len());
    //    assert!((target_start + src_stop - src_start) / 8 < target.len());
    let src_stop = src_start + bits;
    // println!("{}, {}, {}", target_start, src_start, src_stop);
    while src_start < src_stop {
        fast_copy_code!(
            target,
            target_start,
            src,
            src_start,
            src_stop,
            u128,
            u64,
            u32,
            u16,
            u8
        );
    }
    // println!("{}, {}, {}", target_start, src_start, src_stop);
    if src_start >= src_stop {
        *target.offset((*target_start as isize - 1) / 8) &= CLEAR_U8[src_start - src_stop];
        //        println!("[({} - 1) / 8 = {}] {:08b} &= {:08b}", target_start, (target_start-1) / 8, target[(target_start - 1) / 8], CLEAR_U8[src_start - src_stop])
    }
}

#[test]
fn test_bit_copy_slice() {
    let mut target = [0u8];
    let src = [0x00, 0x00, 0x00, 0xff];
    unsafe {
        bit_copy(target.as_mut_ptr(), &mut 0, src.as_ptr(), 28, 4);
    }
    assert_eq!(0xf0, target[0]);
}
#[test]
fn test_bit_copy_slice_be() {
    let mut target = [0xff; 20];
    let src = [
        0b1010_1010,
        0b1111_0000,
        0b0000_1111,
        0b1100_1100,
        0b0011_0011,
        0b1010_1010,
        0b1111_0000,
        0b0000_1111,
        0b1100_1100,
        0b0011_0011,
        0b1010_1010,
        0b1111_0000,
        0b0000_1111,
        0b1100_1100,
        0b0011_0011,
        0b1010_1010,
        0b1111_0000,
        0b0000_1111,
        0b1100_1100,
        0b0011_0011,
    ];
    unsafe {
        bit_copy(target.as_mut_ptr(), &mut 4, src.as_ptr(), 0, 15 * 8);
    }
    assert_eq!(target[0], 0b11111010);
    assert_eq!(target[1], 0b10101111);
    assert_eq!(target[14], 0b11000011);
    assert_eq!(target[15], 0b00110000);
    //
    let mut target = [0xfc; 20];
    let src = [
        0xff, 0x01, 0xff, 0x02, 0xff, 0x03, 0xff, 0x04, 0xff, 0x05, 0xff, 0x06, 0xff, 0x07, 0xff,
        0x08, 0xff, 0xff,
    ];
    unsafe {
        bit_copy(target.as_mut_ptr(), &mut 6, src.as_ptr(), 1, 16 * 8 + 1);
    }

    assert_eq!(target[0], 0b11111111);
    assert_eq!(target[1], 0b11111000);
    assert_eq!(target[15], 0b11111000);
    assert_eq!(target[16], 0b01000111);
    assert_eq!(target[17], 0b11000000);
}
