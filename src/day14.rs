use std::mem::MaybeUninit;

const NUM_ROBOTS: usize = 500;
const STEPS_P1: i32 = 100;
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[repr(C)]
struct Robots {
    pos: [(i32, i32); NUM_ROBOTS],
    vel: [(i32, i32); NUM_ROBOTS],
}

#[repr(C)]
struct RobotsUninit {
    pos: [(MaybeUninit<i32>, MaybeUninit<i32>); NUM_ROBOTS],
    vel: [(MaybeUninit<i32>, MaybeUninit<i32>); NUM_ROBOTS],
}

impl Default for RobotsUninit {
    fn default() -> Self {
        Self {
            pos: [(MaybeUninit::uninit(), MaybeUninit::uninit()); NUM_ROBOTS],
            vel: [(MaybeUninit::uninit(), MaybeUninit::uninit()); NUM_ROBOTS],
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

// p=PX,PY v=VX,VY
// ...
//
// PX and PY are 1-3 digits
// VX and VY are 1-2(?) digit pos/neg numbers
fn parse(input: &[u8], robots: &mut RobotsUninit) {
    let mut ptr = unsafe { input.as_ptr().add(2) };

    for idx in 0..NUM_ROBOTS {
        robots.pos[idx]
            .0
            .write(unsafe { parse_pcoord::<b','>(&mut ptr) });
        robots.pos[idx]
            .1
            .write(unsafe { parse_pcoord::<b' '>(&mut ptr) });
        ptr = unsafe { ptr.add(2) };

        robots.vel[idx]
            .0
            .write(unsafe { parse_vcoord::<b','>(&mut ptr) });
        robots.vel[idx]
            .1
            .write(unsafe { parse_vcoord::<b'\n'>(&mut ptr) });
        ptr = unsafe { ptr.add(2) };
    }
}

fn inner_p1(input: &[u8]) -> u64 {
    let mut robots = RobotsUninit::default();
    parse(input, &mut robots);

    let robots: Robots = unsafe { std::mem::transmute(robots) };

    let mut quads = [0u64; 4];
    for i in 0..NUM_ROBOTS {
        let mut pos = robots.pos[i];
        let vel = robots.vel[i];

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
    inner_p1(input.as_bytes())
}

pub fn part2(input: &str) -> u32 {
    0
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
