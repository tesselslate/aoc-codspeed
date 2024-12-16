#![allow(static_mut_refs)]

use std::simd::{cmp::SimdPartialEq, num::SimdUint, simd_swizzle, u32x8, u8x16, u8x32, u8x8, Mask};

const DIR_LINES: usize = 20;
const DIR_LENGTH: usize = 1000;
const WALK_DATA_SZ: usize = 63;

const LUT1: [u32; 2560] = {
    let mut values = [0; 2560];

    let mut i = 0u32;
    while i < 2560 {
        values[i as usize] = (i / 51) * 100 + (i % 51);
        i += 1;
    }

    values
};

// const LUT_SUM_P2: [u32; 128 * 50] = {
//     let mut values = [0; 128 * 50];
//
//     let mut r = 0;
//     while r < 50 {
//         let mut c = 0;
//         while c < 100 {
//             values[(r * 128 + c) as usize] = r * 100 + c;
//             c += 1;
//         }
//         r += 1;
//     }
//
//     values
// };

#[repr(C)]
struct WalkData {
    num_boxes: usize,
    boxes: [*mut u8; WALK_DATA_SZ],
}

#[repr(align(16))]
struct Grid([u8; 128 * 50]);

static mut WALK_DATA: WalkData = WalkData {
    num_boxes: 0,
    boxes: [std::ptr::null_mut(); WALK_DATA_SZ],
};

static mut GRID_2: Grid = Grid([0; 128 * 50]);

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

    let mut grid = [0; 2560];
    grid[..2550].copy_from_slice(input.as_bytes().get_unchecked(..2550));
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

    let simd_box = u8x8::splat(b'O');
    let simd_zero = u32x8::splat(0);

    for i in 0..2560 / 32 {
        let d1 = u8x8::from_array(*grid[i * 32..i * 32 + 8].as_array().unwrap_unchecked());
        let v1 = u32x8::from_array(*LUT1[i * 32..i * 32 + 8].as_array().unwrap_unchecked());
        let d2 = u8x8::from_array(*grid[i * 32 + 8..i * 32 + 16].as_array().unwrap_unchecked());
        let v2 = u32x8::from_array(*LUT1[i * 32 + 8..i * 32 + 16].as_array().unwrap_unchecked());
        let d3 = u8x8::from_array(*grid[i * 32 + 16..i * 32 + 24].as_array().unwrap_unchecked());
        let v3 = u32x8::from_array(*LUT1[i * 32 + 16..i * 32 + 24].as_array().unwrap_unchecked());
        let d4 = u8x8::from_array(*grid[i * 32 + 24..i * 32 + 32].as_array().unwrap_unchecked());
        let v4 = u32x8::from_array(*LUT1[i * 32 + 24..i * 32 + 32].as_array().unwrap_unchecked());

        let b1: Mask<i32, 8> = d1.simd_eq(simd_box).into();
        let b2: Mask<i32, 8> = d2.simd_eq(simd_box).into();
        let b3: Mask<i32, 8> = d3.simd_eq(simd_box).into();
        let b4: Mask<i32, 8> = d4.simd_eq(simd_box).into();

        sum += b1.select(v1, simd_zero).reduce_sum();
        sum += b2.select(v2, simd_zero).reduce_sum();
        sum += b3.select(v3, simd_zero).reduce_sum();
        sum += b4.select(v4, simd_zero).reduce_sum();
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
    const OFFSETS: [isize; 256] = {
        let mut offsets = [0; 256];

        offsets[b'<' as usize] = -1;
        offsets[b'>' as usize] = 1;
        offsets[b'^' as usize] = -128;
        offsets[b'v' as usize] = 128;

        offsets
    };

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

            left.copy_to_slice(
                GRID_2
                    .0
                    .get_unchecked_mut(r * 128 + c * 2..r * 128 + c * 2 + 16),
            );
            right.copy_to_slice(
                GRID_2
                    .0
                    .get_unchecked_mut(r * 128 + c * 2 + 16..r * 128 + c * 2 + 32),
            );
        }
    }

    let mut dir = input.as_bytes().as_ptr().add(2551);
    let mut robot = GRID_2.0.as_mut_ptr().add(24 * 128 + 48) as *mut u8;

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
            if *GRID_2.0.get_unchecked(r * 128 + c) == b'[' {
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
