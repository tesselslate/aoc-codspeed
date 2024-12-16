#![allow(static_mut_refs)]

use std::{
    clone::CloneToUninit,
    mem::MaybeUninit,
    simd::{cmp::SimdPartialEq, u8x16},
};

const DIR_LINES: usize = 20;
const DIR_LENGTH: usize = 1000;
const WALK_DATA_SZ: usize = 63;

#[repr(C)]
struct WalkData {
    num_boxes: usize,
    boxes: [*mut u8; WALK_DATA_SZ],
}

static mut WALK_DATA: WalkData = WalkData {
    num_boxes: 0,
    boxes: [std::ptr::null_mut(); WALK_DATA_SZ],
};

impl WalkData {
    #[inline]
    pub fn clear(&mut self) {
        self.num_boxes = 0;
    }

    #[inline]
    pub unsafe fn push(&mut self, pos: *mut u8) {
        std::hint::assert_unchecked(self.num_boxes < WALK_DATA_SZ);

        *self.boxes.get_unchecked_mut(self.num_boxes) = pos;
        self.num_boxes += 1;
    }

    #[inline]
    pub unsafe fn push_only(&mut self, pos: *mut u8) {
        if !self.walked(pos) {
            self.push(pos);
        }
    }

    #[inline]
    pub unsafe fn walked(&self, pos: *mut u8) -> bool {
        self.boxes.get_unchecked(..self.num_boxes).contains(&pos)
    }
}

unsafe fn inner_p1(input: &str) -> u32 {
    const OFFSETS: [isize; 256] = {
        let mut offsets = [0; 256];

        offsets[b'<' as usize] = -1;
        offsets[b'>' as usize] = 1;
        offsets[b'^' as usize] = -51;
        offsets[b'v' as usize] = 51;

        offsets
    };

    let mut grid = [0; 2550];
    grid.copy_from_slice(input.as_bytes().get_unchecked(..2550));
    let mut dir = input.as_bytes().as_ptr().add(2551);

    let mut robot = grid.as_mut_ptr().add(24 * 51 + 24);
    *robot = b'.';

    for _ in 0..DIR_LINES {
        for _ in 0..DIR_LENGTH {
            let offset = OFFSETS[*dir as usize];
            let pos = robot.offset(offset);

            match *pos {
                b'.' => robot = pos,
                b'#' => (),
                b'O' => {
                    let mut box_pos = pos;

                    loop {
                        if *box_pos == b'.' {
                            *pos = b'.';
                            *box_pos = b'O';
                            robot = pos;
                            break;
                        } else if *box_pos == b'#' {
                            break;
                        } else {
                            std::hint::assert_unchecked(*box_pos == b'O');
                        }

                        box_pos = box_pos.offset(offset);
                    }
                }
                _ => std::hint::unreachable_unchecked(),
            }
            dir = dir.add(1);
        }
        dir = dir.add(1);
    }

    let mut sum = 0;
    for i in 0..2550 {
        // TODO: SIMD and/or LUT
        if *grid.get_unchecked(i) == b'O' {
            sum += (i / 51) * 100 + (i % 51)
        }
    }
    sum as u32
}

#[inline]
unsafe fn push_h(pos: *mut u8, offset: isize) -> bool {
    let mut box_pos = pos.offset(offset);

    loop {
        if *box_pos == 0 {
            break;
        } else if *box_pos == b'#' {
            return false;
        } else {
            box_pos = box_pos.offset(offset);
        }
    }

    while pos != box_pos {
        *box_pos = *box_pos.offset(-offset);
        box_pos = box_pos.offset(-offset);
    }

    *pos = 0;

    true
}

