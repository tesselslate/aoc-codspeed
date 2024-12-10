pub fn part1(input: &str) -> u64 {}

pub fn part2(input: &str) -> u64 {}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input9.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 6432869891895);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 6467290479134);
    }
}
