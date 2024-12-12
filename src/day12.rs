pub fn part1(input: &str) -> u32 {}

pub fn part2(input: &str) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input12.txt");
    const TEST: &str = include_str!("../testdata/input12.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 1467094);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 881182);
    }

    #[test]
    fn test_a() {
        assert_eq!(part1(TEST), 1930);
    }

    #[test]
    fn test_b() {
        assert_eq!(part2(TEST), 1206);
    }
}
