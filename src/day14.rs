pub fn part1(input: &str) -> u64 {
    0
}

pub fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input14.txt");
    const TEST: &str = include_str!("../testdata/input14.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 228457125);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 6493);
    }

    #[test]
    fn test_a() {
        assert_eq!(part1(TEST), 12);
    }
}
