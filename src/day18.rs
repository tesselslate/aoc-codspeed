#![allow(static_mut_refs)]

use arrayvec::ArrayVec;

const QUEUE_SIZE: usize = 5120;
const GRID_DIM: usize = 71;
const GRID_SIZE: usize = GRID_DIM * GRID_DIM;

unsafe fn inner_p1(input: &[u8]) -> u64 {
    static mut QUEUE: ArrayVec<(i32, i32, u64), QUEUE_SIZE> = ArrayVec::new_const();
    static mut GRID: [u8; GRID_SIZE] = [0; GRID_SIZE];

    GRID.iter_mut().for_each(|x| *x = 0);
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

        *GRID.get_unchecked_mut(x * GRID_DIM + y) = 1;
    }

    QUEUE.clear();
    QUEUE.push((0, 0, 0));

    let mut i = 0;
    while i < QUEUE.len() {
        let (x, y, dist) = *QUEUE.get_unchecked(i);

        for offset in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let (nx, ny) = (x + offset.0, y + offset.1);
            if nx < 0 || ny < 0 || nx == GRID_DIM as i32 || ny == GRID_DIM as i32 {
                continue;
            }

            if *GRID.get_unchecked(nx as usize * GRID_DIM + ny as usize) == 0 {
                if nx == GRID_DIM as i32 - 1 && ny == GRID_DIM as i32 - 1 {
                    return dist + 1;
                }

                QUEUE.push((nx, ny, dist + 1));
                *GRID.get_unchecked_mut(nx as usize * GRID_DIM + ny as usize) = 1;
            }
        }

        i += 1;
    }

    std::hint::unreachable_unchecked()
}

unsafe fn inner_p2(input: &[u8]) -> &'static str {
    static mut OUTBUF: [u8; 16] = [0; 16];

    std::str::from_utf8_unchecked(&OUTBUF[..0])
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> &'static str {
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
