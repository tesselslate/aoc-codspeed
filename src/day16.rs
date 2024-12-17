#![allow(static_mut_refs)]

use std::collections::BinaryHeap;

const START_OFFSET: isize = 142 * 139 + 1;
const END_OFFSET: isize = 142 + 139;
const GRID_SIZE: usize = 142 * 141;

const D_EAST: isize = 1;
const D_WEST: isize = -1;
const D_SOUTH: isize = 142;
const D_NORTH: isize = -142;

#[derive(PartialEq, Eq)]
struct PqEntry {
    loc: *const u8,
    dir: i32,
    dist: u32,
}

impl Ord for PqEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then(other.dir.cmp(&self.dir))
            .then(other.loc.cmp(&self.loc))
    }
}

impl PartialOrd for PqEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

static mut PQ: BinaryHeap<PqEntry> = BinaryHeap::new();
static mut DIST: [u32; GRID_SIZE] = [u32::MAX; GRID_SIZE];

unsafe fn inner_p1(input: &[u8]) -> u32 {
    macro_rules! dijkstra_inner {
        ($entry: ident, $dir: ident, $DIST: ident, $PQ: ident, $input: ident) => {
            if *$entry.loc.offset($dir) != b'#' {
                let cost = if $entry.dir == $dir as i32 { 2 } else { 1002 };
                let dist_offset = $entry.loc.offset($dir * 2).sub_ptr($input.as_ptr());
                if *$DIST.get_unchecked(dist_offset) > $entry.dist + cost {
                    $PQ.push(PqEntry {
                        loc: $entry.loc.offset($dir * 2),
                        dir: $dir as i32,
                        dist: $entry.dist + cost,
                    });
                    *$DIST.get_unchecked_mut(dist_offset) = $entry.dist + cost;
                }
            }
        };
    }

    PQ.clear();
    std::ptr::write_bytes(DIST.as_mut_ptr(), 0xFF, GRID_SIZE);

    PQ.push(PqEntry {
        loc: input.as_ptr().offset(START_OFFSET),
        dir: D_EAST as i32,
        dist: 0,
    });

    while let Some(entry) = PQ.pop() {
        if entry.loc == input.as_ptr().add(END_OFFSET as usize) {
            return entry.dist;
        }

        dijkstra_inner!(entry, D_EAST, DIST, PQ, input);
        dijkstra_inner!(entry, D_WEST, DIST, PQ, input);
        dijkstra_inner!(entry, D_SOUTH, DIST, PQ, input);
        dijkstra_inner!(entry, D_NORTH, DIST, PQ, input);
    }

    std::hint::unreachable_unchecked()
}

pub fn part1(input: &str) -> u32 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input16.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 95476);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 511);
    }
}
