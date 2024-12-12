const GRID_LEN: isize = 140;
const GRID_PLEN: isize = GRID_LEN + 2;
const GRID_SIZE: isize = GRID_PLEN * GRID_PLEN;
const GRID_BSZ: usize = (GRID_SIZE as usize).div_ceil(64);

struct Grid<'a, const LEN: isize>(&'a [u8]);

impl<'a, const LEN: isize> Grid<'a, LEN> {
    #[inline]
    pub fn get(&self, row: isize, col: isize) -> u8 {
        let pos = row * (LEN + 1) + col;

        // this bounds check *requires* a trailing newline
        if pos < 0 || pos >= (LEN + 1) * LEN {
            0
        } else {
            self.0[pos as usize]
        }
    }
}

struct Bitmap([u64; GRID_BSZ]);

impl Default for Bitmap {
    fn default() -> Self {
        Bitmap([0; GRID_BSZ])
    }
}

impl Bitmap {
    #[inline]
    pub fn get(&self, row: isize, col: isize) -> bool {
        let pos = ((row + 1) * GRID_PLEN + (col + 1)) as usize;
        let idx = pos / 64;
        let bit = pos % 64;

        (self.0[idx] & (1 << bit)) != 0
    }

    #[inline]
    pub fn set(&mut self, row: isize, col: isize) {
        let pos = ((row + 1) * GRID_PLEN + (col + 1)) as usize;
        let idx = pos / 64;
        let bit = pos % 64;

        self.0[idx] |= 1 << bit;
    }
}

fn dfs_p1<const LEN: isize>(
    grid: &Grid<LEN>,
    visited: &mut Bitmap,
    area: &mut u32,
    peri: &mut u32,
    row: isize,
    col: isize,
    id: u8,
) {
    visited.set(row, col);
    *area += 1;

    for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let new_id = grid.get(row + dr, col + dc);

        if new_id == id && !visited.get(row + dr, col + dc) {
            dfs_p1(grid, visited, area, peri, row + dr, col + dc, id);
        } else if new_id != id {
            *peri += 1;
        }
    }
}

fn inner_p1<const LEN: isize>(input: &str) -> u32 {
    let grid = Grid::<LEN>(input.as_bytes());

    let mut visited = Bitmap::default();
    let mut sum = 0;
    for r in 0..LEN {
        for c in 0..LEN {
            if visited.get(r, c) {
                continue;
            }

            let id = grid.get(r, c);
            debug_assert!(id != 0);

            let (mut area, mut peri) = (0, 0);
            dfs_p1(&grid, &mut visited, &mut area, &mut peri, r, c, id);
            sum += area * peri;
        }
    }

    sum
}

fn dfs_p2<const LEN: isize>(
    grid: &Grid<LEN>,
    visited: &mut Bitmap,
    area: &mut u32,
    peri: &mut u32,
    row: isize,
    col: isize,
    id: u8,
) {
    visited.set(row, col);
    *area += 1;

    for dr in [-1, 1] {
        let new_id = grid.get(row + dr, col);

        if new_id == id && !visited.get(row + dr, col) {
            dfs_p2(grid, visited, area, peri, row + dr, col, id);
        } else if new_id != id {
            let a = grid.get(row + dr, col - 1) == id;
            let b = grid.get(row, col - 1) == id;
            *peri += (a || !b) as u32;
        }
    }

    for dc in [-1, 1] {
        let new_id = grid.get(row, col + dc);

        if new_id == id && !visited.get(row, col + dc) {
            dfs_p2(grid, visited, area, peri, row, col + dc, id);
        } else if new_id != id {
            let a = grid.get(row - 1, col + dc) == id;
            let b = grid.get(row - 1, col) == id;
            *peri += (a || !b) as u32;
        }
    }
}

fn inner_p2<const LEN: isize>(input: &str) -> u32 {
    let grid = Grid::<LEN>(input.as_bytes());

    let mut visited = Bitmap::default();
    let mut sum = 0;
    for r in 0..LEN {
        for c in 0..LEN {
            if visited.get(r, c) {
                continue;
            }

            let id = grid.get(r, c);
            debug_assert!(id != 0);

            let (mut area, mut peri) = (0, 0);
            dfs_p2(&grid, &mut visited, &mut area, &mut peri, r, c, id);
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
