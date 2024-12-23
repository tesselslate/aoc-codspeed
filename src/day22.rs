// shoutout incompleteusern
const MAGIC_4: [[u32; 16]; 6] = const {
    let mut bits = [0u32; 24];

    let mut i = 0;
    while i < 24 {
        bits[i] = hash_2k(1 << i);
        i += 1;
    }

    let mut lut = [[0; 16]; 6];

    let mut i = 0;
    while i < 6 {
        let mut j = 0;
        while j < 16 {
            if (j & 1) != 0 {
                lut[i][j] ^= bits[i * 4];
            }
            if (j & 2) != 0 {
                lut[i][j] ^= bits[i * 4 + 1];
            }
            if (j & 4) != 0 {
                lut[i][j] ^= bits[i * 4 + 2];
            }
            if (j & 8) != 0 {
                lut[i][j] ^= bits[i * 4 + 3];
            }
            j += 1;
        }
        i += 1;
    }

    lut
};

#[inline(always)]
const fn hash(mut x: u32) -> u32 {
    x = (x ^ (x << 6)) & 0xFFFFFF;
    x = x ^ (x >> 5);
    (x ^ (x << 11)) & 0xFFFFFF
}

const fn hash_2k(mut x: u32) -> u32 {
    let mut i = 0;
    while i < 2000 {
        x = hash(x);
        i += 1;
    }
    x
}

#[inline(always)]
unsafe fn skip_2k(x: u32) -> u32 {
    let mut s = 0;

    for i in 0..6 {
        s ^= MAGIC_4
            .get_unchecked(i)
            .get_unchecked(((x >> (i * 4)) & 0xF) as usize);
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

unsafe fn inner_p2(input: &[u8]) -> u32 {
    0
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> u32 {
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
