const MAP_LEN: usize = 64;
const MAP_SZ: usize = MAP_LEN * MAP_LEN;
const MAP_ROW_OFFSET: usize = 1;

struct VisitMap([u16; MAP_SZ], u16);

impl Default for VisitMap {
    fn default() -> Self {
        Self([0; MAP_SZ], 0)
    }
}

impl VisitMap {
    #[inline]
    pub fn has(&mut self, row: usize, col: usize) -> bool {
        self.0[row * MAP_LEN + col] == self.1
    }

    #[inline]
    pub fn mark(&mut self, row: usize, col: usize) {
        self.0[row * MAP_LEN + col] = self.1
    }

    #[inline]
    pub fn increment(&mut self) {
        self.1 += 1;
    }
}

struct Map([u8; MAP_SZ]);

impl Map {
    pub fn new<const LEN: usize>(input: &[u8]) -> Self {
        let mut map = Map([0xFF; MAP_SZ]);

        for row in 0..LEN {
            let dst_start = (row + 1) * MAP_LEN + MAP_ROW_OFFSET;
            let src_start = row * (LEN + 1);

            map.0[dst_start..dst_start + LEN].copy_from_slice(&input[src_start..src_start + LEN]);
        }

        map
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> i8 {
        (self.0[row * MAP_LEN + col] - b'0') as i8
    }
}

struct Memo([u8; MAP_SZ]);

impl Default for Memo {
    fn default() -> Self {
        Memo([0xFF; MAP_SZ])
    }
}

impl Memo {
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Option<u32> {
        let x = self.0[row * MAP_LEN + col];

        if x == 0xFF {
            None
        } else {
            Some(x as u32)
        }
    }

    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: u32) {
        self.0[row * MAP_LEN + col] = value as u8;
    }
}

macro_rules! recurse_p1_impl {
    ($func_name:ident,$func_next:ident,$next_value:literal) => {
        fn $func_name<const L: bool, const R: bool, const U: bool, const D: bool>(
            visited: &mut VisitMap,
            sum: &mut u32,
            map: &Map,
            r: usize,
            c: usize,
        ) {
            visited.mark(r, c);

            if L && map.get(r, c - 1) == $next_value && !visited.has(r, c - 1) {
                $func_next::<true, false, true, true>(visited, sum, map, r, c - 1);
            }
            if R && map.get(r, c + 1) == $next_value && !visited.has(r, c + 1) {
                $func_next::<false, true, true, true>(visited, sum, map, r, c + 1);
            }
            if U && map.get(r - 1, c) == $next_value && !visited.has(r - 1, c) {
                $func_next::<true, true, true, false>(visited, sum, map, r - 1, c);
            }
            if D && map.get(r + 1, c) == $next_value && !visited.has(r + 1, c) {
                $func_next::<true, true, false, true>(visited, sum, map, r + 1, c);
            }
        }
    };
}

recurse_p1_impl!(recurse_p1_1, recurse_p1_2, 2);
recurse_p1_impl!(recurse_p1_2, recurse_p1_3, 3);
recurse_p1_impl!(recurse_p1_3, recurse_p1_4, 4);
recurse_p1_impl!(recurse_p1_4, recurse_p1_5, 5);
recurse_p1_impl!(recurse_p1_5, recurse_p1_6, 6);
recurse_p1_impl!(recurse_p1_6, recurse_p1_7, 7);
recurse_p1_impl!(recurse_p1_7, recurse_p1_8, 8);
recurse_p1_impl!(recurse_p1_8, recurse_p1_9, 9);

#[inline(always)]
fn recurse_p1_9<const L: bool, const R: bool, const U: bool, const D: bool>(
    visited: &mut VisitMap,
    sum: &mut u32,
    _: &Map,
    r: usize,
    c: usize,
) {
    visited.mark(r, c);
    *sum += 1;
}

