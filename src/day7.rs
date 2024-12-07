use memchr::memchr;

/*

TODO:
    parse_u64 and get_nums are slow af, it's like 60usec to process the whole file
    which is pretty bad

*/

const NUM_LIMIT: usize = 16;

fn parse_u64(b: &[u8]) -> u64 {
    b.iter().fold(0, |acc, &b| acc * 10 + (b & 0xF) as u64)
}

fn get_nums<'a>(l: &[u8], storage: &'a mut [u64; NUM_LIMIT]) -> (u64, &'a [u64]) {
    let colon = unsafe { memchr(b':', l).unwrap_unchecked() };
    let target = parse_u64(unsafe { l.get_unchecked(..colon) });

    let mut i = colon + 2;
    let mut j = 0;
    loop {
        match memchr(b' ', unsafe { l.get_unchecked(i..) }) {
            Some(x) => {
                storage[j] = parse_u64(unsafe { l.get_unchecked(i..i + x) });
                j += 1;
                i += x + 1;
            }
            None => {
                storage[j] = parse_u64(unsafe { l.get_unchecked(i..) });
                break;
            }
        }
    }

    (target, &storage[..j + 1])
}

fn backtrack(target: u64, nums: &[u64]) -> bool {
    let &last = unsafe { nums.last().unwrap_unchecked() };

    if nums.len() == 1 {
        target == last
    } else {
        let next = &nums[..nums.len() - 1];
        let overflow = target < last;

        if target % last == 0 {
            backtrack(target / last, next) || (!overflow && backtrack(target - last, next))
        } else if !overflow {
            backtrack(target - last, next)
        } else {
            false
        }
    }
}

fn process_p1(l: &str) -> u64 {
    let mut storage = [0u64; NUM_LIMIT];

    let (target, nums) = get_nums(l.as_bytes(), &mut storage);

    if backtrack(target, nums) {
        target
    } else {
        0
    }
}

pub fn part1(input: &str) -> u64 {
    input.lines().map(|l| process_p1(l)).sum()
}

pub fn part2(input: &str) -> u64 {
    0
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
        assert_eq!(part1(TEST), 3749);
    }

    #[test]
    fn test_b() {
        assert_eq!(part2(TEST), 11387);
    }
}
