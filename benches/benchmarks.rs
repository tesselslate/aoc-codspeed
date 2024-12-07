use aoc_codspeed::day6;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

    const D6_INPUT: &str = include_str!("../inputs/input6.txt");

pub fn d6p1(c: &mut Criterion) {
    c.bench_function("6a", |b| b.iter(|| black_box(day6::part1(D6_INPUT))));
}

pub fn d6p2(c: &mut Criterion) {
    c.bench_function("6b", |b| b.iter(|| black_box(day6::part2(D6_INPUT))));
}

criterion_group!(benches, d6p1, d6p2);
criterion_main!(benches);
