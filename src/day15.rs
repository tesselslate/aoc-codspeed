struct Dirs<const DIR_LINES: usize, const DIR_LENGTH: usize> {
    data: *const u8,
}

impl<const DIR_LINES: usize, const DIR_LENGTH: usize> Dirs<DIR_LINES, DIR_LENGTH> {
    #[inline]
    fn new(value: *const u8) -> Self {
        Self { data: value }
    }

    #[inline]
    fn next(&mut self) -> u8 {
        unsafe {
            let val = *self.data;
            self.data = self.data.add(1);
            val
        }
    }

    #[inline]
    fn next_line(&mut self) {
        unsafe { self.data = self.data.add(1) };
    }
}

unsafe fn inner_p1<
    const SZ: usize,
    const W: usize,
    const DIR_LINES: usize,
    const DIR_LENGTH: usize,
>(
    input: &str,
) -> u32 {
    #[allow(non_snake_case)]
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
    let mut dirs = Dirs::<DIR_LINES, DIR_LENGTH>::new(input.as_bytes().as_ptr().add(SZ + 1));

    let mut robot = grid.as_mut_ptr().add(24 * 51 + 24);
    *robot = b'.';

    for _ in 0..DIR_LINES {
        for _ in 0..DIR_LENGTH {
            let offset = OFFSETS[dirs.next() as usize];
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
        }

        dirs.next_line();
    }

    let mut sum = 0;
    for i in 0..SZ {
        if *grid.get_unchecked(i) == b'O' {
            sum += (i / W) * 100 + (i % W)
        }
    }
    sum as u32
}

pub fn part1(input: &str) -> u32 {
    unsafe { inner_p1::<2550, 51, 20, 1000>(input) }
}

pub fn part2(input: &str) -> u32 {
    0
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
