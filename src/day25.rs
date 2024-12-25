unsafe fn inner_p1(input: &[u8]) -> u64 {
    0
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input25.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 3021);
    }
}
