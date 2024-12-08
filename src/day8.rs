use memchr::memchr;

const LEN: usize = 50;
const SZ: usize = LEN * LEN;
const BSZ: usize = ((SZ.div_ceil(64) + 3) / 4) * 4;
const NODES: usize = 128;

#[derive(Copy, Clone)]
struct Bitmap([u64; BSZ]);

#[derive(Copy, Clone)]
struct Point(i32, i32);

struct Points {
    data: [[Point; 4]; NODES],
    len: [u32; NODES],
}

impl Default for Bitmap {
    fn default() -> Self {
        Bitmap([0; BSZ])
    }
}

impl Bitmap {
    #[inline(always)]
    pub fn set(&mut self, pt: Point) {
        let pos = pt.0 as usize * LEN + pt.1 as usize;
        let idx = pos / 64;
        let bit = pos % 64;

        self.0[idx] |= 1 << bit;
    }

    #[inline]
    pub fn sum(&self) -> u32 {
        self.0.iter().map(|x| x.count_ones()).sum()
    }
}

impl Default for Points {
    fn default() -> Self {
        Points {
            data: [[Point(0, 0); 4]; NODES],
            len: [0; NODES],
        }
    }
}

fn parse(input: &str, points: &mut Points) {
    let mut input = input.as_bytes();
    let mut row = 0;

    fn process_line(line: &[u8], row: i32, points: &mut Points) {
        line.iter().enumerate().for_each(|(i, c)| {
            if *c != b'.' {
                let idx = *c as usize;

                points.data[idx][points.len[idx] as usize] = Point(row, i as i32);
                points.len[idx] += 1;
            }
        });
    }

    while let Some(end) = memchr(b'\n', input) {
        process_line(&input[..end], row, points);

        row += 1;
        input = &input[end + 1..];
    }
    process_line(input, row, points);
}

fn match2_1(a: Point, b: Point, bitmap: &mut Bitmap) {
    let dr = a.0 - b.0;
    let dc = a.1 - b.1;

    let p = Point(a.0 + dr, a.1 + dc);
    if p.0 >= 0 && p.1 >= 0 && p.0 < LEN as i32 && p.1 < LEN as i32 {
        bitmap.set(p);
    }
}

fn match3_1(points: &[Point; 4], bitmap: &mut Bitmap) {
    match2_1(points[0], points[1], bitmap);
    match2_1(points[0], points[2], bitmap);
    match2_1(points[1], points[0], bitmap);
    match2_1(points[1], points[2], bitmap);
    match2_1(points[2], points[0], bitmap);
    match2_1(points[2], points[1], bitmap);
}

fn match4_1(points: &[Point; 4], bitmap: &mut Bitmap) {
    match2_1(points[0], points[1], bitmap);
    match2_1(points[0], points[2], bitmap);
    match2_1(points[0], points[3], bitmap);
    match2_1(points[1], points[0], bitmap);
    match2_1(points[1], points[2], bitmap);
    match2_1(points[1], points[3], bitmap);
    match2_1(points[2], points[0], bitmap);
    match2_1(points[2], points[1], bitmap);
    match2_1(points[2], points[3], bitmap);
    match2_1(points[3], points[0], bitmap);
    match2_1(points[3], points[1], bitmap);
    match2_1(points[3], points[2], bitmap);
}

fn match_1(points: &[Point; 4], len: u32, bitmap: &mut Bitmap) {
    match len {
        2 => {
            match2_1(points[0], points[1], bitmap);
            match2_1(points[1], points[0], bitmap);
        }
        3 => match3_1(points, bitmap),
        4 => match4_1(points, bitmap),
        _ => return,
    }
}

fn antinodes_1(points: &Points, bitmap: &mut Bitmap) {
    for i in b'0'..=b'9' {
        match_1(&points.data[i as usize], points.len[i as usize], bitmap);
    }
    for i in b'A'..=b'Z' {
        match_1(&points.data[i as usize], points.len[i as usize], bitmap);
    }
    for i in b'a'..=b'z' {
        match_1(&points.data[i as usize], points.len[i as usize], bitmap);
    }
}

fn match2_2(a: Point, b: Point, bitmap: &mut Bitmap) {
    let dr = a.0 - b.0;
    let dc = a.1 - b.1;

    let mut p = a;
    loop {
        if !(p.0 >= 0 && p.1 >= 0 && p.0 < LEN as i32 && p.1 < LEN as i32) {
            return;
        }

        bitmap.set(p);
        p = Point(p.0 + dr, p.1 + dc);
    }
}

fn match_2(points: &[Point; 4], len: u32, bitmap: &mut Bitmap) {
    for i in 0..len {
        for j in 0..len {
            if i != j {
                match2_2(points[i as usize], points[j as usize], bitmap);
            }
        }
    }
}

fn antinodes_2(points: &Points, bitmap: &mut Bitmap) {
    for i in b'0'..=b'9' {
        match_2(&points.data[i as usize], points.len[i as usize], bitmap);
    }
    for i in b'A'..=b'Z' {
        match_2(&points.data[i as usize], points.len[i as usize], bitmap);
    }
    for i in b'a'..=b'z' {
        match_2(&points.data[i as usize], points.len[i as usize], bitmap);
    }
}

pub fn part1(input: &str) -> u32 {
    let mut points = Points::default();
    let mut bitmap = Bitmap::default();
    parse(input, &mut points);
    antinodes_1(&points, &mut bitmap);

    bitmap.sum()
}

pub fn part2(input: &str) -> u32 {
    let mut points = Points::default();
    let mut bitmap = Bitmap::default();
    parse(input, &mut points);
    antinodes_2(&points, &mut bitmap);

    bitmap.sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input8.txt");
    const TEST: &str = include_str!("../testdata/input8.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 240);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 955);
    }

    // #[test]
    // fn test_a() {
    //     assert_eq!(part1(&TEST[..TEST.len() - 1]), 14);
    // }
    //
    // #[test]
    // fn test_b() {
    //     assert_eq!(part2(&TEST[..TEST.len() - 1]), 34);
    // }
}
