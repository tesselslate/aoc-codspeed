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

macro_rules! recurse_impl {
    ($func_name: ident, $func_next: ident, $steps: literal) => {
        fn $func_name(memo: &mut Memo, stone: u64) -> u64 {
            if let Some(result) = memo.get(stone, $steps) {
                *result
            } else {
                let result = if stone == 0 {
                    $func_next(memo, 1)
                } else {
                    let log10 = fast_ilog10(stone);

                    if log10 % 2 == 1 {
                        let pow = 10u64.pow(log10 / 2 + 1);

                        let lhs = $func_next(memo, stone / pow);
                        let rhs = $func_next(memo, stone % pow);

                        lhs + rhs
                    } else {
                        $func_next(memo, stone * 2024)
                    }
                };

                memo.set(stone, $steps, result);
                result
            }
        }
    };
}

#[inline(always)]
fn stone_0(_: &mut Memo, _: u64) -> u64 {
    1
}

recurse_impl!(stone_1, stone_0, 1);
recurse_impl!(stone_2, stone_1, 2);
recurse_impl!(stone_3, stone_2, 3);
recurse_impl!(stone_4, stone_3, 4);
recurse_impl!(stone_5, stone_4, 5);
recurse_impl!(stone_6, stone_5, 6);
recurse_impl!(stone_7, stone_6, 7);
recurse_impl!(stone_8, stone_7, 8);
recurse_impl!(stone_9, stone_8, 9);
recurse_impl!(stone_10, stone_9, 10);
recurse_impl!(stone_11, stone_10, 11);
recurse_impl!(stone_12, stone_11, 12);
recurse_impl!(stone_13, stone_12, 13);
recurse_impl!(stone_14, stone_13, 14);
recurse_impl!(stone_15, stone_14, 15);
recurse_impl!(stone_16, stone_15, 16);
recurse_impl!(stone_17, stone_16, 17);
recurse_impl!(stone_18, stone_17, 18);
recurse_impl!(stone_19, stone_18, 19);
recurse_impl!(stone_20, stone_19, 20);
recurse_impl!(stone_21, stone_20, 21);
recurse_impl!(stone_22, stone_21, 22);
recurse_impl!(stone_23, stone_22, 23);
recurse_impl!(stone_24, stone_23, 24);
recurse_impl!(stone_25, stone_24, 25);
recurse_impl!(stone_26, stone_25, 26);
recurse_impl!(stone_27, stone_26, 27);
recurse_impl!(stone_28, stone_27, 28);
recurse_impl!(stone_29, stone_28, 29);
recurse_impl!(stone_30, stone_29, 30);
recurse_impl!(stone_31, stone_30, 31);
recurse_impl!(stone_32, stone_31, 32);
recurse_impl!(stone_33, stone_32, 33);
recurse_impl!(stone_34, stone_33, 34);
recurse_impl!(stone_35, stone_34, 35);
recurse_impl!(stone_36, stone_35, 36);
recurse_impl!(stone_37, stone_36, 37);
recurse_impl!(stone_38, stone_37, 38);
recurse_impl!(stone_39, stone_38, 39);
recurse_impl!(stone_40, stone_39, 40);
recurse_impl!(stone_41, stone_40, 41);
recurse_impl!(stone_42, stone_41, 42);
recurse_impl!(stone_43, stone_42, 43);
recurse_impl!(stone_44, stone_43, 44);
recurse_impl!(stone_45, stone_44, 45);
recurse_impl!(stone_46, stone_45, 46);
recurse_impl!(stone_47, stone_46, 47);
recurse_impl!(stone_48, stone_47, 48);
recurse_impl!(stone_49, stone_48, 49);
recurse_impl!(stone_50, stone_49, 50);
recurse_impl!(stone_51, stone_50, 51);
recurse_impl!(stone_52, stone_51, 52);
recurse_impl!(stone_53, stone_52, 53);
recurse_impl!(stone_54, stone_53, 54);
recurse_impl!(stone_55, stone_54, 55);
recurse_impl!(stone_56, stone_55, 56);
recurse_impl!(stone_57, stone_56, 57);
recurse_impl!(stone_58, stone_57, 58);
recurse_impl!(stone_59, stone_58, 59);
recurse_impl!(stone_60, stone_59, 60);
recurse_impl!(stone_61, stone_60, 61);
recurse_impl!(stone_62, stone_61, 62);
recurse_impl!(stone_63, stone_62, 63);
recurse_impl!(stone_64, stone_63, 64);
recurse_impl!(stone_65, stone_64, 65);
recurse_impl!(stone_66, stone_65, 66);
recurse_impl!(stone_67, stone_66, 67);
recurse_impl!(stone_68, stone_67, 68);
recurse_impl!(stone_69, stone_68, 69);
recurse_impl!(stone_70, stone_69, 70);
recurse_impl!(stone_71, stone_70, 71);
recurse_impl!(stone_72, stone_71, 72);
recurse_impl!(stone_73, stone_72, 73);
recurse_impl!(stone_74, stone_73, 74);
recurse_impl!(stone_75, stone_74, 75);

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
        if STEPS == 25 {
            sum += stone_25(&mut memo, stone);
        } else if STEPS == 75 {
            sum += stone_75(&mut memo, stone);
        }
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
