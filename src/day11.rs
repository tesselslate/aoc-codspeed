const LUT: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/day11_lut.bin"));
const LUT_STONE_COUNT: u64 = 10000000;

fn lut_lookup<const P1: bool>(stone: u64) -> Option<u64> {
    if stone >= LUT_STONE_COUNT {
        None
    } else {
        let mut offset = if P1 { 0 } else { LUT_STONE_COUNT as usize * 8 };
        offset += stone as usize * 8;

        Some(u64::from_ne_bytes(
            LUT[offset..offset + 8].try_into().unwrap(),
        ))
    }
}

fn calculate<const P1: bool>(input: &str) -> u64 {
    let input = &input[..input.find('\n').unwrap_or(input.len())];

    let mut sum = 0;
    input.split(' ').for_each(|x| {
        let stone = x.parse::<u64>().unwrap();
        if let Some(count) = lut_lookup::<P1>(stone) {
            sum += count;
        }
    });

    sum
}

pub fn part1(input: &str) -> u64 {
    calculate::<true>(input)
}

pub fn part2(input: &str) -> u64 {
    calculate::<false>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input11.txt");
    const TEST: &str = include_str!("../testdata/input11.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 216042);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 255758646442399);
    }

    #[test]
    fn test_a() {
        assert_eq!(part1(TEST), 55312);
    }

    #[test]
    fn test_b() {
        assert_eq!(part2(TEST), 65601038650482);
    }
}
