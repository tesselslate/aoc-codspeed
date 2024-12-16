use std::{
    mem::MaybeUninit,
    ops::AddAssign,
    simd::{cmp::SimdPartialEq, num::SimdUint, u32x8, u8x8, Mask},
};

const DIR_LINES: usize = 20;
const DIR_LENGTH: usize = 1000;

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
unsafe fn push_v(scratch: *mut MaybeUninit<u8>, pos: *mut u8, offset: isize) -> bool {
    #[inline]
    unsafe fn is_visited(scratch: *mut MaybeUninit<u8>, pos: *mut u8, val: u8) -> bool {
        (*scratch.add(scratch.sub_ptr(pos as *mut MaybeUninit<u8>))).assume_init() == val
    }

    #[inline]
    unsafe fn mark_visited(scratch: *mut MaybeUninit<u8>, pos: *mut u8, val: u8) {
        (*scratch.add(scratch.sub_ptr(pos as *mut MaybeUninit<u8>))).write(val);
    }

    #[inline]
    unsafe fn clear_visited(scratch: *mut MaybeUninit<u8>) {
        std::ptr::write_bytes(scratch, 0, 5000);
    }

    unsafe fn check(scratch: *mut MaybeUninit<u8>, pos: *mut u8, offset: isize) -> bool {
        if is_visited(scratch, pos, 1) {
            return true;
        }
        mark_visited(scratch, pos, 1);

        match *pos {
            0 => true,
            b'#' => false,
            b']' => {
                check(scratch, pos.sub(1), offset) && check(scratch, pos.offset(offset), offset)
            }
            b'[' => {
                check(scratch, pos.add(1), offset) && check(scratch, pos.offset(offset), offset)
            }
            _ => std::hint::unreachable_unchecked(),
        }
    }

    unsafe fn walk(scratch: *mut MaybeUninit<u8>, pos: *mut u8, offset: isize) {
        if is_visited(scratch, pos, 2) {
            return;
        }
        mark_visited(scratch, pos, 2);

        match *pos {
            b']' => {
                walk(scratch, pos.offset(offset), offset);
                walk(scratch, pos.sub(1), offset);
                *pos.offset(offset) = b']';
                *pos = 0;
            }
            b'[' => {
                walk(scratch, pos.offset(offset), offset);
                walk(scratch, pos.add(1), offset);
                *pos.offset(offset) = b'[';
                *pos = 0;
            }
            _ => (),
        }
    }

    clear_visited(scratch);
    if !check(scratch, pos, offset) {
        false
    } else {
        walk(scratch, pos, offset);
        true
    }
}

unsafe fn debug_grid(grid: *const u8, robot: *const u8) {
    for r in 0..50 {
        for c in 0..100 {
            if grid.add(r * 100 + c) == robot {
                print!("@");
            } else if *grid.add(r * 100 + c) != 0 {
                print!("{}", *grid.add(r * 100 + c) as char);
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!("\n\n\n");
}

unsafe fn inner_p2(input: &str) -> u32 {
    #[repr(C)]
    struct StorageUninit {
        grid: [MaybeUninit<u8>; 5000],
        scratch: [MaybeUninit<u8>; 5000],
    }

    #[repr(C)]
    struct Storage {
        grid: [u8; 5000],
        scratch: [MaybeUninit<u8>; 5000],
    }

    const OFFSETS: [isize; 256] = {
        let mut offsets = [0; 256];

        offsets[b'<' as usize] = -1;
        offsets[b'>' as usize] = 1;
        offsets[b'^' as usize] = -100;
        offsets[b'v' as usize] = 100;

        offsets
    };

    let mut storage = StorageUninit {
        grid: [MaybeUninit::uninit(); 5000],
        scratch: [MaybeUninit::uninit(); 5000],
    };
    for r in 0..50 {
        for c in 0..50 {
            match *input.as_bytes().get_unchecked(r * 51 + c) {
                b'#' => {
                    storage.grid[r * 100 + c * 2].write(b'#');
                    storage.grid[r * 100 + c * 2 + 1].write(b'#');
                }
                b'O' => {
                    storage.grid[r * 100 + c * 2].write(b'[');
                    storage.grid[r * 100 + c * 2 + 1].write(b']');
                }
                _ => {
                    storage.grid[r * 100 + c * 2].write(0);
                    storage.grid[r * 100 + c * 2 + 1].write(0);
                }
            }
        }
    }

    let mut storage: Storage = std::mem::transmute(storage);
    let mut dir = input.as_bytes().as_ptr().add(2551);
    let mut robot = storage.grid.as_mut_ptr().add(24 * 100 + 48);

    for _ in 0..DIR_LINES {
        for _ in 0..DIR_LENGTH {
            let offset = OFFSETS[*dir as usize];
            let pos = robot.offset(offset);

            match *pos {
                0 => robot = pos,
                b'#' => (),
                b'[' | b']' => {
                    if *dir == b'<' || *dir == b'>' {
                        if push_h(pos, offset) {
                            robot = pos;
                        }
                    } else {
                        if push_v(storage.scratch.as_mut_ptr(), pos, offset) {
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

    let mut values = u32x8::from_array([0, 1, 2, 3, 4, 5, 6, 7]);
    let zero = u32x8::splat(0);
    let eight = u32x8::splat(8);
    let box_mask = u8x8::splat(b'[');
    for i in 0..5000 / 8 {
        let boxes: Mask<i32, 8> = u8x8::from_array(
            *storage
                .grid
                .get_unchecked(i * 8..i * 8 + 8)
                .as_array()
                .unwrap_unchecked(),
        )
        .simd_eq(box_mask)
        .into();

        sum += boxes.select(values, zero).reduce_sum();
        values.add_assign(eight);
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
