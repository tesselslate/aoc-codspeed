use std::collections::HashMap;

use rustc_hash::FxBuildHasher;

const LUT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/day11_lut.bin"));
const LUT_STONE_COUNT: u64 = 10000000;

struct Memo {
    data: HashMap<(u64, usize), u64, FxBuildHasher>,
}

impl Memo {
    pub fn new(cap: usize) -> Self {
        Self {
            data: HashMap::with_capacity_and_hasher(cap, FxBuildHasher::default()),
        }
    }

    #[inline]
    pub fn get(&mut self, stone: u64, steps: usize) -> Option<&u64> {
        self.data.get(&(stone, steps))
    }

    #[inline]
    pub fn set(&mut self, stone: u64, steps: usize, substones: u64) {
        self.data.insert((stone, steps), substones);
    }
}

// https://da-data.blogspot.com/2023/02/integer-log10-in-rust-and-c.html
// adapted for u64
#[inline]
fn fast_ilog10(x: u64) -> u32 {
    #[rustfmt::skip]
    const LUT: [u64; 15] = [
        9,                  99,                 999,
        9_999,              99_999,             999_999,
        9_999_999,          99_999_999,         999_999_999,
        9_999_999_999,      99_999_999_999,     999_999_999_999,
        9_999_999_999_999,  99_999_999_999_999, 999_999_999_999_999,
    ];

    const MASK: u64 = 0b0001001001000100100100010010010001001001000100100100010010010000;

    let guess = (MASK << x.leading_zeros()).count_ones() as usize;
    guess as u32 + (x > LUT[guess]) as u32
}

fn calculate_inner(memo: &mut Memo, stone: u64, steps: usize) -> u64 {
    if let Some(result) = memo.get(stone, steps) {
        *result
    } else if steps == 0 {
        1
    } else {
        let result = if stone == 0 {
            calculate_inner(memo, 1, steps - 1)
        } else {
            let log10 = fast_ilog10(stone);

            if log10 % 2 == 1 {
                let pow = 10u64.pow(log10 / 2 + 1);

                let lhs = calculate_inner(memo, stone / pow, steps - 1);
                let rhs = calculate_inner(memo, stone % pow, steps - 1);

                lhs + rhs
            } else {
                calculate_inner(memo, stone * 2024, steps - 1)
            }
        };

        memo.set(stone, steps, result);
        result
    }
}

fn lut_lookup<const P1: bool>(stone: u64) -> Option<u64> {
    if stone >= LUT_STONE_COUNT {
        None
    } else {
        let mut offset = if P1 { 0 } else { LUT_STONE_COUNT as usize * 8 };
        offset += stone as usize * 8;

        Some(u64::from_ne_bytes(
            unsafe { LUT.get_unchecked(offset..offset + 8) }
                .try_into()
                .unwrap(),
        ))
    }
}

fn process_stone<const P1: bool>(memo: &mut Option<Memo>, input: &[u8]) -> u64 {
    let stone = parse_num(input);

    if let Some(count) = lut_lookup::<P1>(stone) {
        count
    } else {
        if memo.is_none() {
            *memo = Some(Memo::new(if P1 { 6400 } else { 160000 }));
        }

        calculate_inner(&mut memo.as_mut().unwrap(), stone, if P1 { 25 } else { 75 })
    }
}

fn calculate<const P1: bool>(input: &str) -> u64 {
    let input = input.as_bytes();
    let end = memchr::memchr(b'\n', input).unwrap_or(input.len());

    let mut memo = None; // Memo::new(if P1 { 6400 } else { 160000 });

    let mut pos = 0;
    let mut sum = 0;
    for delim in memchr::memchr_iter(b' ', input) {
        sum += process_stone::<P1>(&mut memo, &input[pos..delim]);
        pos = delim + 1;
    }

    sum += process_stone::<P1>(&mut memo, &input[pos..end]);

    sum
}

fn parse_num(b: &[u8]) -> u64 {
    b.iter().fold(0, |acc, x| acc * 10 + (x - b'0') as u64)
}

pub fn part1(input: &str) -> u64 {
    calculate::<true>(input)
}

pub fn part2(input: &str) -> u64 {
    calculate::<false>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input11.txt");
    const TEST: &str = include_str!("../testdata/input11.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 216042);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 255758646442399);
    }

    #[test]
    fn test_a() {
        assert_eq!(part1(TEST), 55312);
    }

    #[test]
    fn test_b() {
        assert_eq!(part2(TEST), 65601038650482);
    }
}
