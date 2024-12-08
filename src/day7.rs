use std::num::NonZeroU64;

use memchr::memchr;

/*

TODO:
    parse_u64 and get_nums are slow af, it's like 60usec to process the whole file
    which is pretty bad

    use inline asm for ja_unconcat
*/

const NUM_LIMIT: usize = 12;

unsafe fn parse_u64(b: &[u8]) -> u64 {
    b.iter().fold(0, |acc, &b| acc * 10 + (b & 0xF) as u64)
}

unsafe fn get_nums(l: &[u8], storage: &mut [u64; NUM_LIMIT]) -> (u64, *const u64, *const u64) {
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

    (target, storage.as_ptr(), storage.as_ptr().add(j))
}

unsafe fn backtrack(mut target: u64, start: *const u64, mut end: *const u64) -> bool {
    while start != end {
        let b = NonZeroU64::try_from(*end).unwrap_unchecked();

        let (div, rem) = (target / b.get(), target % b.get());
        if rem == 0 && backtrack(div, start, end.sub(1)) {
            return true;
        }

        let sub = target.wrapping_sub(b.get());
        if sub > target {
            return false;
        }

        target = sub;
        end = end.sub(1);
    }

    *start == target
}

unsafe fn process_p1(l: &[u8], storage: &mut [u64; NUM_LIMIT]) -> u64 {
    let (target, start, end) = get_nums(l, storage);

    if backtrack(target, start, end) {
        target
    } else {
        0
    }
}

unsafe fn process_p2(l: &[u8], storage: &mut [u64; NUM_LIMIT]) -> u64 {
    0
    // let (target, nums) = get_nums(l, storage);
    //
    // if backtrack_concat(target, nums) {
    //     target
    // } else {
    //     0
    // }
}

unsafe fn inner_p1(input: &str) -> u64 {
    let mut storage = [0u64; NUM_LIMIT];

    let bytes = input.as_bytes();
    let mut sum = 0;

    let mut i = 0;
    loop {
        match memchr(b'\n', bytes.get_unchecked(i..)) {
            Some(j) => {
                sum += process_p1(bytes.get_unchecked(i..i + j), &mut storage);
                i += j + 1;
            }
            None => {
                sum += process_p1(bytes.get_unchecked(i..), &mut storage);
                return sum;
            }
        }
    }
}

unsafe fn inner_p2(input: &str) -> u64 {
    let mut storage = [0u64; NUM_LIMIT];

    let bytes = input.as_bytes();
    let mut sum = 0;

    let mut i = 0;
    unsafe {
        loop {
            match memchr(b'\n', bytes.get_unchecked(i..)) {
                Some(j) => {
                    sum += process_p2(bytes.get_unchecked(i..i + j), &mut storage);
                    i += j + 1;
                }
                None => {
                    sum += process_p2(bytes.get_unchecked(i..), &mut storage);
                    return sum;
                }
            }
        }
    }
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { inner_p2(input) }
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
