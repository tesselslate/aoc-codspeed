use std::time::Instant;

use aoc_codspeed::day22;

const INPUT: &str = include_str!("../../inputs/input22.txt");

fn main() {
    const N: usize = 1;
    let start = Instant::now();
    for _ in 0..N {
        std::hint::black_box(day22::part1(std::hint::black_box(INPUT)));
    }
    println!(
        "p1: {:.3} usec",
        Instant::now().duration_since(start).as_nanos() as f64 / 1000.0 / N as f64
    );

    let start = Instant::now();
    for _ in 0..N {
        std::hint::black_box(day22::part2(std::hint::black_box(INPUT)));
    }
    println!(
        "p2: {:.3} usec",
        Instant::now().duration_since(start).as_nanos() as f64 / 1000.0 / N as f64
    );
}
