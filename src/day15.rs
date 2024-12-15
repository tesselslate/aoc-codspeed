#[repr(transparent)]
struct Grid<const SZ: usize, const W: usize>([u8; SZ]);

impl<const SZ: usize, const W: usize> Grid<SZ, W> {
    pub unsafe fn from_p1<const DST_SZ: usize, const SRC_DIM: usize>(
        input: &str,
    ) -> Grid<DST_SZ, SRC_DIM> {
        let mut grid = [0; DST_SZ];
        grid.copy_from_slice(input.as_bytes().get_unchecked(..DST_SZ));
        Grid(grid)
    }

    // pub fn from_p2<const DST_SZ: usize, const SRC_DIM: usize, const DST_DIM: usize>(
    //     input: &str,
    // ) -> Grid<DST_SZ, DST_DIM> {
    //     let mut grid = [MaybeUninit::<u8>::uninit(); DST_SZ];
    // }
}

impl<const SZ: usize, const W: usize> ToString for Grid<SZ, W> {
    fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.0).to_string()
    }
}

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
    let OFFSETS: [isize; 256] = {
        let mut offsets = [0; 256];

        offsets[b'<' as usize] = -1;
        offsets[b'>' as usize] = 1;
        offsets[b'^' as usize] = -(W as isize);
        offsets[b'v' as usize] = W as isize;

        offsets
    };

    let mut grid = Grid::<SZ, W>::from_p1::<SZ, W>(input);
    let mut dirs = Dirs::<DIR_LINES, DIR_LENGTH>::new(input.as_bytes().as_ptr().add(SZ + 1));

    let mut robot = if W == 51 {
        // real input is always centered
        grid.0.as_mut_ptr().add(24 * 51 + 24)
    } else {
        panic!();
    };
    *robot = b'.';

    for _ in 0..DIR_LINES {
        for _ in 0..DIR_LENGTH {
            let offset = OFFSETS[dirs.next() as usize];
            let pos = unsafe { robot.offset(offset) };

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
                            unsafe { std::hint::assert_unchecked(*box_pos == b'O') };
                        }

                        box_pos = box_pos.offset(offset);
                    }
                }
                _ => unsafe { std::hint::unreachable_unchecked() },
            }
        }

        dirs.next_line();
    }

    let mut sum = 0;
    for i in 0..SZ {
        if *grid.0.get_unchecked(i) == b'O' {
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
    const TEST_1: &str = include_str!("../testdata/input15_a.txt");
    const TEST_2: &str = include_str!("../testdata/input15_b.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 1568399);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 1575877);
    }

    #[test]
    fn test_a1() {
        assert_eq!(unsafe { inner_p1::<110, 11, 10, 70>(TEST_1) }, 10092);
    }

    #[test]
    fn test_a2() {
        assert_eq!(unsafe { inner_p1::<72, 9, 1, 15>(TEST_2) }, 2028);
    }

    // #[test]
    // fn test_b1() {
    //     assert_eq!(part2(TEST_1), 9021);
    // }
}
