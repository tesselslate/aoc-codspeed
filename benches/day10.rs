use aoc_codspeed::day10;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D10_INPUT: &str = include_str!("../inputs/input10.txt");

pub fn d10p1(c: &mut Criterion) {
    c.bench_function("10a", |b| b.iter(|| black_box(day10::part1(D10_INPUT))));
}

pub fn d10p2(c: &mut Criterion) {
    c.bench_function("10b", |b| b.iter(|| black_box(day10::part2(D10_INPUT))));
}

criterion_group!(d10, d10p1, d10p2);
criterion_main!(d10);
