pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    0
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input10.txt");
    const TEST: &str = include_str!("../testdata/input10.txt");

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
        assert_eq!(part1(TEST), 36);
    }

    #[test]
    fn test_b() {
        assert_eq!(part2(TEST), 81);
    }
}
