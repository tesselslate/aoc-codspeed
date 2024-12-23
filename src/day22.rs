#![allow(static_mut_refs)]

use fastdiv::{FastDiv, PrecomputedDivU32};

const SEQ_COUNT: usize = 19 * 19 * 19 * 19;

const MAGIC2000: [[u32; 256]; 3] = create_magic::<2000>();

// shoutout incompleteusern
const fn create_magic<const ITERATIONS: usize>() -> [[u32; 256]; 3] {
    let mut bits = [0u32; 24];

    let mut i = 0;
    while i < 24 {
        let mut j = 0;
        let mut x = 1 << i;
        while j < ITERATIONS {
            x = hash(x);
            j += 1;
        }
        bits[i] = x;
        i += 1;
    }

    let mut lut = [[0; 256]; 3];

    let mut i = 0;
    while i < 3 {
        let mut j = 0;
        while j < 256 {
            if (j & 1) != 0 {
                lut[i][j] ^= bits[i * 8];
            }
            if (j & 2) != 0 {
                lut[i][j] ^= bits[i * 8 + 1];
            }
            if (j & 4) != 0 {
                lut[i][j] ^= bits[i * 8 + 2];
            }
            if (j & 8) != 0 {
                lut[i][j] ^= bits[i * 8 + 3];
            }
            if (j & 16) != 0 {
                lut[i][j] ^= bits[i * 8 + 4];
            }
            if (j & 32) != 0 {
                lut[i][j] ^= bits[i * 8 + 5];
            }
            if (j & 64) != 0 {
                lut[i][j] ^= bits[i * 8 + 6];
            }
            if (j & 128) != 0 {
                lut[i][j] ^= bits[i * 8 + 7];
            }
            j += 1;
        }
        i += 1;
    }

    lut
}

#[inline(always)]
const fn hash(mut x: u32) -> u32 {
    x = (x ^ (x << 6)) & 0xFFFFFF;
    x = x ^ (x >> 5);
    (x ^ (x << 11)) & 0xFFFFFF
}

#[inline(always)]
unsafe fn skip_2k(x: u32) -> u32 {
    let mut s = 0;

    for i in 0..3 {
        s ^= MAGIC2000
            .get_unchecked(i)
            .get_unchecked(((x >> (i * 8)) & 0xFF) as usize);
    }

    s
}

unsafe fn inner_p1(input: &[u8]) -> u64 {
    let mut sum = 0;

    let input_end = input.as_ptr_range().end.sub(1);
    let mut input = input.as_ptr();

    loop {
        let mut acc = 0;
        while *input != b'\n' {
            acc = acc * 10 + (*input - b'0') as u32;
            input = input.add(1);
        }

        sum += skip_2k(acc) as u64;
        if input == input_end {
            return sum;
        }

        input = input.add(1);
    }
}

unsafe fn inner_p2(input: &[u8]) -> i16 {
    let d10: PrecomputedDivU32 = 10u32.precompute_div();

    static mut VALUE: [i16; SEQ_COUNT] = [0; SEQ_COUNT];
    static mut SEEN: [u16; SEQ_COUNT] = [0; SEQ_COUNT];

    VALUE.iter_mut().for_each(|x| *x = 0);
    SEEN.iter_mut().for_each(|x| *x = 0);

    let mut best = 0;

    let input_end = input.as_ptr_range().end.sub(1);
    let mut input = input.as_ptr();

    let mut secret_id = 1;
    let (mut a, mut b, mut c, mut d);
    loop {
        let mut secret = 0;
        while *input != b'\n' {
            secret = secret * 10 + (*input - b'0') as u32;
            input = input.add(1);
        }

        let next = hash(secret);
        a = 9 + next.fast_mod(d10, 10) - secret.fast_mod(d10, 10);
        secret = next;

        let next = hash(secret);
        b = 9 + next.fast_mod(d10, 10) - secret.fast_mod(d10, 10);
        secret = next;

        let next = hash(secret);
        c = 9 + next.fast_mod(d10, 10) - secret.fast_mod(d10, 10);
        secret = next;

        let next = hash(secret);
        d = 9 + next.fast_mod(d10, 10) - secret.fast_mod(d10, 10);
        secret = next;

        for _ in 0..1996 {
            let seq_id = (a * 19 * 19 * 19 + b * 19 * 19 + c * 19 + d) as usize;

            if *SEEN.get_unchecked(seq_id) < secret_id {
                *SEEN.get_unchecked_mut(seq_id) = secret_id;
                *VALUE.get_unchecked_mut(seq_id) += secret.fast_mod(d10, 10) as i16;
                best = i16::max(best, *VALUE.get_unchecked(seq_id));
            }

            let next = hash(secret);
            (a, b, c, d) = (
                b,
                c,
                d,
                9 + next.fast_mod(d10, 10) - secret.fast_mod(d10, 10),
            );
            secret = next;
        }

        if input == input_end {
            return best;
        }

        input = input.add(1);
        secret_id += 1;
    }
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> i16 {
    unsafe { inner_p2(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input22.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 13584398738);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 1612);
    }
}
