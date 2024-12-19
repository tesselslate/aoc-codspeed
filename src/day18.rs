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
    static mut UNION: [u16; 71 * 71 + 2] = [0xFFFF; 71 * 71 + 2];

    #[inline]
    unsafe fn merge(union: &mut [u16; 71 * 71 + 2], a: usize, b: usize) {
        let a = find(union, a);
        let b = find(union, b);
        if a != b {
            *union.get_unchecked_mut(b as usize) = a;
        }
    }

    #[inline]
    unsafe fn find(union: &mut [u16; 71 * 71 + 2], a: usize) -> u16 {
        if *union.get_unchecked(a) == a as u16 {
            return a as u16;
        }

        *union.get_unchecked_mut(a) = find(union, *union.get_unchecked(a) as usize);
        *union.get_unchecked(a)
    }

    macro_rules! try_offset {
        ($id: ident, $offset: literal, $UNION: ident) => {{
            let nid = $id as isize + $offset;
            if (nid as usize) < 71 * 71 && *$UNION.get_unchecked(nid as usize) != 0xFFFF {
                merge(&mut $UNION, $id, nid as usize);
            }
        }};
    }

    // clear
    UNION.iter_mut().for_each(|x| *x = 0xFFFF);
    UNION[71 * 71] = 71 * 71;
    UNION[71 * 71 + 1] = 71 * 71 + 1;

    let mut input = input.as_ptr();
    let mut i = 0;

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

        if *input.add(1) == b'\n' {
            y = (*input - b'0') as usize;
            input = input.add(2);
        } else {
            y = (*input - b'0') as usize * 10 + (*input.add(1) - b'0') as usize;
            input = input.add(3);
        }

        let id = y * 71 + x;
        *UNION.get_unchecked_mut(id) = id as u16;

        if x > 0 {
            try_offset!(id, -72, UNION);
            try_offset!(id, -1, UNION);
            try_offset!(id, 70, UNION);
        }
        if x < 70 {
            try_offset!(id, -70, UNION);
            try_offset!(id, 1, UNION);
            try_offset!(id, 72, UNION);
        }
        try_offset!(id, -71, UNION);
        try_offset!(id, 71, UNION);

        if x == 0 || y == 70 {
            merge(&mut UNION, id, 71 * 71);
        }
        if x == 70 || y == 0 {
            merge(&mut UNION, id, 71 * 71 + 1);
        }

        if i >= 1024 {
            if find(&mut UNION, 71 * 71) == find(&mut UNION, 71 * 71 + 1) {
                return std::str::from_raw_parts(
                    line,
                    if *line.add(3) == b'\n' {
                        3
                    } else if *line.add(4) == b'\n' {
                        4
                    } else {
                        5
                    },
                );
            }
        }

        i += 1;
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
