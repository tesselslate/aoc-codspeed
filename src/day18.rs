#![allow(static_mut_refs)]

use arrayvec::ArrayVec;

const QUEUE_SIZE: usize = 5120;
const QUEUE_SIZE_2: usize = 1048576;
const GRID_SIZE: usize = 72 * 73;

unsafe fn inner_p1(input: &[u8]) -> u64 {
    static mut QUEUE: ArrayVec<(*mut u8, u64), QUEUE_SIZE> = ArrayVec::new_const();
    static mut GRID: [u8; GRID_SIZE] = [0; GRID_SIZE];

    // clear
    GRID.iter_mut().for_each(|x| *x = 0);

    // bottom and top rows
    std::ptr::write_bytes(GRID.as_mut_ptr(), 1, 72);
    std::ptr::write_bytes(GRID.as_mut_ptr().add(72 * 72), 1, 72);

    for i in 1..72 {
        GRID[i * 72] = 1;
    }

    let mut input = input.as_ptr();

    for _ in 0..1024 {
        let (x, y): (usize, usize);

        if *input.add(1) == b',' {
            x = (*input - b'0') as usize;
            input = input.add(2);
        } else {
            x = (*input - b'0') as usize * 10 + (*input.add(1) - b'0') as usize;
            input = input.add(3);
        }

        if *input.add(1) == b'\n' {
            y = (*input - b'0') as usize;
            input = input.add(2);
        } else {
            y = (*input - b'0') as usize * 10 + (*input.add(1) - b'0') as usize;
            input = input.add(3);
        }

        *GRID.get_unchecked_mut(y * 72 + x + 73) = 1;
    }

    let gridptr = GRID.as_mut_ptr();

    QUEUE.clear();
    QUEUE.push_unchecked((gridptr.add(73), 0));

    let mut i = 0;
    loop {
        let (loc, dist) = *QUEUE.get_unchecked(i);

        for offset in [1, -1, 72, -72] {
            let nloc = loc.offset(offset);
            if *nloc == 0 {
                if nloc == gridptr.add(72 * 72 - 1) {
                    return dist + 1;
                }

                QUEUE.push_unchecked((nloc, dist + 1));
                *nloc = 1;
            }
        }

        i += 1;
    }
}

unsafe fn inner_p2(input: &[u8]) -> &str {
    static mut QUEUE: ArrayVec<(*mut (u16, u16), u32), QUEUE_SIZE_2> = ArrayVec::new_const();
    static mut GRID: [(u16, u16); GRID_SIZE] = [(0, 0); GRID_SIZE];
    static mut PT: [*const u8; 3450] = [std::ptr::null(); 3450];

    // clear
    GRID.iter_mut().for_each(|x| *x = (0, 0));

    std::ptr::write_bytes(GRID.as_mut_ptr(), 0xFF, 72);
    std::ptr::write_bytes(GRID.as_mut_ptr().add(72 * 72), 0xFF, 72);

    for i in 1..72 {
        GRID[i * 72] = (0xFFFF, 0xFFFF);
    }

    let mut input = input.as_ptr();

    for i in 0..3450 {
        let (x, y): (usize, usize);
        let line = input;

        if *input.add(1) == b',' {
            x = (*input - b'0') as usize;
            input = input.add(2);
        } else {
            x = (*input - b'0') as usize * 10 + (*input.add(1) - b'0') as usize;
            input = input.add(3);
        }

        if *input.add(1) == b'\n' {
            y = (*input - b'0') as usize;
            if i != 3449 {
                input = input.add(2);
            }
        } else {
            y = (*input - b'0') as usize * 10 + (*input.add(1) - b'0') as usize;
            if i != 3449 {
                input = input.add(3);
            }
        }

        *GRID.get_unchecked_mut(y * 72 + x + 73) = (i + 1, 0);
        PT[i as usize] = line;
    }

    let gridptr = GRID.as_mut_ptr();

    QUEUE.clear();
    QUEUE.push((gridptr.add(73), 3449));

    let mut i = 0;
    while i < QUEUE.len() {
        let (loc, mut time) = *QUEUE.get_unchecked(i);

        #[allow(non_snake_case)]
        let (O, M) = *loc;
        debug_assert!(O != 0xFFFF && M != 0xFFFF);

        if O > 0 {
            time = u32::min(time, O as u32);
        }
        if M >= time as u16 {
            i += 1;
            continue;
        }
        (*loc).1 = time as u16;

        for offset in [1, -1, 72, -72] {
            let nloc = loc.offset(offset);
            if *nloc == (0xFFFF, 0xFFFF) {
                continue;
            }

            QUEUE.push((nloc, time));
        }

        i += 1;
    }

    let time = GRID[72 * 72 - 1].1 - 1;
    let line = *PT.get_unchecked(time as usize);

    let len = if *line.add(3) == b'\n' {
        3
    } else if *line.add(4) == b'\n' {
        4
    } else {
        5
    };

    std::str::from_raw_parts(line, len)
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> &str {
    unsafe { inner_p2(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input18.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 280);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), "28,56");
    }
}
