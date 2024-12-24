#![allow(static_mut_refs)]

use std::simd::{cmp::SimdPartialEq, u16x16};

#[inline(always)]
fn id(a: u8, b: u8) -> usize {
    (a - b'a') as usize * 26 + (b - b'a') as usize
}

#[repr(align(64))]
struct Graph([[u16; 16]; 26 * 26]);

impl Graph {
    const fn new() -> Self {
        Self([[0; 16]; 26 * 26])
    }

    #[inline(always)]
    unsafe fn read(&mut self, mut input: *const u8) {
        let mut counts = [0u8; 26 * 26];

        for _ in 0..3380 {
            let a = id(*input, *input.add(1));
            let b = id(*input.add(3), *input.add(4));

            input = input.add(6);

            self.0.get_unchecked_mut(a)[*counts.get_unchecked(a) as usize] = b as u16;
            self.0.get_unchecked_mut(b)[*counts.get_unchecked(b) as usize] = a as u16;
            *counts.get_unchecked_mut(a) += 1;
            *counts.get_unchecked_mut(b) += 1;
        }
    }
}

unsafe fn inner_p1(input: &[u8]) -> u64 {
    const T_START: usize = (b't' - b'a') as usize * 26;

    static mut COMPUTERS: Graph = Graph::new();
    COMPUTERS.read(input.as_ptr());

    let mut groups = 0;
    let mut dupes = 0;

    for i in T_START..T_START + 26 {
        for j in 1..13 {
            for k in 0..j {
                let a = COMPUTERS.0[i][j] as usize;
                let b = COMPUTERS.0[i][k] as usize;

                let edges =
                    u16x16::from_array(*COMPUTERS.0.get_unchecked(a).as_array().unwrap_unchecked());
                if (edges.simd_eq(u16x16::splat(b as u16)).to_bitmask() & 0x1fff) != 0 {
                    groups += 1;

                    if (a >= T_START && a < T_START + 26) || (b >= T_START && b < T_START + 26) {
                        dupes += 1;
                    }
                }
            }
        }
    }

    groups - (dupes >> 1)
}

unsafe fn inner_p2(input: &[u8]) -> &'static str {
    static mut OUTBUF: [u8; 64] = [0; 64];

    std::str::from_raw_parts(OUTBUF.as_ptr(), 0)
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

    const INPUT: &str = include_str!("../inputs/input23.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 1083);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), "as,bu,cp,dj,ez,fd,hu,it,kj,nx,pp,xh,yu");
    }
}
