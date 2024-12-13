const GRID_LEN: usize = 140;
const GRID_PLEN: usize = GRID_LEN + 2;
const GRID_SIZE: usize = GRID_PLEN * GRID_PLEN;

struct PadGrid<const LEN: usize>([u8; GRID_SIZE]);

impl<const LEN: usize> PadGrid<LEN> {
    pub fn new(input: &[u8]) -> Self {
        let mut grid = PadGrid([0; GRID_SIZE]);

        for r in 0..LEN {
            let dst_start = (r + 1) * (LEN + 2) + 1;
            let src_start = r * (LEN + 1);

            grid.0[dst_start..dst_start + LEN].copy_from_slice(&input[src_start..src_start + LEN]);
        }

        grid
    }

    #[inline]
    pub fn get(&self, pt: Point<LEN>) -> u8 {
        unsafe { *self.0.get_unchecked(pt.0 as usize) }
    }
}

struct Boolmap<const LEN: usize>([bool; GRID_SIZE]);

impl<const LEN: usize> Default for Boolmap<LEN> {
    fn default() -> Self {
        Boolmap([false; GRID_SIZE])
    }
}

impl<const LEN: usize> Boolmap<LEN> {
    #[inline]
    pub fn get(&self, pt: Point<LEN>) -> bool {
        unsafe { *self.0.get_unchecked(pt.0 as usize) }
    }

    #[inline]
    pub fn set(&mut self, pt: Point<LEN>) {
        unsafe {
            *self.0.get_unchecked_mut(pt.0 as usize) = true;
        }
    }
}

#[derive(Copy, Clone)]
struct Point<const LEN: usize>(u32);

impl<const LEN: usize> Point<LEN> {
    #[inline]
    pub fn new(row: usize, col: usize) -> Self {
        Point((row * (LEN + 2) + col) as u32)
    }

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
        Point(self.0 - (LEN + 2) as u32)
    }

    #[inline]
    pub fn down(self) -> Self {
        Point(self.0 + (LEN + 2) as u32)
    }
}

macro_rules! dfs_p1_inner {
    ($grid: ident, $visited: ident, $area: ident, $peri: ident, $pt: ident, $id: ident, $move: ident, $L: literal, $R: literal, $U: literal, $D: literal, $check: ident) => {
        if $check {
            let new_id = $grid.get($pt.$move());
            if new_id == $id && !$visited.get($pt.$move()) {
                dfs_p1::<LEN, $L, $R, $U, $D>($grid, $visited, $area, $peri, $pt.$move(), $id);
            } else if new_id != $id {
                *$peri += 1;
            }
        }
    };
}

fn dfs_p1<const LEN: usize, const L: bool, const R: bool, const U: bool, const D: bool>(
    grid: &PadGrid<LEN>,
    visited: &mut Boolmap<LEN>,
    area: &mut u32,
    peri: &mut u32,
    pt: Point<LEN>,
    id: u8,
) {
    visited.set(pt);
    *area += 1;

    dfs_p1_inner!(grid, visited, area, peri, pt, id, down, true, true, false, true, D);
    dfs_p1_inner!(grid, visited, area, peri, pt, id, up, true, true, true, false, U);
    dfs_p1_inner!(grid, visited, area, peri, pt, id, right, false, true, true, true, R);
    dfs_p1_inner!(grid, visited, area, peri, pt, id, left, true, false, true, true, L);
}

fn inner_p1<const LEN: usize>(input: &str) -> u32 {
    let grid = PadGrid::new(input.as_bytes());

    let mut visited = Boolmap::default();
    let mut sum = 0;
    for r in 1..=LEN {
        for c in 1..=LEN {
            let pt = Point::new(r, c);
            if visited.get(pt) {
                continue;
            }

            let id = grid.get(pt);
            debug_assert!(id != 0);

            let (mut area, mut peri) = (0, 0);
            dfs_p1::<LEN, true, true, true, true>(
                &grid,
                &mut visited,
                &mut area,
                &mut peri,
                pt,
                id,
            );
            sum += area * peri;
        }
    }

    sum
}

macro_rules! dfs_p2_inner {
    ($grid: ident, $visited: ident, $area: ident, $peri: ident, $pt: ident, $id: ident, $move: ident, $pmove: ident, $L: literal, $R: literal, $U: literal, $D: literal, $check: ident) => {
        if $check {
            let new_id = $grid.get($pt.$move());
            if new_id == $id && !$visited.get($pt.$move()) {
                dfs_p2::<LEN, $L, $R, $U, $D>($grid, $visited, $area, $peri, $pt.$move(), $id);
            } else if new_id != $id {
                let a = $grid.get($pt.$move().$pmove()) == $id;
                let b = $grid.get($pt.$pmove()) == $id;
                *$peri += (a || !b) as u32;
            }
        }
    };
}

fn dfs_p2<const LEN: usize, const L: bool, const R: bool, const U: bool, const D: bool>(
    grid: &PadGrid<LEN>,
    visited: &mut Boolmap<LEN>,
    area: &mut u32,
    peri: &mut u32,
    pt: Point<LEN>,
    id: u8,
) {
    visited.set(pt);
    *area += 1;

    dfs_p2_inner!(grid, visited, area, peri, pt, id, down, left, true, true, false, true, D);
    dfs_p2_inner!(grid, visited, area, peri, pt, id, up, left, true, true, true, false, U);
    dfs_p2_inner!(grid, visited, area, peri, pt, id, right, up, false, true, true, true, R);
    dfs_p2_inner!(grid, visited, area, peri, pt, id, left, up, true, false, true, true, L);
}

fn inner_p2<const LEN: usize>(input: &str) -> u32 {
    let grid = PadGrid::new(input.as_bytes());

    let mut visited = Boolmap::default();
    let mut sum = 0;
    for r in 1..=LEN {
        for c in 1..=LEN {
            let pt = Point::new(r, c);
            if visited.get(pt) {
                continue;
            }

            let id = grid.get(pt);
            debug_assert!(id != 0);

            let (mut area, mut peri) = (0, 0);
            dfs_p2::<LEN, true, true, true, true>(
                &grid,
                &mut visited,
                &mut area,
                &mut peri,
                pt,
                id,
            );
            sum += area * peri;
        }
    }

    sum
}

pub fn part1(input: &str) -> u32 {
    inner_p1::<GRID_LEN>(input)
}

pub fn part2(input: &str) -> u32 {
    inner_p2::<GRID_LEN>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input12.txt");
    const TEST: &str = include_str!("../testdata/input12.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 1467094);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 881182);
    }

    #[test]
    fn test_a() {
        assert_eq!(inner_p1::<10>(TEST), 1930);
    }

    #[test]
    fn test_b() {
        assert_eq!(inner_p2::<10>(TEST), 1206);
    }
}
