pub fn part1(input: &str) -> u32 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { inner_p2(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input19.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 298);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 572248688842069);
    }
}
