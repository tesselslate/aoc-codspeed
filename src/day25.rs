#![allow(static_mut_refs)]

use std::{
    ops::BitAnd,
    simd::{cmp::SimdPartialEq, num::SimdUint, u64x4, u8x32},
};

#[derive(Copy, Clone)]
#[repr(align(32))]
struct Bits([u64; 4]);

#[repr(align(64))]
struct Keys([[Bits; 6]; 5]);

impl Bits {
    const fn new() -> Self {
        Self([0; 4])
    }

    #[inline(always)]
    unsafe fn bitor(&mut self, other: Self) {
        self.0[0] |= other.0[0];
        self.0[1] |= other.0[1];
        self.0[2] |= other.0[2];
        self.0[3] |= other.0[3];
    }

    #[inline(always)]
    unsafe fn set(&mut self, bit: usize) {
        let (idx, bit) = (bit / 64, bit % 64);

        *self.0.get_unchecked_mut(idx) |= 1 << bit;
    }
}

impl Keys {
    const fn new() -> Self {
        Self([[Bits::new(); 6]; 5])
    }

    #[inline(always)]
    fn clear(&mut self) {
        self.0 = [[Bits::new(); 6]; 5];
    }
}

unsafe fn inner_p1(input: &[u8]) -> u64 {
    const MASKS: [[u8; 32]; 5] = const {
        let mut arrays = [[0; 32]; 5];

        let mut i = 0;
        while i < 5 {
            let mut j = 0;
            while j < 32 {
                if j >= i && (j - i) % 6 == 0 {
                    arrays[i][j] = 0xFF;
                }

                j += 1;
            }

            i += 1;
        }

        arrays
    };

    static mut LOCKS: [[u32; 5]; 250] = [[0; 5]; 250];
    static mut KEYS: Keys = Keys::new();

    LOCKS.iter_mut().for_each(|x| *x = [0; 5]);
    KEYS.clear();

    let mut input = input.as_ptr();
    let (mut lock_idx, mut key_idx) = (0, 0);

    let masks = MASKS.map(|x| u8x32::from_array(x));

    for i in 0..500 {
        debug_assert!(*input.add(6) != b'\n');

        let data = u8x32::from_array(
            *std::slice::from_raw_parts(input.add(6), 32)
                .as_array()
                .unwrap_unchecked(),
        );

        let is_key = *input == b'.';
        let eqmask = u8x32::splat(*input);

        let a = data
            .bitand(masks[0])
            .simd_eq(eqmask)
            .to_bitmask()
            .count_ones();
        let b = data
            .bitand(masks[1])
            .simd_eq(eqmask)
            .to_bitmask()
            .count_ones();
        let c = data
            .bitand(masks[2])
            .simd_eq(eqmask)
            .to_bitmask()
            .count_ones();
        let d = data
            .bitand(masks[3])
            .simd_eq(eqmask)
            .to_bitmask()
            .count_ones();
        let e = data
            .bitand(masks[4])
            .simd_eq(eqmask)
            .to_bitmask()
            .count_ones();

        if is_key {
            KEYS.0[0].get_unchecked_mut(a as usize).set(key_idx);
            KEYS.0[1].get_unchecked_mut(b as usize).set(key_idx);
            KEYS.0[2].get_unchecked_mut(c as usize).set(key_idx);
            KEYS.0[3].get_unchecked_mut(d as usize).set(key_idx);
            KEYS.0[4].get_unchecked_mut(e as usize).set(key_idx);

            key_idx += 1;
        } else {
            *LOCKS.get_unchecked_mut(lock_idx) = [a, b, c, d, e];

            lock_idx += 1;
        }

        // TODO make sure this gets optimized
        if i != 499 {
            input = input.add(43);
        }
    }

    for i in (1..=5).rev() {
        KEYS.0[0][i - 1].bitor(KEYS.0[0][i]);
        KEYS.0[1][i - 1].bitor(KEYS.0[1][i]);
        KEYS.0[2][i - 1].bitor(KEYS.0[2][i]);
        KEYS.0[3][i - 1].bitor(KEYS.0[3][i]);
        KEYS.0[4][i - 1].bitor(KEYS.0[4][i]);
    }

    debug_assert!(lock_idx == 250 && key_idx == 250);

    let mut sum = 0;

    for lock in LOCKS {
        let [a, b, c, d, e] = lock;

        let a = u64x4::from_array(KEYS.0[0].get_unchecked(a as usize).0);
        let b = u64x4::from_array(KEYS.0[1].get_unchecked(b as usize).0);
        let c = u64x4::from_array(KEYS.0[2].get_unchecked(c as usize).0);
        let d = u64x4::from_array(KEYS.0[3].get_unchecked(d as usize).0);
        let e = u64x4::from_array(KEYS.0[4].get_unchecked(e as usize).0);

        sum += std::intrinsics::simd::simd_ctpop(a.bitand(b).bitand(c).bitand(d).bitand(e))
            .reduce_sum();
    }

    sum
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input25.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 3021);
    }
}