fn inner_p1<const LEN: usize>(input: &[u8]) -> u32 {
    let map = Map::new::<LEN>(input);

    let mut visited = VisitMap::default();
    let mut sum = 0;
    for r in MAP_ROW_OFFSET..MAP_ROW_OFFSET + LEN {
        for c in MAP_ROW_OFFSET..MAP_ROW_OFFSET + LEN {
            if map.get(r, c) != 0 {
                continue;
            }

            visited.increment();
            visited.mark(r as usize, c as usize);

            let adj = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];
            for (r, c) in adj {
                if map.get(r, c) == 1 {
                    recurse_p1_1::<true, true, true, true>(&mut visited, &mut sum, &map, r, c);
                }
            }
        }
    }

    sum
}

macro_rules! recurse_p2_impl {
    ($func_name:ident,$func_next:ident,$next_value:literal) => {
        fn $func_name<const L: bool, const R: bool, const U: bool, const D: bool>(
            memo: &mut Memo,
            map: &Map,
            r: usize,
            c: usize,
        ) -> u32 {
            let mut acc = 0;

            if L && map.get(r, c - 1) == $next_value {
                if let Some(val) = memo.get(r, c - 1) {
                    acc += val;
                } else {
                    acc += $func_next::<true, false, true, true>(memo, map, r, c - 1);
                }
            }
            if R && map.get(r, c + 1) == $next_value {
                if let Some(val) = memo.get(r, c + 1) {
                    acc += val;
                } else {
                    acc += $func_next::<false, true, true, true>(memo, map, r, c + 1);
                }
            }
            if U && map.get(r - 1, c) == $next_value {
                if let Some(val) = memo.get(r - 1, c) {
                    acc += val;
                } else {
                    acc += $func_next::<true, true, true, false>(memo, map, r - 1, c);
                }
            }
            if D && map.get(r + 1, c) == $next_value {
                if let Some(val) = memo.get(r + 1, c) {
                    acc += val;
                } else {
                    acc += $func_next::<true, true, false, true>(memo, map, r + 1, c);
                }
            }

            memo.set(r, c, acc);
            acc
        }
    };
}

recurse_p2_impl!(recurse_p2_1, recurse_p2_2, 2);
recurse_p2_impl!(recurse_p2_2, recurse_p2_3, 3);
recurse_p2_impl!(recurse_p2_3, recurse_p2_4, 4);
recurse_p2_impl!(recurse_p2_4, recurse_p2_5, 5);
recurse_p2_impl!(recurse_p2_5, recurse_p2_6, 6);
recurse_p2_impl!(recurse_p2_6, recurse_p2_7, 7);
recurse_p2_impl!(recurse_p2_7, recurse_p2_8, 8);
recurse_p2_impl!(recurse_p2_8, recurse_p2_9, 9);

#[inline(always)]
fn recurse_p2_9<const L: bool, const R: bool, const U: bool, const D: bool>(
    memo: &mut Memo,
    _: &Map,
    r: usize,
    c: usize,
) -> u32 {
    memo.set(r, c, 1);
    1
}

fn inner_p2<const LEN: usize>(input: &[u8]) -> u32 {
    let map = Map::new::<LEN>(input);

    let mut sum = 0;
    let mut memo = Memo::default();
    for r in MAP_ROW_OFFSET..MAP_ROW_OFFSET + LEN {
        for c in MAP_ROW_OFFSET..MAP_ROW_OFFSET + LEN {
            if map.get(r, c) != 0 {
                continue;
            }

            let adj = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];
            for (r, c) in adj {
                if map.get(r, c) == 1 {
                    sum += recurse_p2_1::<true, true, true, true>(&mut memo, &map, r, c);
                }
            }
        }
    }

    sum
}

pub fn part1(input: &str) -> u32 {
    inner_p1::<54>(input.as_bytes())
}

pub fn part2(input: &str) -> u32 {
    inner_p2::<54>(input.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input10.txt");
    const TEST: &str = include_str!("../testdata/input10.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 733);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 1514);
    }

    #[test]
    fn test_a() {
        assert_eq!(inner_p1::<8>(TEST.as_bytes()), 36);
    }

    #[test]
    fn test_b() {
        assert_eq!(inner_p2::<8>(TEST.as_bytes()), 81);
    }
}
