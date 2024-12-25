#![allow(static_mut_refs)]

use std::simd::{cmp::SimdPartialEq, u16x16};

use arrayvec::ArrayVec;

#[repr(align(64))]
#[derive(Copy, Clone)]
struct Alphabit {
    bits: [bool; 16],
    ids: [u16; 16],
    count: u32,
}

impl Alphabit {
    const fn new() -> Self {
        Self {
            bits: [false; 16],
            ids: [0; 16],
            count: 0,
        }
    }

    #[inline(always)]
    unsafe fn get(&self, id: u16) -> Option<bool> {
        let ids: u16x16 = u16x16::from_array(*self.ids[..16].as_array().unwrap_unchecked());
        let eq = ids.simd_eq(u16x16::splat(id));

        if (eq.to_bitmask() & ((1 << self.count) - 1)) != 0 {
            Some(*self.bits.get_unchecked(eq.first_set().unwrap_unchecked()))
        } else {
            None
        }
    }

    #[inline(always)]
    unsafe fn insert(&mut self, id: u16, bit: bool) {
        debug_assert!(!self.ids[..self.count as usize].contains(&id));
        debug_assert!(self.count < self.bits.len() as u32);

        *self.bits.get_unchecked_mut(self.count as usize) = bit;
        *self.ids.get_unchecked_mut(self.count as usize) = id;
        self.count += 1;
    }
}

struct Bits {
    bits: [Alphabit; 26],
}

impl Bits {
    const fn new() -> Self {
        Self {
            bits: [Alphabit::new(); 26],
        }
    }

    #[inline(always)]
    fn clear(&mut self) {
        self.bits.iter_mut().for_each(|x| x.count = 0);
    }

    #[inline(always)]
    unsafe fn get(&self, id1: usize, id2: usize) -> Option<bool> {
        self.bits.get_unchecked(id1).get(id2 as u16)
    }

    #[inline(always)]
    unsafe fn insert(&mut self, id1: usize, id2: usize, bit: bool) {
        self.bits.get_unchecked_mut(id1).insert(id2 as u16, bit);
    }
}

#[inline(always)]
fn id(reg: [u8; 3]) -> (u16, u16) {
    debug_assert!(reg[0] >= b'0' && reg[1] >= b'0' && reg[2] >= b'0');

    (
        (reg[0] - b'a') as u16,
        (reg[1] - b'0') as u16 * 128 + (reg[2] - b'0') as u16,
    )
}

#[inline(always)]
unsafe fn read_bits(mut input: *const u8) -> (u64, u64) {
    let (mut x, mut y) = (0, 0);

    input = input.add(5);

    for i in 0..45 {
        let bit = (*input - b'0') as u64;
        x |= bit << i;

        input = input.add(7);
    }

    for i in 0..45 {
        let bit = (*input - b'0') as u64;
        y |= bit << i;

        input = input.add(7);
    }

    (x, y)
}

unsafe fn inner_p1(input: &[u8]) -> u64 {
    const Z_IDS: [usize; 46] = const {
        let mut ids = [0; 46];

        let mut i = 0;
        while i < 46 {
            ids[i] = (i / 10) * 128 + (i % 10);
            i += 1;
        }

        ids
    };

    static mut BITS: Bits = Bits::new();
    static mut GATES: [ArrayVec<(u16, u16, u16, u16, u16, u16), 48>; 26] =
        [const { ArrayVec::new_const() }; 26];

    BITS.clear();
    GATES.iter_mut().for_each(|x| x.clear());
    let input = input.as_ptr();

    let (x, y) = read_bits(input);
    let mut input = input.add(631);

    for _ in 0..222 {
        let first = *input;

        if first == b'x' || first == b'y' {
            let bit = (*input.add(1) - b'0') * 10 + (*input.add(2) - b'0');
            let (bitx, bity) = ((x >> bit) & 1, (y >> bit) & 1);
            let dst: (u16, u16);

            let out = match *input.add(4) {
                b'A' => {
                    dst = id([*input.add(15), *input.add(16), *input.add(17)]);
                    input = input.add(19);
                    bitx & bity
                }
                b'X' => {
                    dst = id([*input.add(15), *input.add(16), *input.add(17)]);
                    input = input.add(19);
                    bitx ^ bity
                }
                b'O' => {
                    dst = id([*input.add(14), *input.add(15), *input.add(16)]);
                    input = input.add(18);
                    bitx | bity
                }
                _ => std::hint::unreachable_unchecked(),
            };

            BITS.insert(dst.0 as usize, dst.1 as usize, out != 0);
        } else {
            let a = id([*input, *input.add(1), *input.add(2)]);
            let op = *input.add(4);
            let b: (u16, u16);
            let c: (u16, u16);

            match op {
                b'A' | b'X' => {
                    b = id([*input.add(8), *input.add(9), *input.add(10)]);
                    c = id([*input.add(15), *input.add(16), *input.add(17)]);
                    input = input.add(19);
                }
                b'O' => {
                    b = id([*input.add(7), *input.add(8), *input.add(9)]);
                    c = id([*input.add(14), *input.add(15), *input.add(16)]);
                    input = input.add(18);
                }
                _ => std::hint::unreachable_unchecked(),
            };

            GATES
                .get_unchecked_mut(c.0 as usize)
                .push_unchecked((op as u16, a.0, a.1, b.0, b.1, c.1));
        }
    }

    unsafe fn dfs<const INSERT_CACHE: bool>(reg: (u16, u16)) -> bool {
        if let Some(bit) = BITS.get(reg.0 as usize, reg.1 as usize) {
            return bit;
        }

        let gates = GATES.get_unchecked(reg.0 as usize);

        for &gate in gates {
            let (op, a0, a1, b0, b1, c1) = gate;
            if c1 != reg.1 {
                continue;
            }

            let result = match op as u8 {
                b'A' => dfs::<true>((a0, a1)) && dfs::<true>((b0, b1)),
                b'X' => dfs::<true>((a0, a1)) != dfs::<true>((b0, b1)),
                b'O' => dfs::<true>((a0, a1)) || dfs::<true>((b0, b1)),
                _ => std::hint::unreachable_unchecked(),
            };

            if INSERT_CACHE {
                BITS.insert(reg.0 as usize, reg.1 as usize, result);
            }
            return result;
        }

        std::hint::unreachable_unchecked();
    }

    let mut z = 0;

    for i in 0..=45 {
        if dfs::<false>(((b'z' - b'a') as u16, Z_IDS[i] as u16)) {
            z |= 1 << i;
        }
    }

    z
}

unsafe fn inner_p2(input: &[u8]) -> &'static str {
    ""
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> &'static str {
    unsafe { inner_p2(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input24.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 53755311654662);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), "dkr,ggk,hhh,htp,rhv,z05,z15,z20");
    }
}
