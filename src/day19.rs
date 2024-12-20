#![allow(static_mut_refs)]

const NUM_PATTERNS: usize = 400;
const TRIE_TERMINAL: usize = 1;
const TRIE_SIZE: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq)]
struct UnsafeSlice {
    data: *const u8,
    len: usize,
}

#[inline]
#[target_feature(enable = "popcnt")]
unsafe fn phf(x: u8) -> usize {
    (x.count_ones() as u8 - (x & 0b10) - 1) as usize
}

struct Trie {
    free: usize,
    data: [usize; 16384],
}

impl Trie {
    pub const fn new() -> Self {
        Self {
            free: TRIE_SIZE,
            data: [0; 16384],
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.free = TRIE_SIZE;
        self.data[0..TRIE_SIZE].iter_mut().for_each(|x| *x = 0);
    }

    #[inline]
    pub unsafe fn insert(&mut self, pattern: UnsafeSlice) {
        let mut node_index = 0;

        for i in 0..pattern.len {
            let char = *pattern.data.add(i);
            let idx = node_index + phf(char);
            let next = *self.data.get_unchecked(idx);

            if next > 0 {
                node_index = next as usize;
            } else {
                *self.data.get_unchecked_mut(idx) = self.free;
                node_index = self.free;

                self.data
                    .get_unchecked_mut(self.free..self.free + TRIE_SIZE)
                    .iter_mut()
                    .for_each(|x| *x = 0);

                self.free += TRIE_SIZE;
            }
        }

        *self.data.get_unchecked_mut(node_index + TRIE_SIZE - 1) = 1;
    }
}

#[inline]
unsafe fn parse_towels(mut input: *const u8, trie: &mut Trie) -> *const u8 {
    trie.clear();

    loop {
        let start = input;
        while *input != b',' {
            input = input.add(1);
            if *input == b'\n' {
                trie.insert(UnsafeSlice {
                    data: start,
                    len: input.sub_ptr(start),
                });
                return input;
            }
        }

        trie.insert(UnsafeSlice {
            data: start,
            len: input.sub_ptr(start),
        });
        input = input.add(2);
    }
}

unsafe fn dfs_p1(trie: &Trie, cache: *mut u64, start: *const u8, mut offset: usize) -> usize {
    let cache_idx = offset;
    if (*cache & (1 << cache_idx)) != 0 {
        return 0;
    }

    let mut trie_node = 0;
    loop {
        let char = *start.add(offset);
        if char == b'\n' {
            if *trie.data.get_unchecked(trie_node + TRIE_SIZE - 1) == TRIE_TERMINAL {
                return offset;
            } else {
                return 0;
            }
        }

        let idx = trie_node + phf(char);
        let next = *trie.data.get_unchecked(idx);

        if next == 0 {
            *cache |= 1 << cache_idx;
            return 0;
        }

        if *trie.data.get_unchecked(next as usize + TRIE_SIZE - 1) == TRIE_TERMINAL {
            let ret = dfs_p1(trie, cache, start, offset + 1);
            if ret > 0 {
                return ret;
            }
        }

        trie_node = next as usize;
        offset += 1;
    }
}

unsafe fn dfs_p2(trie: &Trie, cache: &mut [u64; 64], start: *const u8, mut offset: usize) -> u64 {
    let cache_idx = offset;
    if cache[cache_idx] != u64::MAX {
        return cache[cache_idx];
    }

    let mut sum = 0;
    let mut trie_node = 0;
    loop {
        let char = *start.add(offset);
        if char == b'\n' {
            debug_assert!(cache[cache_idx] == u64::MAX);

            let value = if *trie.data.get_unchecked(trie_node as usize + TRIE_SIZE - 1) == TRIE_TERMINAL {
                sum + 1
            } else {
                sum
            };

            cache[cache_idx] = value;
            return value;
        }

        let idx = trie_node + phf(char);
        let next = *trie.data.get_unchecked(idx);

        if next == 0 {
            debug_assert!(cache[cache_idx] == u64::MAX);

            cache[cache_idx] = sum;
            return sum;
        }

        if *trie.data.get_unchecked(next as usize + TRIE_SIZE - 1) == TRIE_TERMINAL {
            sum += dfs_p2(trie, cache, start, offset + 1);
        }

        trie_node = next as usize;
        offset += 1;
    }
}

unsafe fn inner_p1(input: &[u8]) -> u32 {
    static mut TOWELS: Trie = Trie::new();

    let mut input = input.as_ptr();
    input = parse_towels(input, &mut TOWELS);
    input = input.add(2);

    let mut valid = 0;

    for _ in 0..NUM_PATTERNS {
        let mut cache = 0u64;
        let next = dfs_p1(&TOWELS, std::ptr::from_mut(&mut cache), input, 0);

        if next > 0 {
            input = input.add(next + 1);

            valid += 1;
        } else {
            while *input != b'\n' {
                input = input.add(1);
            }
            input = input.add(1);
        }
    }

    valid
}

unsafe fn inner_p2(input: &[u8]) -> u64 {
    static mut TOWELS: Trie = Trie::new();

    let mut input = input.as_ptr();
    input = parse_towels(input, &mut TOWELS);
    input = input.add(2);

    let mut valid = 0;

    for _ in 0..NUM_PATTERNS {
        let mut cache = [u64::MAX; 64];
        valid += dfs_p2(&TOWELS, &mut cache, input, 0);

        while *input != b'\n' {
            input = input.add(1);
        }
        input = input.add(1);
    }

    valid
}

pub fn part1(input: &str) -> u32 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { inner_p2(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input19.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 298);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 572248688842069);
    }
}
