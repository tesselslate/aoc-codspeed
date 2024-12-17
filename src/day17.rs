#![allow(static_mut_refs)]

unsafe fn inner_p1(input: &[u8]) -> &'static str {
    static mut OUTBUF: [u8; 64] = [0; 64];

    std::str::from_utf8_unchecked(&OUTBUF[..0])
}

pub fn part1(input: &str) -> &'static str {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input17.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), "7,0,3,1,2,6,3,7,1");
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 109020013201563);
    }
}
