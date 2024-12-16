use aoc_codspeed::day16;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D16_INPUT: &str = include_str!("../inputs/input16.txt");

pub fn d16p1(c: &mut Criterion) {
    c.bench_function("16a", |b| b.iter(|| black_box(day16::part1(D16_INPUT))));
}

pub fn d16p2(c: &mut Criterion) {
    c.bench_function("16b", |b| b.iter(|| black_box(day16::part2(D16_INPUT))));
}

criterion_group!(d16, d16p1, d16p2);
criterion_main!(d16);
