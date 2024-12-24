#![allow(static_mut_refs)]

use fastdiv::FastDiv;

#[inline(always)]
fn id(a: u8, b: u8) -> usize {
    (a - b'a') as usize * 26 + (b - b'a') as usize
}

unsafe fn inner_p1(input: &[u8]) -> u64 {
    const T_START: usize = (b't' - b'a') as usize * 26;

    static mut COMPUTERS: [[u16; 16]; 26 * 26] = [[0; 16]; 26 * 26];
    COMPUTERS.iter_mut().for_each(|x| *x = [0; 16]);

    let d26 = (26u32).precompute_div();

    let comp_ptr = COMPUTERS.as_mut_ptr();

    let mut input = input.as_ptr();

    for i in 0..3380 {
        let a = id(*input, *input.add(1));
        let b = id(*input.add(3), *input.add(4));

        if i != 3379 {
            // i love undefined behavior!
            input = input.add(6);
        }

        let c_a = comp_ptr.add(a);
        let c_b = comp_ptr.add(b);

        (*c_a)[(*c_a)[15] as usize] = b as u16;
        (*c_b)[(*c_b)[15] as usize] = a as u16;
        (*c_a)[15] += 1;
        (*c_b)[15] += 1;
    }

    let mut groups = 0;
    let mut dupes = 0;

    for i in T_START..T_START + 26 {
        for j in 1..13 {
            for k in 0..j {
                let a = COMPUTERS[i][j] as usize;
                let b = COMPUTERS[i][k] as u16;

                if COMPUTERS[a][..13].contains(&b) {
                    groups += 1;

                    if (a as u32).fast_div(d26) == (b't' - b'a') as u32
                        || (b as u32).fast_div(d26) == (b't' - b'a') as u32
                    {
                        dupes += 1;
                    }
                }
            }
        }
    }

    groups - (dupes >> 1)
}

unsafe fn inner_p2(input: &[u8]) -> &'static str {
    static mut OUTBUF: [u8; 64] = [0; 64];

    std::str::from_raw_parts(OUTBUF.as_ptr(), 0)
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_p1(input.as_bytes()) }
}

pub fn part2(input: &str) -> &'static str {
    unsafe { inner_p2(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input23.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 1083);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), "as,bu,cp,dj,ez,fd,hu,it,kj,nx,pp,xh,yu");
    }
}
