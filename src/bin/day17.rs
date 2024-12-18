use std::time::Instant;

use aoc_codspeed::day17;

const INPUT: &str = include_str!("../../inputs/input17.txt");

fn main() {
    const N: usize = 1000000000;
    let start = Instant::now();
    for _ in 0..N {
        std::hint::black_box(day17::part1(std::hint::black_box(INPUT)));
    }
    println!(
        "p1: {:.3} usec",
        Instant::now().duration_since(start).as_nanos() as f64 / 1000.0 / N as f64
    );
    //
    // let start = Instant::now();
    // for _ in 0..N {
    //     std::hint::black_box(day17::part2(INPUT));
    // }
    // println!(
    //     "p2: {:.3} usec",
    //     Instant::now().duration_since(start).as_nanos() as f64 / 1000.0 / N as f64
    // );
}
