#![allow(static_mut_refs)]

use std::{
    ops::{Mul, Sub},
    simd::{cmp::SimdPartialEq, num::SimdUint, simd_swizzle, u16x16, u8x16, u8x32},
};

use arrayvec::ArrayVec;

#[repr(align(64))]
struct Graph([[u16; 16]; 26 * 26]);

impl Graph {
    const fn new() -> Self {
        Self([[0; 16]; 26 * 26])
    }

    #[inline(always)]
    unsafe fn contains(&self, a: usize, b: u16) -> bool {
        let edges = u16x16::from_array(*self.0.get_unchecked(a).as_array().unwrap_unchecked());
        (edges.simd_eq(u16x16::splat(b)).to_bitmask() & 0x1fff) != 0
    }

    #[inline(always)]
    unsafe fn read(&mut self, counts: &mut [u8; 26 * 26], mut input: *const u8) {
        let alpha = u16x16::splat(b'a' as u16);
        let mul = u16x16::from_array([26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1, 26, 1]);

        for _ in 0..845 {
            let bytes = u8x32::from_array(
                *std::slice::from_raw_parts(input, 32)
                    .as_array()
                    .unwrap_unchecked(),
            );

            let bytes: u8x16 = simd_swizzle!(
                bytes,
                [0, 1, 3, 4, 6, 7, 9, 10, 12, 13, 15, 16, 18, 19, 21, 22]
            );

            let ids: u16x16 = bytes.cast().sub(alpha).mul(mul);

            let a = (ids[0] + ids[1]) as usize;
            let b = (ids[2] + ids[3]) as usize;
            let c = (ids[4] + ids[5]) as usize;
            let d = (ids[6] + ids[7]) as usize;
            let e = (ids[8] + ids[9]) as usize;
            let f = (ids[10] + ids[11]) as usize;
            let g = (ids[12] + ids[13]) as usize;
            let h = (ids[14] + ids[15]) as usize;

            self.0.get_unchecked_mut(a)[*counts.get_unchecked(a) as usize] = b as u16;
            self.0.get_unchecked_mut(b)[*counts.get_unchecked(b) as usize] = a as u16;
            *counts.get_unchecked_mut(a) += 1;
            *counts.get_unchecked_mut(b) += 1;

            self.0.get_unchecked_mut(c)[*counts.get_unchecked(c) as usize] = d as u16;
            self.0.get_unchecked_mut(d)[*counts.get_unchecked(d) as usize] = c as u16;
            *counts.get_unchecked_mut(c) += 1;
            *counts.get_unchecked_mut(d) += 1;

            self.0.get_unchecked_mut(e)[*counts.get_unchecked(e) as usize] = f as u16;
            self.0.get_unchecked_mut(f)[*counts.get_unchecked(f) as usize] = e as u16;
            *counts.get_unchecked_mut(e) += 1;
            *counts.get_unchecked_mut(f) += 1;

            self.0.get_unchecked_mut(g)[*counts.get_unchecked(g) as usize] = h as u16;
            self.0.get_unchecked_mut(h)[*counts.get_unchecked(h) as usize] = g as u16;
            *counts.get_unchecked_mut(g) += 1;
            *counts.get_unchecked_mut(h) += 1;

            input = input.add(24);
        }

        debug_assert!(counts.iter().filter(|&&x| x == 13).count() == 520);
        debug_assert!(counts.iter().filter(|&&x| x == 0).count() == 26 * 26 - 520);
    }
}

#[repr(align(64))]
unsafe fn inner_p1(input: &[u8]) -> u64 {
    const T_START: usize = (b't' - b'a') as usize * 26;

    static mut COMPUTERS: Graph = Graph::new();

    let mut counts = [0; 26 * 26];
    COMPUTERS.read(&mut counts, input.as_ptr());

    let mut groups = 0;
    let mut dupes = 0;
    let mut triplets = 0;

    for i in T_START..T_START + 26 {
        if counts[i] == 0 {
            continue;
        }

        for j in 1..13 {
            for k in 0..j {
                let a = COMPUTERS.0[i][j] as usize;
                let b = COMPUTERS.0[i][k] as usize;

                if COMPUTERS.contains(a, b as u16) {
                    groups += 1;

                    let at = a >= T_START && a < T_START + 26;
                    let bt = b >= T_START && b < T_START + 26;

                    if at != bt {
                        dupes += 1;
                    } else if at && bt {
                        triplets += 1;
                    }
                }
            }
        }
    }

    groups - (dupes / 2) - (triplets / 3) * 2
}

#[repr(align(64))]
unsafe fn inner_p2(input: &[u8]) -> &'static str {
    static mut OUTBUF: [u8; 64] = [
        0, 0, b',', 0, 0, b',', 0, 0, b',', 0, 0, b',', 0, 0, b',', 0, 0, b',', 0, 0, b',', 0, 0,
        b',', 0, 0, b',', 0, 0, b',', 0, 0, b',', 0, 0, b',', 0, 0, b',', 0, 0, b',', 0, 0, b',',
        0, 0, b',', 0, 0, b',', 0, 0, b',', 0, 0, b',', 0, 0, b',', 0, 0, b',', 0,
    ];
    static mut COMPUTERS: Graph = Graph::new();

    let mut counts = [0; 26 * 26];
    COMPUTERS.read(&mut counts, input.as_ptr());

    for i in 0..26 * 26 {
        if counts[i] == 0 {
            continue;
        }

        let ids: &[u16; 16] = COMPUTERS.0.get_unchecked(i);
        let adj: [u16x16; 13] = std::array::from_fn(|j| {
            u16x16::from_array(
                *COMPUTERS
                    .0
                    .get_unchecked(ids[j] as usize)
                    .as_array()
                    .unwrap_unchecked(),
            )
        });

        let mut adjcounts: [u16; 16] = [0; 16];
        for j in 0..13 {
            let id = u16x16::splat(ids[j]);

            for k in 0..13 {
                if (adj[k].simd_eq(id).to_bitmask() & 0x1fff) != 0 {
                    adjcounts[j] |= 1 << k;
                }
            }
        }

        let mut valid = 0;
        for count in adjcounts {
            valid += 1 * (count.count_ones() == 11) as usize;
        }

        if valid != 12 {
            continue;
        }

        let mut clique: ArrayVec<u16, 13> = ArrayVec::new_const();
        clique.push_unchecked(i as u16);
        for i in 0..13 {
            if adjcounts[i].count_ones() == 11 {
                clique.push_unchecked(ids[i]);
            }
        }

        debug_assert!(clique.len() == 13);

        clique.sort_unstable();

        for i in 0..13 {
            let (a, b) = (clique.get_unchecked(i) / 26, clique.get_unchecked(i) % 26);
            OUTBUF[i * 3] = a as u8 + b'a';
            OUTBUF[i * 3 + 1] = b as u8 + b'a';
        }

        return std::str::from_raw_parts(OUTBUF.as_ptr(), 38);
    }

    std::hint::unreachable_unchecked();
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
