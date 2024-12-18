#![allow(static_mut_refs)]

use std::{
    ops::{Add, BitAnd, BitXor, Mul, Shr, Sub},
    simd::{cmp::SimdPartialEq, num::SimdUint, u32x8, u8x16, u8x8},
};

const LUT_RAW: &[u8] = include_bytes!("../lut/day17.bin");

const REG_A_OFFSET: usize = "Register A: ".len();
const PROG_BXL1_OFFSET: usize =
    "Register A: XXXXXXXX\nRegister B: 0\nRegister C: 0\n\nProgram: X,Y,X,".len();
const PROG_BODY_OFFSET: usize =
    "Register A: XXXXXXXX\nRegister B: 0\nRegister C: 0\n\nProgram: X,Y,X,Y,X,Y,".len();

unsafe fn inner_p1(input: &[u8]) -> &'static str {
    static mut OUTBUF: [u8; 32] = [0; 32];

    let zero = u8x8::splat(b'0');
    let digits = u8x8::from_array(
        *input
            .get_unchecked(REG_A_OFFSET..REG_A_OFFSET + 8)
            .as_array()
            .unwrap_unchecked(),
    );

    let digits = digits.sub(zero);
    let digits: u32x8 = digits.cast();
    let mult = u32x8::from_array([10000000, 1000000, 100000, 10000, 1000, 100, 10, 1]);
    let a = digits.mul(mult).reduce_sum();

    let bxl_1 = (*input.as_ptr().add(PROG_BXL1_OFFSET) - b'0') as u32;

    // we only need to find the operand for B ^= x in the loop, everything else
    // is constant
    let prog = u8x16::from_array(
        *input
            .get_unchecked(PROG_BODY_OFFSET..PROG_BODY_OFFSET + 16)
            .as_array()
            .unwrap_unchecked(),
    );

    let bxl_mask = u8x16::from_array([b'1', 0, 0, 0, b'1', 0, 0, 0, b'1', 0, 0, 0, b'1', 0, 0, 0]);
    let bxl_mask = prog.simd_eq(bxl_mask);
    let bxl_loc = bxl_mask.first_set().unwrap_unchecked();

    let bxl_2 = (*input.as_ptr().add(PROG_BODY_OFFSET).add(bxl_loc).add(2) - b'0') as u32;

    // for each iteration i:
    // b = (a >> (i * 3)) & 0x7
    // out = b ^ bxl_1
    // out ^= a >> b
    // out ^= bxl_2

    let a_simd = u32x8::splat(a);
    let a_shr = u32x8::from_array([0, 3, 6, 9, 12, 15, 18, 21]);
    let modmask = u32x8::splat(0x7);
    let bxl_1_simd = u32x8::splat(bxl_1);
    let bxl_2_simd = u32x8::splat(bxl_2);

    let b = a_simd.shr(a_shr).bitand(modmask); // b = (a >> (i * 3)) & 0x7
    let b = b.bitxor(bxl_1_simd); // b ^= bxl_1
    let b = b.bitxor(a_simd.shr(a_shr).shr(b)); // b ^= a >> b
    let b = b.bitxor(bxl_2_simd); // b ^= bxl_2
    let b = b.bitand(modmask);

    // if A is <2^24 then there are only 8 digits, i'm assuming A is always
    // greater than/equal to 2^24 so there are 9 digits
    let last = {
        let a = a >> 24;
        let b = (a & 0x7) ^ bxl_1;
        let c = a >> b;
        ((b ^ c ^ bxl_2) & 0x7) as u8
    };

    // write the output
    let b: u8x8 = b.cast().add(u8x8::splat(b'0'));
    let (l, r) = b.interleave(u8x8::splat(b','));

    l.copy_to_slice(&mut OUTBUF[0..8]);
    r.copy_to_slice(&mut OUTBUF[8..16]);

    OUTBUF[16] = last + b'0';

    std::str::from_utf8_unchecked(&OUTBUF[..17])
}

unsafe fn inner_p2(input: &[u8]) -> u64 {
    let bxl_1 = (*input.as_ptr().add(PROG_BXL1_OFFSET) - b'0') as u32;

    // we only need to find the operand for B ^= x in the loop, everything else
    // is constant
    let prog = u8x16::from_array(
        *input
            .get_unchecked(PROG_BODY_OFFSET..PROG_BODY_OFFSET + 16)
            .as_array()
            .unwrap_unchecked(),
    );

    let bxl_mask = u8x16::from_array([b'1', 0, 0, 0, b'1', 0, 0, 0, b'1', 0, 0, 0, b'1', 0, 0, 0]);
    let bxl_mask = prog.simd_eq(bxl_mask);
    let bxl_loc = bxl_mask.first_set().unwrap_unchecked();

    let bxl_2 = (*input.as_ptr().add(PROG_BODY_OFFSET).add(bxl_loc).add(2) - b'0') as u32;

    let bxc_mask = u8x16::from_array([b'4', 0, 0, 0, b'4', 0, 0, 0, b'4', 0, 0, 0, b'4', 0, 0, 0]);
    let bxc_mask = prog.simd_eq(bxc_mask);
    let bxc_loc = bxc_mask.first_set().unwrap_unchecked();

    let bxc_operand = (*input.as_ptr().add(PROG_BODY_OFFSET).add(bxc_loc).add(2) - b'0') as u32;

    // orders
    // [0, 1, 4, 5], // 0145
    // [0, 4, 1, 5], // 0415
    // [1, 0, 4, 5], // 1045
    // [1, 4, 0, 5], // 1405
    // [1, 4, 5, 0], // 1450
    // [4, 0, 1, 5], // 4015
    // [4, 1, 0, 5], // 4105
    // [4, 1, 5, 0], // 4150
    let body_ops = [
        *input.as_ptr().add(PROG_BODY_OFFSET) - b'0',
        *input.as_ptr().add(PROG_BODY_OFFSET).add(4) - b'0',
        *input.as_ptr().add(PROG_BODY_OFFSET).add(8) - b'0',
        *input.as_ptr().add(PROG_BODY_OFFSET).add(12) - b'0',
    ];

    let ord = match body_ops {
        [0, 1, 4, 5] => 0,
        [0, 4, 1, 5] => 1,
        [1, 0, 4, 5] => 2,
        [1, 4, 0, 5] => 3,
        [1, 4, 5, 0] => 4,
        [4, 0, 1, 5] => 5,
        [4, 1, 0, 5] => 6,
        [4, 1, 5, 0] => 7,
        _ => std::hint::unreachable_unchecked(),
    };

    let index = (bxl_1 as usize) * 512
        + (ord as usize) * 64
        + (bxl_2 as usize) * 8
        + (bxc_operand as usize);

    u64::from_ne_bytes(
        LUT_RAW
            .get_unchecked(index * 8..index * 8 + 8)
            .try_into()
            .unwrap_unchecked(),
    )
}

pub fn part1(input: &str) -> &'static str {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { inner_p2(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input17.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), "7,0,3,1,2,6,3,7,1");
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 109020013201563);
    }
}
