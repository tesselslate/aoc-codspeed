#![allow(static_mut_refs)]

const DIR_LINES: usize = 20;
const DIR_LENGTH: usize = 1000;

struct WalkData {
    boxes: [*mut u8; 64],
    num_boxes: usize,
}

static mut WALK_DATA: WalkData = WalkData {
    boxes: [std::ptr::null_mut(); 64],
    num_boxes: 0,
};

impl WalkData {
    #[inline]
    pub fn clear(&mut self) {
        self.num_boxes = 0;
    }

    #[inline]
    pub unsafe fn push(&mut self, pos: *mut u8) {
        std::hint::assert_unchecked(self.num_boxes < 64);

        self.boxes[self.num_boxes] = pos;
        self.num_boxes += 1;
    }

    #[inline]
    pub unsafe fn walked(&self, pos: *mut u8) -> bool {
        self.boxes[..self.num_boxes].contains(&pos)
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
    unsafe fn check(pos: *mut u8, offset: isize) -> bool {
        match *pos {
            0 => true,
            b'#' => false,
            b']' => check(pos.sub(1).offset(offset), offset) && check(pos.offset(offset), offset),
            b'[' => check(pos.add(1).offset(offset), offset) && check(pos.offset(offset), offset),
            _ => std::hint::unreachable_unchecked(),
        }
    }

    unsafe fn walk(pos: *mut u8, offset: isize) {
        if WALK_DATA.walked(pos) {
            return;
        }
        WALK_DATA.push(pos);

        match *pos {
            b']' => {
                walk(pos.offset(offset), offset);
                walk(pos.sub(1), offset);
                *pos.offset(offset) = b']';
                *pos = 0;
            }
            b'[' => {
                walk(pos.offset(offset), offset);
                walk(pos.add(1), offset);
                *pos.offset(offset) = b'[';
                *pos = 0;
            }
            _ => (),
        }
    }

    // fast-path
    if *pos.offset(offset) == b'#' {
        return false;
    }

    WALK_DATA.clear();

    if !check(pos, offset) {
        false
    } else {
        walk(pos, offset);
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
    const OFFSETS: [isize; 256] = {
        let mut offsets = [0; 256];

        offsets[b'<' as usize] = -1;
        offsets[b'>' as usize] = 1;
        offsets[b'^' as usize] = -100;
        offsets[b'v' as usize] = 100;

        offsets
    };

    let mut grid = [0; 5000];
    for r in 0..50 {
        for c in 0..50 {
            match *input.as_bytes().get_unchecked(r * 51 + c) {
                b'#' => {
                    grid[r * 100 + c * 2] = b'#';
                    grid[r * 100 + c * 2 + 1] = b'#';
                }
                b'O' => {
                    grid[r * 100 + c * 2] = b'[';
                    grid[r * 100 + c * 2 + 1] = b']';
                }
                _ => {
                    grid[r * 100 + c * 2] = 0;
                    grid[r * 100 + c * 2 + 1] = 0;
                }
            }
        }
    }

    let mut dir = input.as_bytes().as_ptr().add(2551);
    let mut robot = grid.as_mut_ptr().add(24 * 100 + 48);

    for _ in 0..DIR_LINES {
        for _ in 0..DIR_LENGTH {
            let offset = OFFSETS[*dir as usize];
            let pos = robot.offset(offset);

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
    for i in 0..5000 {
        if *grid.get_unchecked(i) == b'[' {
            sum += i;
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
