struct Scenario([i64; 6]);

impl Default for Scenario {
    fn default() -> Self {
        Self([0; 6])
    }
}

#[inline]
fn solve(scenario: &Scenario) -> Option<i64> {
    let [ax, ay, bx, by, px, py] = scenario.0;

    let div = ax * by - ay * bx;
    unsafe { std::hint::assert_unchecked(div != 0) };

    let a = (px * by - py * bx) / div;
    let b = (py * ax - px * ay) / div;

    if a * ax + b * bx == px && a * ay + b * by == py {
        Some(3 * a + b)
    } else {
        None
    }
}

#[inline(always)]
unsafe fn read_i64_2(ptr: *const u8, acc: &mut i64) {
    let a = (*ptr - b'0') as i64;
    let b = (*ptr.add(1) - b'0') as i64;
    *acc = a * 10 + b;
}

#[inline]
unsafe fn read_i64_3p<const DELIM: u8>(ptr: *const u8, acc: &mut i64) -> *const u8 {
    let a = (*ptr - b'0') as i64;
    let b = (*ptr.add(1) - b'0') as i64;
    let c = (*ptr.add(2) - b'0') as i64;

    if *ptr.add(3) == DELIM {
        *acc = a * 100 + b * 10 + c;
        ptr.add(3)
    } else if *ptr.add(4) == DELIM {
        let d = (*ptr.add(3) - b'0') as i64;

        *acc = a * 1000 + b * 100 + c * 10 + d;
        ptr.add(4)
    } else {
        let d = (*ptr.add(3) - b'0') as i64;
        let e = (*ptr.add(4) - b'0') as i64;

        *acc = a * 10000 + b * 1000 + c * 100 + d * 10 + e;
        ptr.add(5)
    }
}

unsafe fn inner<const N: usize, const D: i64>(input: &[u8]) -> i64 {
    const SKIP_BUTTON_AX: usize = "Button A: X+".len();
    const SKIP_BUTTON_AY: usize = "Button A: X+XX, Y+".len();
    const SKIP_BUTTON_BX: usize = "Button A: X+XX, Y+YY\nButton B: X+".len();
    const SKIP_BUTTON_BY: usize = "Button A: X+XX, Y+YY\nButton B: X+XX, Y+".len();
    const SKIP_PRIZE_X: usize = "Button A: X+XX, Y+YY\nButton B: X+XX, Y+YY\nPrize: X=".len();

    let mut sum = 0;
    let mut scenario = Scenario::default();

    let mut ptr = input.as_ptr();
    for _ in 0..N {
        read_i64_2(ptr.add(SKIP_BUTTON_AX), &mut scenario.0[0]);
        read_i64_2(ptr.add(SKIP_BUTTON_AY), &mut scenario.0[1]);

        read_i64_2(ptr.add(SKIP_BUTTON_BX), &mut scenario.0[2]);
        read_i64_2(ptr.add(SKIP_BUTTON_BY), &mut scenario.0[3]);

        ptr = read_i64_3p::<b','>(ptr.add(SKIP_PRIZE_X), &mut scenario.0[4]);
        ptr = read_i64_3p::<b'\n'>(ptr.add(4), &mut scenario.0[5]).add(2);

        scenario.0[4] += D;
        scenario.0[5] += D;
        sum += solve(&scenario).unwrap_or(0);
    }

    sum
}

pub fn part1(input: &str) -> i64 {
    unsafe { inner::<320, 0>(input.as_bytes()) }
}

pub fn part2(input: &str) -> i64 {
    unsafe { inner::<320, 10000000000000>(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input13.txt");
    const TEST: &str = include_str!("../testdata/input13.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 29201);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 104140871044942);
    }

    #[test]
    fn test_a() {
        assert_eq!(unsafe { inner::<4, 0>(TEST.as_bytes()) }, 480);
    }

    #[test]
    fn test_b() {
        assert_eq!(
            unsafe { inner::<4, 10000000000000>(TEST.as_bytes()) },
            875318608908
        );
    }
}
