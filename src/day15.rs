pub fn part1(input: &str) -> u32 {}

pub fn part2(input: &str) -> u32 {}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input15.txt");
    const TEST_1: &str = include_str!("../testdata/input15_a.txt");
    const TEST_2: &str = include_str!("../testdata/input15_b.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 1568399);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 1575877);
    }

    #[test]
    fn test_a1() {
        assert_eq!(part1(TEST_1), 10092);
    }

    #[test]
    fn test_a2() {
        assert_eq!(part1(TEST_2), 2028);
    }

    #[test]
    fn test_b1() {
        assert_eq!(part2(TEST_1), 9021);
    }
}