#[inline]
unsafe fn push_v(pos: *mut u8, offset: isize) -> bool {
    // fast-path
    if *pos.offset(offset) == b'#' {
        return false;
    }

    WALK_DATA.clear();
    WALK_DATA.push(pos);
    let mut i = 0;
    while i < WALK_DATA.num_boxes {
        let pos = *WALK_DATA.boxes.get_unchecked(i);
        std::hint::assert_unchecked(*pos == b']' || *pos == b'[');

        if *pos == b']' {
            WALK_DATA.push_only(pos.sub(1));
        } else {
            WALK_DATA.push_only(pos.add(1));
        }

        if *pos.offset(offset) > 64 {
            WALK_DATA.push(pos.offset(offset));
        } else if *pos.offset(offset) == b'#' {
            return false;
        }

        i += 1;
    }

    for i in (0..WALK_DATA.num_boxes).rev() {
        let pos = *WALK_DATA.boxes.get_unchecked(i);
        *pos.offset(offset) = *pos;
        *pos = 0;
    }

    true
}

unsafe fn inner_p2(input: &str) -> u32 {
    #[repr(align(16))]
    struct Grid([MaybeUninit<u8>; 128 * 50]);

    const OFFSETS: [isize; 256] = {
        let mut offsets = [0; 256];

        offsets[b'<' as usize] = -1;
        offsets[b'>' as usize] = 1;
        offsets[b'^' as usize] = -128;
        offsets[b'v' as usize] = 128;

        offsets
    };

    let mut grid = Grid([MaybeUninit::<u8>::uninit(); 128 * 50]);

    let simd_box = u8x16::splat(b'O');
    let simd_wall = u8x16::splat(b'#');
    let simd_lbox = u8x16::splat(b'[');
    let simd_rbox = u8x16::splat(b']');
    let simd_zero = u8x16::splat(0);

    for r in 0..50 {
        for c in (0..64).step_by(16) {
            let src = u8x16::from_array(
                *input
                    .as_bytes()
                    .get_unchecked(r * 51 + c..r * 51 + c + 16)
                    .as_array()
                    .unwrap_unchecked(),
            );

            let box_mask = src.simd_eq(simd_box);
            let wall_mask = src.simd_eq(simd_wall);

            let left = wall_mask.select(simd_wall, box_mask.select(simd_lbox, simd_zero));
            let right = wall_mask.select(simd_wall, box_mask.select(simd_rbox, simd_zero));
            let (left, right) = left.interleave(right);

            left.clone_to_uninit(
                grid.0
                    .get_unchecked_mut(r * 128 + c * 2..r * 128 + c * 2 + 16)
                    .as_mut_ptr() as *mut u8,
            );
            right.clone_to_uninit(
                grid.0
                    .get_unchecked_mut(r * 128 + c * 2 + 16..r * 128 + c * 2 + 32)
                    .as_mut_ptr() as *mut u8,
            );
        }
    }

    let mut dir = input.as_bytes().as_ptr().add(2551);
    let mut robot = grid.0.as_mut_ptr().add(24 * 128 + 48) as *mut u8;

    for _ in 0..DIR_LINES {
        for _ in 0..DIR_LENGTH {
            let offset = OFFSETS[*dir as usize];
            let pos = robot.offset(offset) as *mut u8;

            match *pos {
                0 => robot = pos,
                b'#' => (),
                b'[' | b']' => {
                    if *dir < 64 {
                        if push_h(pos, offset) {
                            robot = pos;
                        }
                    } else {
                        if push_v(pos, offset) {
                            robot = pos;
                        }
                    }
                }
                _ => std::hint::unreachable_unchecked(),
            }
            dir = dir.add(1);
        }
        dir = dir.add(1);
    }

    let mut sum = 0;
    for r in 0..50 {
        for c in 0..100 {
            if (*grid.0.get_unchecked(r * 128 + c)).assume_init() == b'[' {
                sum += r * 100 + c;
            }
        }
    }
    sum as u32
}

pub fn part1(input: &str) -> u32 {
    unsafe { inner_p1(input) }
}

pub fn part2(input: &str) -> u32 {
    unsafe { inner_p2(input) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input15.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 1568399);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 1575877);
    }
}
