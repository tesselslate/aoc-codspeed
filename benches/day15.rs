use aoc_codspeed::day15;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D15_INPUT: &str = include_str!("../inputs/input15.txt");

pub fn d15p1(c: &mut Criterion) {
    c.bench_function("15a", |b| b.iter(|| black_box(day15::part1(D15_INPUT))));
}

pub fn d15p2(c: &mut Criterion) {
    c.bench_function("15b", |b| b.iter(|| black_box(day15::part2(D15_INPUT))));
}

criterion_group!(d15, d15p1, d15p2);
criterion_main!(d15);
