fn calculate<const STEPS: usize>(input: &str) -> u64 {
    0
}

pub fn part1(input: &str) -> u64 {
    calculate::<25>(input)
}

pub fn part2(input: &str) -> u64 {
    calculate::<75>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input11.txt");
    const TEST: &str = include_str!("../testdata/input11.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 733);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 1514);
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
