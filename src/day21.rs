use std::arch::x86_64::_pext_u32;

const LUT_RAW: &[u8] = include_bytes!("../lut/day21.bin");

fn lut_lookup_p1(code: usize) -> u64 {
    let offset = code * 8;

    u64::from_ne_bytes(unsafe {
        LUT_RAW
            .get_unchecked(offset..offset + 8)
            .try_into()
            .unwrap_unchecked()
    })
}

fn lut_lookup_p2(code: usize) -> u64 {
    let offset = code * 8;

    u64::from_ne_bytes(unsafe {
        LUT_RAW
            .get_unchecked(offset + 32768..offset + 32768 + 8)
            .try_into()
            .unwrap_unchecked()
    })
}

#[inline]
unsafe fn pext_bits(code: &[u8]) -> usize {
    let u32 = u32::from_ne_bytes(code.get_unchecked(0..4).try_into().unwrap_unchecked());
    unsafe { _pext_u32(u32, 0x000F0F0F) as usize }
}

unsafe fn inner_p1(input: &[u8]) -> u64 {
    lut_lookup_p1(pext_bits(input))
        + lut_lookup_p1(pext_bits(input.get_unchecked(5..)))
        + lut_lookup_p1(pext_bits(input.get_unchecked(10..)))
        + lut_lookup_p1(pext_bits(input.get_unchecked(15..)))
        + lut_lookup_p1(pext_bits(input.get_unchecked(20..)))
}

unsafe fn inner_p2(input: &[u8]) -> u64 {
    lut_lookup_p2(pext_bits(input))
        + lut_lookup_p2(pext_bits(input.get_unchecked(5..)))
        + lut_lookup_p2(pext_bits(input.get_unchecked(10..)))
        + lut_lookup_p2(pext_bits(input.get_unchecked(15..)))
        + lut_lookup_p2(pext_bits(input.get_unchecked(20..)))
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { inner_p2(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input21.txt");
    const TEST: &str = include_str!("../testdata/input21.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 177814);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 220493992841852);
    }

    #[test]
    fn test_a() {
        assert_eq!(part1(TEST), 126384);
    }

    #[test]
    fn test_b() {
        assert_eq!(part2(TEST), 154115708116294);
    }
}
