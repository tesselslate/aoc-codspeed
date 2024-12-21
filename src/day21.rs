unsafe fn inner_p1(input: &[u8]) -> u32 {
    0
}

unsafe fn inner_p2(input: &[u8]) -> u64 {
    0
}

pub fn part1(input: &str) -> u32 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { inner_p2(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input21.txt");
    const TEST: &str = include_str!("../testdata/input21.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 177814);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 220493992841852);
    }

    #[test]
    fn test_a() {
        assert_eq!(part1(TEST), 126384);
    }

    #[test]
    fn test_b() {
        assert_eq!(part2(TEST), 154115708116294);
    }
}
