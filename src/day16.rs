#![allow(static_mut_refs)]

use std::collections::BinaryHeap;

const START_OFFSET: isize = 4830;
const END_OFFSET: isize = 69;
const GRID_SIZE: usize = 4900;

const D_EAST: isize = 1;
const D_WEST: isize = -1;
const D_SOUTH: isize = 70;
const D_NORTH: isize = -70;

const D_EAST_MASK: u8 = 0x1;
const D_WEST_MASK: u8 = 0x2;
const D_SOUTH_MASK: u8 = 0x4;
const D_NORTH_MASK: u8 = 0x8;

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
static mut GRID: [u8; GRID_SIZE] = [0; GRID_SIZE];
static mut DIST: [u32; GRID_SIZE] = [u32::MAX; GRID_SIZE];

unsafe fn parse_grid(input: &[u8]) {
    let mut src = input.as_ptr().add(143);
    let mut dst = GRID.as_mut_ptr();

    for r in 0..70 {
        for _ in 0..70 {
            let mut mask = 0;

            if *src.offset(1) != b'#' {
                mask |= 0x1;
            }
            if *src.offset(-1) != b'#' {
                mask |= 0x2;
            }
            if *src.offset(142) != b'#' {
                mask |= 0x4;
            }
            if *src.offset(-142) != b'#' {
                mask |= 0x8;
            }

            *dst = mask;
            dst = dst.add(1);
            src = src.add(2);
        }
        if r != 69 {
            // I HATE POINTER UB
            src = src.add(144);
        }
    }
}

unsafe fn inner_p1(input: &[u8]) -> u32 {
    macro_rules! dijkstra_inner {
        ($entry: ident, $dir: ident, $mask: ident, $GRID: ident, $DIST: ident, $PQ: ident) => {
            if *$entry.loc & $mask != 0 {
                let cost = if $entry.dir == $dir as i32 { 2 } else { 1002 };
                let dist_offset = $entry.loc.offset($dir).sub_ptr($GRID.as_ptr());
                if *$DIST.get_unchecked(dist_offset) > $entry.dist + cost {
                    $PQ.push(PqEntry {
                        loc: $entry.loc.offset($dir),
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
    parse_grid(input);

    PQ.push(PqEntry {
        loc: GRID.as_ptr().offset(START_OFFSET),
        dir: D_EAST as i32,
        dist: 0,
    });

    while let Some(entry) = PQ.pop() {
        if *DIST.get_unchecked(entry.loc.sub_ptr(GRID.as_ptr())) < entry.dist {
            continue;
        }
        if entry.loc == GRID.as_ptr().add(END_OFFSET as usize) {
            return entry.dist;
        }

        dijkstra_inner!(entry, D_EAST, D_EAST_MASK, GRID, DIST, PQ);
        dijkstra_inner!(entry, D_WEST, D_WEST_MASK, GRID, DIST, PQ);
        dijkstra_inner!(entry, D_SOUTH, D_SOUTH_MASK, GRID, DIST, PQ);
        dijkstra_inner!(entry, D_NORTH, D_NORTH_MASK, GRID, DIST, PQ);
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
