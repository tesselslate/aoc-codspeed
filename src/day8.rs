const LEN: usize = 50;
const SZ: usize = LEN * LEN;
const NODES: usize = 128;

#[derive(Copy, Clone)]
pub struct Bitmap([u8; SZ]);

#[derive(Copy, Clone)]
pub struct Point(pub i32, pub i32);

pub struct Points {
    data: [[Point; 4]; NODES],
    len: [u32; NODES],
}

impl Default for Bitmap {
    fn default() -> Self {
        Bitmap([0; SZ])
    }
}

impl Bitmap {
    #[inline]
    pub fn set(&mut self, pt: Point) {
        self.0[pt.0 as usize * LEN + pt.1 as usize] = 1;
    }

    #[inline]
    pub fn sum(&self) -> usize {
        self.0.iter().filter(|&&x| x != 0).count()
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

pub fn parse(input: &str, points: &mut Points) {
    let input = input.as_bytes();

    fn process_line(line: &[u8], row: i32, points: &mut Points) {
        debug_assert!(line.len() == 50);

        line.iter().enumerate().for_each(|(i, c)| {
            if *c != b'.' {
                let idx = *c as usize;

                points.data[idx][points.len[idx] as usize] = Point(row, i as i32);
                points.len[idx] += 1;
            }
        });
    }

    let mut offset = 0;
    for row in 0..LEN {
        process_line(&input[offset..offset + LEN], row as i32, points);
        offset += LEN + 1;
    }
}

pub fn match2_1(a: Point, b: Point, bitmap: &mut Bitmap) {
    let dr = a.0 - b.0;
    let dc = a.1 - b.1;

    let p = Point(a.0 + dr, a.1 + dc);
    if p.0 >= 0 && p.1 >= 0 && p.0 < LEN as i32 && p.1 < LEN as i32 {
        bitmap.set(p);
    }
}

pub fn match3_1(points: &[Point; 4], bitmap: &mut Bitmap) {
    match2_1(points[0], points[1], bitmap);
    match2_1(points[0], points[2], bitmap);
    match2_1(points[1], points[0], bitmap);
    match2_1(points[1], points[2], bitmap);
    match2_1(points[2], points[0], bitmap);
    match2_1(points[2], points[1], bitmap);
}

pub fn match4_1(points: &[Point; 4], bitmap: &mut Bitmap) {
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

pub fn match_1(points: &[Point; 4], len: u32, bitmap: &mut Bitmap) {
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

pub fn antinodes_1(points: &Points, bitmap: &mut Bitmap) {
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

pub fn match2_2(a: Point, b: Point, bitmap: &mut Bitmap) {
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

pub fn match_2(points: &[Point; 4], len: u32, bitmap: &mut Bitmap) {
    for i in 0..len {
        for j in 0..len {
            if i != j {
                match2_2(points[i as usize], points[j as usize], bitmap);
            }
        }
    }
}

pub fn antinodes_2(points: &Points, bitmap: &mut Bitmap) {
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

pub fn part1(input: &str) -> usize {
    let mut points = Points::default();
    let mut bitmap = Bitmap::default();
    parse(input, &mut points);
    antinodes_1(&points, &mut bitmap);

    bitmap.sum()
}

pub fn part2(input: &str) -> usize {
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
