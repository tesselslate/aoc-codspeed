#![allow(static_mut_refs)]

unsafe fn inner_p1(input: &[u8]) -> u64 {
    0
}

unsafe fn inner_p2(input: &[u8]) -> &'static str {
    static mut OUTBUF: [u8; 16] = [0; 16];

    std::str::from_utf8_unchecked(&OUTBUF[..0])
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

    const INPUT: &str = include_str!("../inputs/input18.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 280);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), "28,56");
    }
}
