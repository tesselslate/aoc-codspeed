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

struct DiskB {
    file_id: [u16; LEN / 2 + 1],
    file_pos: [u16; LEN / 2 + 1],
    file_sz: [u8; LEN / 2 + 1],
    file_len: usize,

    free_pos: [u16; LEN / 2],
    free_sz: [u8; LEN / 2],
    free_len: usize,
}

impl Default for DiskB {
    fn default() -> Self {
        Self {
            file_id: [0; LEN / 2 + 1],
            file_pos: [0; LEN / 2 + 1],
            file_sz: [0; LEN / 2 + 1],
            file_len: 0,

            free_pos: [0; LEN / 2],
            free_sz: [0; LEN / 2],
            free_len: 0,
        }
    }
}

impl DiskB {
    pub fn push_file(&mut self, pos: usize, sz: u8, id: u16) {
        self.file_id[self.file_len] = id;
        self.file_pos[self.file_len] = pos as u16;
        self.file_sz[self.file_len] = sz;
        self.file_len += 1;
    }

    pub fn push_free(&mut self, pos: usize, sz: u8) {
        self.free_pos[self.free_len] = pos as u16;
        self.free_sz[self.free_len] = sz;
        self.free_len += 1;
    }
}

fn inner_p1<const LEN: usize>(input: &[u8]) -> u64 {
    let mut disk = DiskA::default();
    for i in 0..LEN {
        if i % 2 == 0 {
            for _ in 0..(input[i] & 0xF) {
                disk.push((i / 2) as u16);
            }
        } else {
            for _ in 0..(input[i] & 0xF) {
                disk.push(0);
            }
        }
    }

    let (mut i, mut j) = ((input[0] & 0xF) as usize, disk.len - 1);
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

fn inner_p2<const LEN: usize>(input: &[u8]) -> u64 {
    let mut disk = DiskB::default();

    let mut pos = 0;
    for i in 0..LEN {
        if i % 2 == 0 {
            disk.push_file(pos, input[i] & 0xF, (i / 2) as u16);
            pos += (input[i] & 0xF) as usize;
        } else {
            disk.push_free(pos, input[i] & 0xF);
            pos += (input[i] & 0xF) as usize;
        }
    }

    for i in (0..disk.file_len).rev() {
        let (pos, sz, id) = (disk.file_pos[i], disk.file_sz[i], disk.file_id[i]);

        for j in 0..disk.free_len {
            let (free_pos, free_sz) = (disk.free_pos[j], disk.free_sz[j]);
            if free_pos > pos {
                break;
            }

            if free_sz == sz {
                disk.file_pos[i] = free_pos;
                disk.file_sz[i] = sz;
                disk.file_id[i] = id;

                disk.free_sz[j] = 0;
                break;
            } else if free_sz > sz {
                disk.file_pos[i] = free_pos;
                disk.file_sz[i] = sz;
                disk.file_id[i] = id;

                disk.free_pos[j] = free_pos + sz as u16;
                disk.free_sz[j] = free_sz - sz;
                break;
            }
        }
    }

    (0..disk.file_len)
        .map(|i| {
            let mut x = 0;
            for j in disk.file_pos[i] as u64..disk.file_pos[i] as u64 + disk.file_sz[i] as u64 {
                x += j * disk.file_id[i] as u64;
            }
            x
        })
        .sum()
}

pub fn part1(input: &str) -> u64 {
    let input = input.as_bytes();
    inner_p1::<LEN>(input)
}

pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    inner_p2::<LEN>(input)
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

    #[test]
    fn test_b() {
        assert_eq!(inner_p2::<TEST_LEN>(TEST.as_bytes()), 2858);
    }
}
