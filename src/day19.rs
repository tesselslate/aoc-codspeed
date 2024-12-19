#![allow(static_mut_refs)]

use std::collections::HashSet;

use arrayvec::ArrayVec;
use rustc_hash::FxSeededState;

const NUM_PATTERNS: usize = 400;

#[derive(Clone, Copy, PartialEq, Eq)]
struct UnsafeSlice {
    data: *const u8,
    len: usize,
}

impl UnsafeSlice {
    #[inline]
    pub unsafe fn matches(&self, other: *const u8) -> *const u8 {
        for i in 0..self.len {
            if *self.data.add(i) != *other.add(i) {
                return std::ptr::null();
            }
        }

        other.add(self.len)
    }
}

unsafe fn inner_p1(input: &[u8]) -> u32 {
    static mut TOWELS: ArrayVec<UnsafeSlice, 16384> = ArrayVec::new_const();
    static mut CACHE: HashSet<*const u8, FxSeededState> =
        HashSet::with_hasher(FxSeededState::with_seed(9587345));

    TOWELS.clear();
    CACHE.clear();

    let mut input = input.as_ptr();

    'outer: loop {
        let start = input;
        while *input != b',' {
            input = input.add(1);
            if *input == b'\n' {
                TOWELS.push_unchecked(UnsafeSlice {
                    data: start,
                    len: input.sub_ptr(start),
                });
                break 'outer;
            }
        }

        TOWELS.push_unchecked(UnsafeSlice {
            data: start,
            len: input.sub_ptr(start),
        });
        input = input.add(2);
    }

    unsafe fn dfs(start: *const u8) -> *const u8 {
        if *start == b'\n' {
            return start;
        }

        if CACHE.contains(&start) {
            return std::ptr::null();
        }

        for towel in &TOWELS {
            let next = towel.matches(start);
            if !next.is_null() {
                let ptr = dfs(next);
                if !ptr.is_null() {
                    return ptr;
                }
            }
        }

        CACHE.insert(start);
        std::ptr::null()
    }

    let mut valid = 0;

    input = input.add(2);
    for _ in 0..NUM_PATTERNS {
        debug_assert!(*input != b'\n');
        let start = input;
        let next = dfs(start);
        if !next.is_null() {
            valid += 1;
            input = next.add(1);
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
