unsafe fn inner_p1(input: &[u8]) -> u64 {
    0
}

unsafe fn inner_p2(input: &[u8]) -> &'static str {
    ""
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> &'static str {
    unsafe { inner_p2(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input24.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 53755311654662);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), "dkr,ggk,hhh,htp,rhv,z05,z15,z20");
    }
}
