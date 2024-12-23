#![allow(static_mut_refs)]

unsafe fn inner_p1(input: &[u8]) -> u64 {
    0
}

unsafe fn inner_p2(input: &[u8]) -> &'static str {
    static mut OUTBUF: [u8; 64] = [0; 64];

    std::str::from_raw_parts(OUTBUF.as_ptr(), 0)
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

    const INPUT: &str = include_str!("../inputs/input23.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 1083);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), "as,bu,cp,dj,ez,fd,hu,it,kj,nx,pp,xh,yu");
    }
}
