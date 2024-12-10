use std::{ops::Index, u16};

const LEN: usize = 19999;
const SZ: usize = LEN * 9;

struct DiskA {
    data: [u16; SZ],
    len: usize,
}

impl Default for DiskA {
    fn default() -> Self {
        Self {
            data: [0; SZ],
            len: 0,
        }
    }
}

impl DiskA {
    pub fn push(&mut self, item: u16) {
        self.data[self.len] = item;
        self.len += 1;
    }

    pub fn swap(&mut self, i: usize, j: usize) {
        self.data.swap(i, j);
    }
}

impl Index<usize> for DiskA {
    type Output = u16;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

fn inner_p1<const LEN: usize>(input: &[u8]) -> u64 {
    let mut disk = DiskA::default();
    for i in 0..LEN {
        if i % 2 == 0 {
            for _ in 0..(input[i] - b'0') {
                disk.push((i / 2) as u16);
            }
        } else {
            for _ in 0..(input[i] - b'0') {
                disk.push(0);
            }
        }
    }

    let (mut i, mut j) = ((input[0] - b'0') as usize, disk.len - 1);
    loop {
        while disk[i] != 0 {
            i += 1;
        }
        while disk[j] == 0 {
            j -= 1;
        }
        if i > j {
            break;
        }

        disk.swap(i, j);
    }

    disk.data[..=j]
        .iter()
        .enumerate()
        .map(|(i, &x)| i as u64 * x as u64)
        .sum()
}

pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    inner_p1::<LEN>(input)
}

pub fn part2(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input9.txt");
    const TEST: &str = include_str!("../testdata/input9.txt");
    const TEST_LEN: usize = TEST.as_bytes().len() - 1;

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 6432869891895);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 6467290479134);
    }

    #[test]
    fn test_a() {
        assert_eq!(inner_p1::<TEST_LEN>(TEST.as_bytes()), 1928);
    }
}
