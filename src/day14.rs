use std::{
    hint::unreachable_unchecked,
    mem::MaybeUninit,
    ops::{Add, BitAnd, Div, Mul, Sub},
    simd::{cmp::SimdPartialOrd, i32x8, u32x8},
};

const NUM_ROBOTS: usize = 500;
const NUM_ROBOTS_PAD8: usize = 504;
const STEPS_P1: i32 = 100;
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[repr(C)]
struct Robots {
    x: [i32; NUM_ROBOTS_PAD8],
    y: [i32; NUM_ROBOTS_PAD8],
    vx: [i32; NUM_ROBOTS_PAD8],
    vy: [i32; NUM_ROBOTS_PAD8],
}

#[repr(C)]
struct RobotsUninit {
    x: [MaybeUninit<i32>; NUM_ROBOTS_PAD8],
    y: [MaybeUninit<i32>; NUM_ROBOTS_PAD8],
    vx: [MaybeUninit<i32>; NUM_ROBOTS_PAD8],
    vy: [MaybeUninit<i32>; NUM_ROBOTS_PAD8],
}

impl Default for RobotsUninit {
    fn default() -> Self {
        let mut this = Self {
            x: [MaybeUninit::uninit(); NUM_ROBOTS_PAD8],
            y: [MaybeUninit::uninit(); NUM_ROBOTS_PAD8],
            vx: [MaybeUninit::uninit(); NUM_ROBOTS_PAD8],
            vy: [MaybeUninit::uninit(); NUM_ROBOTS_PAD8],
        };

        for i in NUM_ROBOTS..NUM_ROBOTS_PAD8 {
            this.x[i].write(50);
            this.y[i].write(51);
            this.vx[i].write(0);
            this.vy[i].write(0);
        }

        this
    }
}

#[inline(always)]
unsafe fn parse_pcoord(input: &mut *const u8) -> i32 {
    let a = *input.add(0);
    let b = *input.add(1);

    if b < b'0' {
        *input = input.add(2);
        (a - b'0') as i32
    } else {
        let c = *input.add(2);

        if c < b'0' {
            *input = input.add(3);
            (a - b'0') as i32 * 10 + (b - b'0') as i32
        } else {
            *input = input.add(4);
            (a - b'0') as i32 * 100 + (b - b'0') as i32 * 10 + (c - b'0') as i32
        }
    }
}

#[inline(always)]
unsafe fn parse_vcoord<const DELIM: u8>(input: &mut *const u8) -> i32 {
    let a = *input.add(0);
    let b = *input.add(1);

    if b == DELIM {
        *input = input.add(2);
        (a - b'0') as i32
    } else {
        let neg = a == b'-';
        let c = *input.add(2);

        if c == DELIM {
            *input = input.add(3);
            if neg {
                -((b - b'0') as i32)
            } else {
                (a - b'0') as i32 * 10 + (b - b'0') as i32
            }
        } else {
            *input = input.add(4);
            -((b - b'0') as i32 * 10 + (c - b'0') as i32)
        }
    }
}

#[inline(always)]
unsafe fn inner_p1(input: &[u8]) -> u64 {
    let mut robots = RobotsUninit::default();
    parse(input, &mut robots);
    let robots: Robots = std::mem::transmute(robots);

    let mult = i32x8::splat(STEPS_P1);
    let zero = i32x8::splat(0);

    let width = i32x8::splat(WIDTH);
    let height = i32x8::splat(HEIGHT);

    let width_half = i32x8::splat(WIDTH / 2);
    let height_half = i32x8::splat(HEIGHT / 2);

    let mut quads = [0u64; 4];

    const _: () = assert!(NUM_ROBOTS / 8 == 62);
    for i in 0..NUM_ROBOTS_PAD8 / 8 {
        let xs = i32x8::from_array(*robots.x[i * 8..i * 8 + 8].as_array().unwrap_unchecked());
        let vxs =
            i32x8::from_array(*robots.vx[i * 8..i * 8 + 8].as_array().unwrap_unchecked()).mul(mult);

        let xs = xs.add(vxs);
        let xs = xs.sub(xs.div(width).mul(width));
        let xs = xs.add(xs.simd_lt(zero).select(width, zero));

        let xs_lt = xs.simd_lt(width_half);
        let xs_gt = xs.simd_gt(width_half);

        let ys = i32x8::from_array(*robots.y[i * 8..i * 8 + 8].as_array().unwrap_unchecked());
        let vys =
            i32x8::from_array(*robots.vy[i * 8..i * 8 + 8].as_array().unwrap_unchecked()).mul(mult);

        let ys = ys.add(vys);
        let ys = ys.sub(ys.div(height).mul(height));
        let ys = ys.add(ys.simd_lt(zero).select(height, zero));

        let ys_lt = ys.simd_lt(height_half);
        let ys_gt = ys.simd_gt(height_half);

        quads[0] += xs_lt.bitand(ys_lt).to_bitmask().count_ones() as u64;
        quads[1] += xs_lt.bitand(ys_gt).to_bitmask().count_ones() as u64;
        quads[2] += xs_gt.bitand(ys_lt).to_bitmask().count_ones() as u64;
        quads[3] += xs_gt.bitand(ys_gt).to_bitmask().count_ones() as u64;
    }

    quads[0] * quads[1] * quads[2] * quads[3]
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input.as_bytes()) }
}

