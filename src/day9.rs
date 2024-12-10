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

struct FreeBuckets {
    buckets: [[u16; LEN / 2]; 10],
    read: [usize; 10],
    write: [usize; 10],
}

impl Default for FreeBuckets {
    fn default() -> Self {
        Self {
            buckets: [[0; LEN / 2]; 10],
            read: [0; 10],
            write: [0; 10],
        }
    }
}

impl FreeBuckets {
    pub fn add(&mut self, sz: u8, pos: usize) {
        let sz = sz as usize;

        self.buckets[sz][self.write[sz]] = pos as u16;
        self.write[sz] += 1;
    }

    pub fn get(&mut self, sz: u8) -> Option<u16> {
        let sz = sz as usize;

        if self.read[sz] == self.write[sz] {
            None
        } else {
            let pos = self.buckets[sz][self.read[sz]];
            self.read[sz] += 1;
            Some(pos)
        }
    }

    pub fn peek(&self, sz: u8) -> Option<u16> {
        let sz = sz as usize;

        if self.read[sz] == self.write[sz] {
            None
        } else {
            Some(self.buckets[sz][self.read[sz]])
        }
    }

    pub fn insert(&mut self, sz: u8, pos: u16) {
        let sz = sz as usize;

        // TODO: binary search? are the buckets big enough
        self.buckets[sz][self.write[sz]] = pos;
        self.write[sz] += 1;
        self.buckets[sz][self.read[sz]..self.write[sz]].sort_unstable();
    }
}

struct Files {
    pos: [u16; LEN / 2 + 1],
    sz: [u8; LEN / 2 + 1],
    len: usize,
}

impl Default for Files {
    fn default() -> Self {
        Self {
            pos: [0; LEN / 2 + 1],
            sz: [0; LEN / 2 + 1],
            len: 0,
        }
    }
}

impl Files {
    pub fn get(&self, index: usize) -> (u16, u8) {
        (self.pos[index], self.sz[index])
    }

    pub fn set(&mut self, index: usize, pos: u16, sz: u8) {
        self.pos[index] = pos;
        self.sz[index] = sz;
    }

    pub fn push(&mut self, pos: u16, sz: u8) {
        self.pos[self.len] = pos;
        self.sz[self.len] = sz;
        self.len += 1;
    }
}

fn inner_p1<const LEN: usize>(input: &[u8]) -> u64 {
    let input = &input[..LEN];

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
    let input = &input[..LEN];

    let mut free = FreeBuckets::default();
    let mut files = Files::default();

    let mut pos = 0;
    for i in 0..LEN {
        if i % 2 == 0 {
            files.push(pos as u16, input[i] & 0xF);
        } else {
            free.add(input[i] & 0xF, pos);
        }

        pos += (input[i] & 0xF) as usize;
    }

    for i in (0..files.len).rev() {
        let (pos, sz) = files.get(i);

        let mut min_sz = 0;
        let mut min_pos = u16::MAX;
        for try_sz in sz..=9 {
            if let Some(free_pos) = free.peek(try_sz) {
                if free_pos < min_pos {
                    min_pos = free_pos;
                    min_sz = try_sz;
                }
            }
        }

        if min_sz == 0 || min_pos > pos {
            continue;
        }

        let free_pos = free.get(min_sz).unwrap();
        files.set(i, free_pos, sz);
        free.insert(sz, pos);
        if min_sz > sz {
            free.insert(min_sz - sz, free_pos + sz as u16);
        }
    }

    let mut sum = 0;
    for i in 0..files.len {
        let (pos, sz) = files.get(i);
        for j in pos as u64..pos as u64 + sz as u64 {
            sum += i as u64 * j;
        }
    }
    sum
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
