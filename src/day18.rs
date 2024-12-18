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
    QUEUE.push((gridptr.add(73), 0));

    let mut i = 0;
    while i < QUEUE.len() {
        let (loc, dist) = *QUEUE.get_unchecked(i);

        for offset in [1, -1, 72, -72] {
            let nloc = loc.offset(offset);
            if *nloc == 0 {
                if nloc == gridptr.add(72 * 72 - 1) {
                    return dist + 1;
                }

                QUEUE.push((nloc, dist + 1));
                *nloc = 1;
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
