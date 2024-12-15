use std::mem::MaybeUninit;

#[repr(transparent)]
struct Grid<const SZ: usize, const W: usize>([u8; SZ]);

impl<const SZ: usize, const W: usize> Grid<SZ, W> {
    pub fn from_p1<const DST_SZ: usize, const SRC_DIM: usize>(
        input: &str,
    ) -> Grid<DST_SZ, SRC_DIM> {
        let input = input.as_bytes();

        let mut grid = [MaybeUninit::<u8>::uninit(); DST_SZ];

        for i in 0..SRC_DIM {
            let src =
                unsafe { input.get_unchecked(i * (SRC_DIM + 1)..i * (SRC_DIM + 1) + SRC_DIM) };
            let dst = unsafe { grid.get_unchecked_mut(i * SRC_DIM..i * SRC_DIM + SRC_DIM) };

            unsafe { std::hint::assert_unchecked(src.len() == dst.len()) };
            debug_assert!(src.len() == SRC_DIM);

            for j in 0..SRC_DIM {
                dst[j].write(src[j]);
            }
        }

        let dst = unsafe { std::mem::transmute_copy(&grid) };
        Grid(dst)
    }

    // pub fn from_p2<const DST_SZ: usize, const SRC_DIM: usize, const DST_DIM: usize>(
    //     input: &str,
    // ) -> Grid<DST_SZ, DST_DIM> {
    //     let mut grid = [MaybeUninit::<u8>::uninit(); DST_SZ];
    // }

    #[inline]
    pub fn get(&self, pt: Point<W>) -> u8 {
        unsafe { *self.0.get_unchecked(pt.0 as usize) }
    }

    #[inline]
    pub fn set(&mut self, pt: Point<W>, val: u8) {
        unsafe { *self.0.get_unchecked_mut(pt.0 as usize) = val };
    }
}

impl<const SZ: usize, const W: usize> ToString for Grid<SZ, W> {
    fn to_string(&self) -> String {
        let mut str = String::with_capacity(SZ + W);

        for row in 0..SZ / W {
            for col in 0..W {
                str.push(self.0[row * W + col] as char)
            }
            str.push('\n')
        }

        str
    }
}

#[derive(Copy, Clone)]
struct Point<const W: usize>(u32);

impl<const W: usize> Point<W> {
    #[inline]
    pub fn left(self) -> Self {
        Point(self.0 - 1)
    }

    #[inline]
    pub fn right(self) -> Self {
        Point(self.0 + 1)
    }

    #[inline]
    pub fn up(self) -> Self {
        Point(self.0 - W as u32)
    }

    #[inline]
    pub fn down(self) -> Self {
        Point(self.0 + W as u32)
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

#[inline]
fn find_robot_p1<const SZ: usize, const W: usize>(grid: &mut Grid<SZ, W>) -> Point<W> {
    for i in 0..SZ {
        if grid.0[i] == b'@' {
            grid.0[i] = b'.';
            return Point(i as u32);
        }
    }

    unsafe { std::hint::unreachable_unchecked() };
}

macro_rules! inner_p1_step {
    ($grid: ident, $robot: ident, $pmove: ident) => {{
        let pos = $robot.$pmove();

        match $grid.get(pos) {
            b'.' => $robot = pos,
            b'#' => (),
            b'O' => {
                let mut box_pos = pos;

                loop {
                    if $grid.get(box_pos) == b'.' {
                        $grid.set(pos, b'.');
                        $grid.set(box_pos, b'O');
                        $robot = pos;
                        break;
                    } else if $grid.get(box_pos) == b'#' {
                        break;
                    } else {
                        unsafe { std::hint::assert_unchecked($grid.get(box_pos) == b'O') };
                    }

                    box_pos = box_pos.$pmove();
                }
            }
            _ => unsafe { std::hint::unreachable_unchecked() },
        }
    }};
}

fn inner_p1<const SZ: usize, const W: usize, const DIR_LINES: usize, const DIR_LENGTH: usize>(
    input: &str,
) -> u32 {
    let mut grid = Grid::<SZ, W>::from_p1::<SZ, W>(input);
    let mut robot = find_robot_p1(&mut grid);
    let mut dirs =
        Dirs::<DIR_LINES, DIR_LENGTH>::new(unsafe { input.as_bytes().as_ptr().add(SZ + W + 1) });

    for _ in 0..DIR_LINES {
        for _ in 0..DIR_LENGTH {
            match dirs.next() {
                b'<' => inner_p1_step!(grid, robot, left),
                b'>' => inner_p1_step!(grid, robot, right),
                b'^' => inner_p1_step!(grid, robot, up),
                b'v' => inner_p1_step!(grid, robot, down),
                _ => unsafe { std::hint::unreachable_unchecked() },
            }
        }

        dirs.next_line();
    }

    let mut sum = 0;
    for i in 0..SZ {
        if grid.get(Point(i as u32)) == b'O' {
            sum += (i / W) * 100 + (i % W)
        }
    }
    sum as u32
}

pub fn part1(input: &str) -> u32 {
    inner_p1::<2500, 50, 20, 1000>(input)
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
        assert_eq!(inner_p1::<100, 10, 10, 70>(TEST_1), 10092);
    }

    #[test]
    fn test_a2() {
        assert_eq!(inner_p1::<64, 8, 1, 15>(TEST_2), 2028);
    }

    // #[test]
    // fn test_b1() {
    //     assert_eq!(part2(TEST_1), 9021);
    // }
}
