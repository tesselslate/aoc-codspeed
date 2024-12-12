use std::collections::HashMap;

use rustc_hash::FxBuildHasher;

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

fn calculate(memo: &mut Memo, stone: u64, steps: usize) -> u64 {
    if let Some(result) = memo.get(stone, steps) {
        *result
    } else if steps == 0 {
        1
    } else {
        let result = if stone == 0 {
            calculate(memo, 1, steps - 1)
        } else {
            let log10 = fast_ilog10(stone);

            if log10 % 2 == 1 {
                let pow = 10u64.pow(log10 / 2 + 1);

                let lhs = calculate(memo, stone / pow, steps - 1);
                let rhs = calculate(memo, stone % pow, steps - 1);

                lhs + rhs
            } else {
                calculate(memo, stone * 2024, steps - 1)
            }
        };

        memo.set(stone, steps, result);
        result
    }
}

fn calculate_outer<const STEPS: usize>(input: &str) -> u64 {
    let input = input.as_bytes();
    let end = memchr::memchr(b'\n', input).unwrap_or(input.len());

    let mut stones = [0u64; 16];
    let mut num_stones = 0;
    let mut pos = 0;

    for delim in memchr::memchr_iter(b' ', input) {
        let bytes = &input[pos..delim];

        stones[num_stones] = parse_num(bytes);
        num_stones += 1;

        pos = delim + 1;
    }

    stones[num_stones] = parse_num(&input[pos..end]);
    num_stones += 1;

    let expect_cap = if STEPS == 25 { 6400 } else { 160000 };

    let mut memo = Memo::new(expect_cap);
    let mut sum = 0;
    for &stone in &stones[..num_stones] {
        sum += calculate(&mut memo, stone, STEPS);
    }

    sum
}

fn parse_num(b: &[u8]) -> u64 {
    b.iter().fold(0, |acc, x| acc * 10 + (x - b'0') as u64)
}

pub fn part1(input: &str) -> u64 {
    calculate_outer::<25>(input)
}

pub fn part2(input: &str) -> u64 {
    calculate_outer::<75>(input)
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
