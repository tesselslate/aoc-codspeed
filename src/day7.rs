use std::{hint::unreachable_unchecked, intrinsics::assume};

use memchr::memchr;

/*

TODO:
    parse_u64 and get_nums are slow af, it's like 60usec to process the whole file
    which is pretty bad

    use inline asm for ja_unconcat
*/

const NUM_LIMIT: usize = 12;

fn parse_u64(b: &[u8]) -> u64 {
    b.iter().fold(0, |acc, &b| acc * 10 + (b & 0xF) as u64)
}

fn get_nums<'a>(l: &[u8], storage: &'a mut [u64; NUM_LIMIT]) -> (u64, &'a [u64]) {
    unsafe {
        let colon = memchr(b':', l).unwrap_unchecked();
        let target = parse_u64(l.get_unchecked(..colon));

        let mut i = colon + 2;
        let mut j = 0;
        loop {
            match memchr(b' ', l.get_unchecked(i..)) {
                Some(x) => {
                    *storage.get_unchecked_mut(j) = parse_u64(l.get_unchecked(i..i + x));
                    j += 1;
                    i += x + 1;
                }
                None => {
                    *storage.get_unchecked_mut(j) = parse_u64(l.get_unchecked(i..));
                    break;
                }
            }
        }

        (target, storage.get_unchecked(..j + 1))
    }
}

fn unconcat(have: u64, concat: u64) -> Option<u64> {
    // if have ends with concat:
    //   Some( have without the concat digits )
    // else:
    //   None
    //
    // have ends with concat IF:
    //   (have % 10^int(log10(concat))) == concat

    let modulo = match concat {
        ..10 => 10,
        ..100 => 100,
        ..1000 => 1000,
        _ => unsafe { unreachable_unchecked() },
    };

    if have % modulo == concat {
        Some(have / modulo)
    } else {
        None
    }
}

fn backtrack(target: u64, nums: &[u64]) -> bool {
    let &last = unsafe { nums.last().unwrap_unchecked() };

    if nums.len() == 1 {
        target == last
    } else {
        let next = unsafe { nums.get_unchecked(..nums.len() - 1) };

        if target % last == 0 && backtrack(target / last, next) {
            return true;
        }
        if target >= last && backtrack(target - last, next) {
            return true;
        }

        return false;
    }
}

fn backtrack_concat(target: u64, nums: &[u64]) -> bool {
    let &last = unsafe { nums.last().unwrap_unchecked() };

    if nums.len() == 1 {
        target == last
    } else {
        let next = unsafe { nums.get_unchecked(..nums.len() - 1) };

        if let Some(x) = unconcat(target, last)
            && backtrack_concat(x, next)
        {
            return true;
        }
        if target % last == 0 && backtrack_concat(target / last, next) {
            return true;
        }
        if target >= last && backtrack_concat(target - last, next) {
            return true;
        }

        return false;
    }
}

fn process_p1(l: &[u8]) -> u64 {
    let mut storage = [0u64; NUM_LIMIT];

    let (target, nums) = get_nums(l, &mut storage);

    if backtrack(target, nums) {
        target
    } else {
        0
    }
}

fn process_p2(l: &[u8]) -> u64 {
    let mut storage = [0u64; NUM_LIMIT];

    let (target, nums) = get_nums(l, &mut storage);

    if backtrack_concat(target, nums) {
        target
    } else {
        0
    }
}

pub fn part1(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let mut sum = 0;

    let mut i = 0;
    unsafe {
        loop {
            match memchr(b'\n', bytes.get_unchecked(i..)) {
                Some(j) => {
                    sum += process_p1(bytes.get_unchecked(i..i + j));
                    i += j + 1;
                }
                None => {
                    sum += process_p1(bytes.get_unchecked(i..));
                    return sum;
                }
            }
        }
    }
}

pub fn part2(input: &str) -> u64 {
    let bytes = input.as_bytes();
    let mut sum = 0;

    let mut i = 0;
    unsafe {
        loop {
            match memchr(b'\n', bytes.get_unchecked(i..)) {
                Some(j) => {
                    sum += process_p2(bytes.get_unchecked(i..i + j));
                    i += j + 1;
                }
                None => {
                    sum += process_p2(bytes.get_unchecked(i..));
                    return sum;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input7.txt");
    const TEST: &str = include_str!("../testdata/input7.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 4122618559853);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 227615740238334);
    }

    #[test]
    fn test_a() {
        assert_eq!(part1(&TEST[..TEST.len() - 1]), 3749);
    }

    #[test]
    fn test_b() {
        assert_eq!(part2(&TEST[..TEST.len() - 1]), 11387);
    }
}
