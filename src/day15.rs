const DIR_LINES: usize = 20;
const DIR_LENGTH: usize = 1000;

unsafe fn inner_p1(input: &str) -> u32 {
    const OFFSETS: [isize; 256] = {
        let mut offsets = [0; 256];

        offsets[b'<' as usize] = -1;
        offsets[b'>' as usize] = 1;
        offsets[b'^' as usize] = -51;
        offsets[b'v' as usize] = 51;

        offsets
    };

    let mut grid = [0; 2550];
    grid.copy_from_slice(input.as_bytes().get_unchecked(..2550));
    let mut dir = input.as_bytes().as_ptr().add(2551);

    let mut robot = grid.as_mut_ptr().add(24 * 51 + 24);
    *robot = b'.';

    for _ in 0..DIR_LINES {
        for _ in 0..DIR_LENGTH {
            let offset = OFFSETS[*dir as usize];
            let pos = robot.offset(offset);

            match *pos {
                b'.' => robot = pos,
                b'#' => (),
                b'O' => {
                    let mut box_pos = pos;

                    loop {
                        if *box_pos == b'.' {
                            *pos = b'.';
                            *box_pos = b'O';
                            robot = pos;
                            break;
                        } else if *box_pos == b'#' {
                            break;
                        } else {
                            std::hint::assert_unchecked(*box_pos == b'O');
                        }

                        box_pos = box_pos.offset(offset);
                    }
                }
                _ => std::hint::unreachable_unchecked(),
            }
            dir = dir.add(1);
        }
        dir = dir.add(1);
    }

    let mut sum = 0;
    for i in 0..2550 {
        if *grid.get_unchecked(i) == b'O' {
            sum += (i / 51) * 100 + (i % 51)
        }
    }
    sum as u32
}

pub fn part1(input: &str) -> u32 {
    unsafe { inner_p1(input) }
}

pub fn part2(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input15.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 1568399);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 1575877);
    }
}
