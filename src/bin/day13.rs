use std::time::Instant;

use aoc_codspeed::day13;

const INPUT: &str = include_str!("../../inputs/input13.txt");

fn main() {
    let start = Instant::now();
    for _ in 0..1000 {
        std::hint::black_box(day13::part1(INPUT));
    }
    println!(
        "p1: {:.3} usec",
        Instant::now().duration_since(start).as_nanos() as f64 / 1000.0 / 1000.0
    );

    let start = Instant::now();
    for _ in 0..1000 {
        std::hint::black_box(day13::part2(INPUT));
    }
    println!(
        "p2: {:.3} usec",
        Instant::now().duration_since(start).as_nanos() as f64 / 1000.0 / 1000.0
    );
}
