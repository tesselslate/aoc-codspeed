#![allow(static_mut_refs)]

const START_OFFSET: isize = 142 * 139 + 1;
const END_OFFSET: isize = 142 + 139;
const GRID_SIZE: usize = 142 * 141;

const D_EAST: isize = 1;
const D_WEST: isize = -1;
const D_SOUTH: isize = 142;
const D_NORTH: isize = -142;

const QSIZE: usize = 4095;

#[derive(Clone, Copy, PartialEq, Eq)]
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

struct Queue {
    step_w: usize,
    step_r: usize,
    step: [PqEntry; QSIZE],

    turn_w: usize,
    turn_r: usize,
    turn: [PqEntry; QSIZE],
}

impl Queue {
    #[inline]
    pub fn clear(&mut self) {
        self.step_w = 0;
        self.step_r = 0;

        self.turn_w = 0;
        self.turn_r = 0;
    }

    #[inline]
    pub fn clear_step(&mut self) {
        self.step_w = 0;
        self.step_r = 0;
    }

    #[inline]
    pub unsafe fn pop_step(&mut self) -> Option<PqEntry> {
        if self.step_r == self.step_w {
            None
        } else {
            let entry = *self.step.get_unchecked(self.step_r);
            self.step_r += 1;
            Some(entry)
        }
    }

    #[inline]
    pub unsafe fn pop_turn(&mut self, max: usize) -> Option<PqEntry> {
        if self.turn_r == max {
            None
        } else {
            let entry = *self.turn.get_unchecked(self.turn_r);
            self.turn_r += 1;
            Some(entry)
        }
    }

    #[inline]
    pub unsafe fn push_step(&mut self, entry: PqEntry) {
        *self.step.get_unchecked_mut(self.step_w) = entry;
        self.step_w += 1;
    }

    #[inline]
    pub unsafe fn push_turn(&mut self, entry: PqEntry) {
        *self.turn.get_unchecked_mut(self.turn_w) = entry;
        self.turn_w += 1;
    }

    pub const fn new() -> Self {
        Self {
            step_w: 0,
            step_r: 0,
            step: [PqEntry {
                loc: std::ptr::null(),
                dir: 0,
                dist: 0,
            }; QSIZE],

            turn_w: 0,
            turn_r: 0,
            turn: [PqEntry {
                loc: std::ptr::null(),
                dir: 0,
                dist: 0,
            }; QSIZE],
        }
    }
}

unsafe fn inner_p1(input: &[u8]) -> u32 {
    static mut PQ: Queue = Queue::new();
    static mut DIST: [u32; GRID_SIZE] = [u32::MAX; GRID_SIZE];

    macro_rules! dijkstra_inner {
        ($entry: ident, $dir: ident, $DIST: ident, $PQ: ident, $input: ident) => {
            if *$entry.loc.offset($dir) != b'#' {
                let cost = if $entry.dir == $dir as i32 { 2 } else { 1002 };
                let dist_offset = $entry.loc.offset($dir * 2).sub_ptr($input.as_ptr());
                if *$DIST.get_unchecked(dist_offset) > $entry.dist + cost {
                    if dist_offset == END_OFFSET as usize {
                        return $entry.dist + cost;
                    }

                    if $entry.dir == $dir as i32 {
                        $PQ.push_step(PqEntry {
                            loc: $entry.loc.offset($dir * 2),
                            dir: $dir as i32,
                            dist: $entry.dist + cost,
                        });
                    } else {
                        $PQ.push_turn(PqEntry {
                            loc: $entry.loc.offset($dir * 2),
                            dir: $dir as i32,
                            dist: $entry.dist + cost,
                        });
                    }
                    *$DIST.get_unchecked_mut(dist_offset) = $entry.dist + cost;
                }
            }
        };
    }

    PQ.clear();
    std::ptr::write_bytes(DIST.as_mut_ptr(), 0xFF, GRID_SIZE);

    PQ.push_step(PqEntry {
        loc: input.as_ptr().offset(START_OFFSET),
        dir: D_EAST as i32,
        dist: 0,
    });

    loop {
        while let Some(entry) = PQ.pop_step() {
            dijkstra_inner!(entry, D_EAST, DIST, PQ, input);
            dijkstra_inner!(entry, D_WEST, DIST, PQ, input);
            dijkstra_inner!(entry, D_SOUTH, DIST, PQ, input);
            dijkstra_inner!(entry, D_NORTH, DIST, PQ, input);
        }

        PQ.clear_step();
        let end = PQ.turn_w;
        while let Some(entry) = PQ.pop_turn(end) {
            dijkstra_inner!(entry, D_EAST, DIST, PQ, input);
            dijkstra_inner!(entry, D_WEST, DIST, PQ, input);
            dijkstra_inner!(entry, D_SOUTH, DIST, PQ, input);
            dijkstra_inner!(entry, D_NORTH, DIST, PQ, input);
        }
    }
}

pub fn part1(input: &str) -> u32 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(_input: &str) -> u32 {
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

    // #[test]
    // fn b() {
    //     assert_eq!(part2(INPUT), 511);
    // }
}
