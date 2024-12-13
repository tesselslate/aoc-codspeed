use aoc_codspeed::day13;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D13_INPUT: &str = include_str!("../inputs/input13.txt");

pub fn d13p1(c: &mut Criterion) {
    c.bench_function("13a", |b| b.iter(|| black_box(day13::part1(D13_INPUT))));
}

pub fn d13p2(c: &mut Criterion) {
    c.bench_function("13b", |b| b.iter(|| black_box(day13::part2(D13_INPUT))));
}

criterion_group!(d13, d13p1, d13p2);
criterion_main!(d13);
