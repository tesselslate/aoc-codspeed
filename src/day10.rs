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

fn recurse_p1(visited: &mut VisitMap, sum: &mut u32, map: &Map, r: usize, c: usize, value: i8) {
    visited.mark(r, c);

    let adj = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];
    for (r, c) in adj {
        let adj_value = map.get(r, c);

        if adj_value == value + 1 {
            if visited.has(r, c) {
                continue;
            }

            if adj_value == 9 {
                visited.mark(r, c);
                *sum += 1;
            } else {
                recurse_p1(visited, sum, map, r, c, adj_value);
            }
        }
    }
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
                    recurse_p1(&mut visited, &mut sum, &map, r, c, 1);
                }
            }
        }
    }

    sum
}

fn recurse_p2(memo: &mut Memo, map: &Map, r: usize, c: usize, value: i8) -> u32 {
    let adj = [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];

    let mut acc = 0;
    for (r, c) in adj {
        let adj_value = map.get(r, c);

        if adj_value == value + 1 {
            if adj_value == 9 {
                acc += 1;
            } else {
                if let Some(x) = memo.get(r, c) {
                    acc += x;
                    continue;
                }

                let ret = recurse_p2(memo, map, r, c, adj_value);
                memo.set(r, c, ret);
                acc += ret;
            }
        }
    }
    acc
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
                    sum += recurse_p2(&mut memo, &map, r, c, 1);
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
