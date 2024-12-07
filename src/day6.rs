use std::{
    ops::{BitAnd, BitOrAssign, Shr},
    simd::u64x4,
};

const LEN: usize = 130;
const LEN_I: isize = LEN as isize;
const SZ: usize = LEN * LEN;
const BITMAP_U64_COUNT: usize = ((SZ.div_ceil(64) + 3) / 4) * 4;

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

struct Grid([bool; SZ]);

impl Grid {
    #[inline(always)]
    pub fn get(&self, pos: (isize, isize)) -> bool {
        self.0[pos.0 as usize * LEN + pos.1 as usize]
    }

    #[inline(always)]
    pub fn set(&mut self, pos: (isize, isize), state: bool) {
        self.0[pos.0 as usize * LEN + pos.1 as usize] = state;
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid([false; SZ])
    }
}

#[derive(Copy, Clone)]
struct Visited([u64; BITMAP_U64_COUNT * 4]);

impl Visited {
    #[inline(always)]
    pub fn contains(&self, dir: usize, pos: (isize, isize)) -> bool {
        let pos = (pos.0 * LEN as isize + pos.1) as usize * 4 + dir;
        let idx = pos / 64;
        let bit = pos % 64;

        return ((self.0[idx] >> bit) & 1) == 1;
    }

    #[inline(always)]
    pub fn mark(&mut self, dir: usize, pos: (isize, isize)) {
        let pos = (pos.0 * LEN as isize + pos.1) as usize * 4 + dir;
        let idx = pos / 64;
        let bit = pos % 64;

        self.0[idx] |= 1 << bit;
    }

    #[inline(always)]
    pub fn sum(&self) -> u32 {
        let mask1 = u64x4::splat(0x1111111111111111);
        let mask2 = u64x4::splat(0x2222222222222222);
        let mask3 = u64x4::splat(0x4444444444444444);
        let mask4 = u64x4::splat(0x8888888888888888);

        self.0
            .into_iter()
            .array_chunks::<4>()
            .map(|x| {
                let mut sum = u64x4::splat(0);
                let xs = u64x4::from(x);
                sum.bitor_assign(xs.bitand(mask1));
                sum.bitor_assign(xs.bitand(mask2).shr(1));
                sum.bitor_assign(xs.bitand(mask3).shr(2));
                sum.bitor_assign(xs.bitand(mask4).shr(3));

                sum.as_array().iter().map(|x| x.count_ones()).sum::<u32>()
            })
            .sum()
    }
}

impl Default for Visited {
    fn default() -> Self {
        Self([0; BITMAP_U64_COUNT * 4])
    }
}

fn parse(input: &str) -> ((isize, isize), Grid) {
    let mut grid = Grid::default();
    let mut start_pos = None;

    let lines = input.lines();
    lines.enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, char)| {
            if char == '#' {
                grid.set((i as isize, j as isize), true);
            }

            if char == '^' {
                start_pos = Some((i as isize, j as isize));
            }
        });
    });

    (start_pos.expect("no start pos"), grid)
}

#[inline(always)]
fn rotate(mut x: (isize, isize)) -> (isize, isize) {
    let x0 = x.0;
    x.0 = x.1;
    x.1 = -x0;
    return x;
}

fn navigate(grid: &Grid, mut pos: (isize, isize), mut dir: usize, visited: &mut Visited) -> bool {
    let mut dd = DIRS[dir];

    loop {
        let new = (pos.0 + dd.0, pos.1 + dd.1);
        if new.0 < 0 || new.1 < 0 || new.0 >= LEN as isize || new.1 >= LEN as isize {
            return false;
        }

        if grid.get(new) {
            if visited.contains(dir, pos) {
                return true;
            }
            visited.mark(dir, pos);

            dir = (dir + 1) % 4;
            dd = rotate(dd);
        } else {
            pos = new;
        }
    }
}

fn navigate_p1<const N: isize>(grid: &Grid, mut pos: (isize, isize)) -> u32 {
    let mut dir = 0;
    let mut dd = DIRS[0];

    let mut visited = Visited::default();

    loop {
        visited.mark(dir, pos);

        let new = (pos.0 + dd.0, pos.1 + dd.1);
        if new.0 < 0 || new.1 < 0 || new.0 >= N || new.1 >= N {
            return visited.sum();
        }

        if grid.get(new) {
            dir = (dir + 1) % 4;
            dd = rotate(dd);
        } else {
            pos = new;
        }
    }
}

fn find_obstructions(grid: &mut Grid, start: (isize, isize)) -> u32 {
    let mut visited = Visited::default();
    let mut visited_any = [0u64; BITMAP_U64_COUNT];
    let mut obstructions = 0;

    let mut pos = start;
    let mut dir = 0;
    let mut dd = DIRS[0];

    loop {
        visited.mark(dir, pos);

        let x = (pos.0 * LEN as isize + pos.1) as usize;
        let idx = x / 64;
        let bit = x % 64;
        visited_any[idx] |= 1 << bit;

        let new = (pos.0 + dd.0, pos.1 + dd.1);
        if new.0 < 0 || new.1 < 0 || new.0 >= LEN as isize || new.1 >= LEN as isize {
            return obstructions;
        }

        if grid.get(new) {
            dir = (dir + 1) % 4;
            dd = rotate(dd);
        } else {
            let x = (new.0 * LEN as isize + new.1) as usize;
            let idx = x / 64;
            let bit = x % 64;

            if ((visited_any[idx] >> bit) & 1) == 0 {
                grid.set(new, true);
                if visited.contains((dir + 1) % 4, pos)
                    || navigate(grid, pos, (dir + 1) % 4, &mut visited.clone())
                {
                    obstructions += 1;
                }
                grid.set(new, false);
            }

            pos = new;
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let (start_pos, grid) = parse(input);
    navigate_p1::<LEN_I>(&grid, start_pos)
}

pub fn part2(input: &str) -> u32 {
    let (start_pos, mut grid) = parse(input);
    find_obstructions(&mut grid, start_pos)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input6.txt");
    const TEST: &str = include_str!("../testdata/input6.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 4515);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 1309);
    }

    #[test]
    fn test_a() {
        let (start_pos, grid) = parse(TEST);
        assert_eq!(navigate_p1::<10>(&grid, start_pos), 41);
    }

    #[test]
    fn test_b() {
        assert_eq!(part2(TEST), 6);
    }
}
