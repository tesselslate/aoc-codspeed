#![allow(static_mut_refs)]

const NUM_PATTERNS: usize = 400;

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
    data: [isize; 16384],
}

impl Trie {
    pub const fn new() -> Self {
        Self {
            free: 6,
            data: [-1; 16384],
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.free = 6;
        self.data[0..6].iter_mut().for_each(|x| *x = -1);
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
                *self.data.get_unchecked_mut(idx) = self.free as isize;
                node_index = self.free;

                self.data
                    .get_unchecked_mut(self.free..self.free + 6)
                    .iter_mut()
                    .for_each(|x| *x = -1);

                self.free += 6;
            }
        }

        *self.data.get_unchecked_mut(node_index + 5) = 0;
    }
}

unsafe fn inner_p1(input: &[u8]) -> u32 {
    static mut TOWELS: Trie = Trie::new();

    TOWELS.clear();

    let mut input = input.as_ptr();

    'outer: loop {
        let start = input;
        while *input != b',' {
            input = input.add(1);
            if *input == b'\n' {
                TOWELS.insert(UnsafeSlice {
                    data: start,
                    len: input.sub_ptr(start),
                });
                break 'outer;
            }
        }

        TOWELS.insert(UnsafeSlice {
            data: start,
            len: input.sub_ptr(start),
        });
        input = input.add(2);
    }

    debug_assert!(TOWELS.data[5] == -1);
    debug_assert!(phf(b'b') == 0);
    debug_assert!(phf(b'r') == 1);
    debug_assert!(phf(b'g') == 2);
    debug_assert!(phf(b'w') == 3);
    debug_assert!(phf(b'u') == 4);

    unsafe fn dfs(trie: &Trie, cache: *mut u64, start: *const u8, mut offset: usize) -> usize {
        let cache_idx = offset;
        if (*cache & (1 << cache_idx)) != 0 {
            return 0;
        }

        let mut trie_node = 0;
        loop {
            let char = *start.add(offset);
            if char == b'\n' {
                if *trie.data.get_unchecked(trie_node + 5) == 0 {
                    return offset;
                } else {
                    return 0;
                }
            }
            debug_assert!(b"bwrug".contains(&char));

            let idx = trie_node + phf(char);
            let next = *trie.data.get_unchecked(idx);

            if next < 0 {
                *cache |= 1 << cache_idx;
                return 0;
            }

            if *trie.data.get_unchecked(next as usize + 5) == 0 {
                let ret = dfs(trie, cache, start, offset + 1);
                if ret > 0 {
                    return ret;
                }
            }

            trie_node = next as usize;
            offset += 1;
        }
    }

    let mut valid = 0;

    input = input.add(2);
    for _ in 0..NUM_PATTERNS {
        debug_assert!(*input != b'\n');

        let mut cache = 0u64;
        let next = dfs(&TOWELS, std::ptr::from_mut(&mut cache), input, 0);

        if next > 0 {
            debug_assert!(*input.add(next) == b'\n');
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

pub fn part1(input: &str) -> u32 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> u64 {
    0
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
