use std::time::Instant;

use aoc_codspeed::day15;

const INPUT: &str = include_str!("../../inputs/input15.txt");

fn main() {
    const N: usize = 1000;
    let start = Instant::now();
    for _ in 0..N {
        std::hint::black_box(day15::part1(INPUT));
    }
    println!(
        "p1: {:.3} usec",
        Instant::now().duration_since(start).as_nanos() as f64 / 1000.0 / N as f64
    );

    let start = Instant::now();
    for _ in 0..N {
        std::hint::black_box(day15::part2(INPUT));
    }
    println!(
        "p2: {:.3} usec",
        Instant::now().duration_since(start).as_nanos() as f64 / 1000.0 / N as f64
    );
}
