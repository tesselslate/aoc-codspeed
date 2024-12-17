use aoc_codspeed::day17;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D17_INPUT: &str = include_str!("../inputs/input17.txt");

pub fn d17p1(c: &mut Criterion) {
    c.bench_function("17a", |b| b.iter(|| black_box(day17::part1(D17_INPUT))));
}

pub fn d17p2(c: &mut Criterion) {
    c.bench_function("17b", |b| b.iter(|| black_box(day17::part2(D17_INPUT))));
}

criterion_group!(d17, d17p1, d17p2);
criterion_main!(d17);