unsafe fn parse(input: &[u8], robots: &mut RobotsUninit) {
    let mut ptr = input.as_ptr().add(2);

    for idx in 0..NUM_ROBOTS {
        robots.x[idx].write(parse_pcoord(&mut ptr));
        robots.y[idx].write(parse_pcoord(&mut ptr));
        ptr = ptr.add(2);

        robots.vx[idx].write(parse_vcoord::<b','>(&mut ptr));
        robots.vy[idx].write(parse_vcoord::<b'\n'>(&mut ptr));
        ptr = ptr.add(2);
    }
}

fn lut_lookup(vpat: i32, hpat: i32) -> u32 {
    const LUT_RAW: &[u8] = include_bytes!("../lut/day14.bin");

    let offset = (vpat * 103 + hpat) as usize * 4;
    u32::from_ne_bytes(unsafe {
        LUT_RAW
            .get_unchecked(offset..offset + 4)
            .try_into()
            .unwrap_unchecked()
    })
}

#[inline]
unsafe fn any_ge<const N: usize>(data: &[u32; N], target: u32) -> bool {
    let cmp = u32x8::splat(target);

    for i in 0..N / 8 {
        let vec = u32x8::from_array(
            *data
                .get_unchecked(i * 8..i * 8 + 8)
                .as_array()
                .unwrap_unchecked(),
        );
        if vec.simd_ge(cmp).any() {
            return true;
        }
    }
    return false;
}

#[inline]
unsafe fn search_cols(robots: &mut Robots) -> i32 {
    let mut cols = [0; WIDTH as usize];

    for step in 0..103 {
        cols.iter_mut().for_each(|x| *x = 0);

        for i in 0..NUM_ROBOTS_PAD8 / 8 {
            let xs = i32x8::from_array(*robots.x[i * 8..i * 8 + 8].as_array().unwrap_unchecked());
            let vxs = i32x8::from_array(*robots.vx[i * 8..i * 8 + 8].as_array().unwrap_unchecked());
            xs.add(vxs).copy_to_slice(&mut robots.x[i * 8..i * 8 + 8]);
        }

        for i in 0..NUM_ROBOTS {
            robots.x[i] = robots.x[i].rem_euclid(WIDTH);
        }

        for i in 0..NUM_ROBOTS {
            *cols.get_unchecked_mut(robots.x[i] as usize) += 1;
        }

        if any_ge(
            cols.get_unchecked(..80).as_array::<80>().unwrap_unchecked(),
            33,
        ) {
            return step as i32;
        }
    }

    unreachable_unchecked()
}

#[inline]
unsafe fn search_rows(robots: &mut Robots) -> i32 {
    let mut rows = [0; HEIGHT as usize];

    for step in 0..103 {
        rows.iter_mut().for_each(|x| *x = 0);

        for i in 0..NUM_ROBOTS_PAD8 / 8 {
            let ys = i32x8::from_array(*robots.y[i * 8..i * 8 + 8].as_array().unwrap_unchecked());
            let vys = i32x8::from_array(*robots.vy[i * 8..i * 8 + 8].as_array().unwrap_unchecked());
            ys.add(vys).copy_to_slice(&mut robots.y[i * 8..i * 8 + 8]);
        }

        for i in 0..NUM_ROBOTS {
            robots.y[i] = robots.y[i].rem_euclid(HEIGHT);
        }

        for i in 0..NUM_ROBOTS {
            *rows.get_unchecked_mut(robots.y[i] as usize) += 1;
        }

        if any_ge(
            rows.get_unchecked(..80).as_array::<80>().unwrap_unchecked(),
            31,
        ) {
            return step as i32;
        }
    }

    unreachable_unchecked()
}

#[inline(always)]
unsafe fn inner_p2(input: &[u8]) -> u32 {
    let mut robots = RobotsUninit::default();
    parse(input, &mut robots);
    let mut robots: Robots = std::mem::transmute(robots);

    let hpat = search_rows(&mut robots);
    let vpat = search_cols(&mut robots);

    lut_lookup(vpat, hpat)
}

pub fn part2(input: &str) -> u32 {
    unsafe { inner_p2(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input14.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 228457125);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 6493);
    }
}
