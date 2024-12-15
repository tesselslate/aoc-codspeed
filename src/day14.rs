use std::mem::MaybeUninit;

const NUM_ROBOTS: usize = 500;
const STEPS_P1: i32 = 100;
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[repr(C)]
struct Robots {
    x: [i32; NUM_ROBOTS],
    y: [i32; NUM_ROBOTS],
    vx: [i32; NUM_ROBOTS],
    vy: [i32; NUM_ROBOTS],
}

#[repr(C)]
struct RobotsUninit {
    x: [MaybeUninit<i32>; NUM_ROBOTS],
    y: [MaybeUninit<i32>; NUM_ROBOTS],
    vx: [MaybeUninit<i32>; NUM_ROBOTS],
    vy: [MaybeUninit<i32>; NUM_ROBOTS],
}

impl Default for RobotsUninit {
    fn default() -> Self {
        Self {
            x: [MaybeUninit::uninit(); NUM_ROBOTS],
            y: [MaybeUninit::uninit(); NUM_ROBOTS],
            vx: [MaybeUninit::uninit(); NUM_ROBOTS],
            vy: [MaybeUninit::uninit(); NUM_ROBOTS],
        }
    }
}

#[inline(always)]
unsafe fn parse_pcoord<const DELIM: u8>(input: &mut *const u8) -> i32 {
    let a = *input.add(0);
    let b = *input.add(1);

    if b == DELIM {
        *input = input.add(2);
        (a - b'0') as i32
    } else {
        let c = *input.add(2);

        if c == DELIM {
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
            // TODO: are there any 3 digit velocities (positive or negative)
            debug_assert!(neg);
            -((b - b'0') as i32 * 10 + (c - b'0') as i32)
        }
    }
}

#[inline(always)]
unsafe fn inner_p1(input: &[u8]) -> u64 {
    let mut ptr = input.as_ptr().add(2);
    let mut quads = [0u64; 4];
    for _ in 0..NUM_ROBOTS {
        let mut pos = (
            parse_pcoord::<b','>(&mut ptr),
            parse_pcoord::<b' '>(&mut ptr),
        );
        ptr = ptr.add(2);

        let vel = (
            parse_vcoord::<b','>(&mut ptr),
            parse_vcoord::<b'\n'>(&mut ptr),
        );
        ptr = ptr.add(2);

        pos.0 = (pos.0 + vel.0 * STEPS_P1).rem_euclid(WIDTH);
        pos.1 = (pos.1 + vel.1 * STEPS_P1).rem_euclid(HEIGHT);

        if pos.0 < WIDTH / 2 {
            if pos.1 < HEIGHT / 2 {
                quads[0] += 1;
            } else if pos.1 > HEIGHT / 2 {
                quads[1] += 1;
            }
        } else if pos.0 > WIDTH / 2 {
            if pos.1 < HEIGHT / 2 {
                quads[2] += 1;
            } else if pos.1 > HEIGHT / 2 {
                quads[3] += 1;
            }
        }
    }

    quads[0] * quads[1] * quads[2] * quads[3]
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input.as_bytes()) }
}

unsafe fn parse(input: &[u8], robots: &mut RobotsUninit) {
    let mut ptr = input.as_ptr().add(2);

    for idx in 0..NUM_ROBOTS {
        robots.x[idx].write(parse_pcoord::<b','>(&mut ptr));
        robots.y[idx].write(parse_pcoord::<b' '>(&mut ptr));
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
fn any_ge<const N: usize>(data: &[u32; N], target: u32) -> bool {
    data.iter().find(|&&x| x >= target).is_some()
}

unsafe fn search<const ROWS: bool, const COLS: bool>(
    start: usize,
    rows: &mut [u32; HEIGHT as usize],
    cols: &mut [u32; WIDTH as usize],
    vpat: &mut i32,
    hpat: &mut i32,
    robots: &mut Robots,
) {
    for step in start..103 {
        if ROWS {
            rows.iter_mut().for_each(|x| *x = 0);
        }
        if COLS {
            cols.iter_mut().for_each(|x| *x = 0);
        }

        for i in 0..NUM_ROBOTS {
            let pos = (robots.x[i], robots.y[i]);
            let vel = (robots.vx[i], robots.vy[i]);

            if ROWS {
                robots.y[i] = (pos.1 + vel.1).rem_euclid(HEIGHT);
                *rows.get_unchecked_mut(robots.y[i] as usize) += 1;
            }
            if COLS {
                robots.x[i] = (pos.0 + vel.0).rem_euclid(WIDTH);
                *cols.get_unchecked_mut(robots.x[i] as usize) += 1;
            }
        }

        if ROWS {
            if any_ge(rows, 31) {
                *hpat = step as i32;

                if COLS {
                    if any_ge(cols, 33) {
                        *vpat = step as i32;
                    } else {
                        search::<false, true>(step + 1, rows, cols, vpat, hpat, robots);
                    }
                }

                return;
            }
        }

        if COLS {
            if any_ge(cols, 33) {
                *vpat = step as i32;

                if ROWS {
                    search::<true, false>(step + 1, rows, cols, vpat, hpat, robots);
                }

                return;
            }
        }
    }
}

#[inline(always)]
unsafe fn inner_p2(input: &[u8]) -> u32 {
    let mut robots = RobotsUninit::default();
    parse(input, &mut robots);
    let mut robots: Robots = std::mem::transmute(robots);

    let (mut vpat, mut hpat) = (0, 0);
    search::<true, true>(
        0,
        &mut [0; HEIGHT as usize],
        &mut [0; WIDTH as usize],
        &mut vpat,
        &mut hpat,
        &mut robots,
    );

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
