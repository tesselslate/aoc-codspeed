#![allow(static_mut_refs)]

use arrayvec::ArrayVec;

const QUEUE_SIZE: usize = 5120;
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
    static mut QUEUE: ArrayVec<(u8, u8), QUEUE_SIZE> = ArrayVec::new_const();
    static mut GRID: [u16; 71 * 71] = [0xFFFF; 71 * 71];

    // clear
    GRID.iter_mut().for_each(|x| *x = 0xFFFF);

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

        *GRID.get_unchecked_mut(y * 71 + x) = 1;
    }

    let gridptr = GRID.as_mut_ptr();

    let mut id = 2;
    loop {
        let (x, y): (usize, usize);
        let line = input;

        if *input.add(1) == b',' {
            x = (*input - b'0') as usize;
            input = input.add(2);
        } else {
            x = (*input - b'0') as usize * 10 + (*input.add(1) - b'0') as usize;
            input = input.add(3);
        }

        // surely it will never be on the last line, since moving the pointer
        // past is technically ub
        if *input.add(1) == b'\n' {
            y = (*input - b'0') as usize;
            input = input.add(2);
        } else {
            y = (*input - b'0') as usize * 10 + (*input.add(1) - b'0') as usize;
            input = input.add(3);
        }

        *gridptr.add(y * 71 + x) = 1;

        QUEUE.clear();
        QUEUE.push_unchecked((x as u8, y as u8));

        let (mut l, mut r, mut d, mut u) = (false, false, false, false);

        let mut i = 0;
        while i < QUEUE.len() {
            let (x, y) = *QUEUE.get_unchecked(i);

            l |= x == 0;
            r |= x == 70;
            d |= y == 70;
            u |= y == 0;

            for (dx, dy) in [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
                (-1, -1),
                (-1, 1),
                (1, -1),
                (1, 1),
            ] {
                let (nx, ny) = (x as i8 + dx as i8, y as i8 + dy as i8);
                if nx > 0 && nx <= 70 && ny > 0 && ny <= 70 {
                    let nptr = gridptr.add(ny as usize * 71 + nx as usize);
                    if *nptr < id {
                        *nptr = id;
                        QUEUE.push_unchecked((nx as u8, ny as u8));
                    }
                }
            }

            i += 1;
        }

        if (l || d) && (r || u) {
            let len = if *line.add(3) == b'\n' {
                3
            } else if *line.add(4) == b'\n' {
                4
            } else {
                5
            };
            return std::str::from_raw_parts(line, len);
        }

        id += 1;
    }
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
