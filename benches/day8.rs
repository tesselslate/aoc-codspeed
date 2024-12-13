use aoc_codspeed::day8;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D8_INPUT: &str = include_str!("../inputs/input8.txt");

pub fn d8p1(c: &mut Criterion) {
    c.bench_function("8a", |b| b.iter(|| black_box(day8::part1(D8_INPUT))));
}

pub fn d8p2(c: &mut Criterion) {
    c.bench_function("8b", |b| b.iter(|| black_box(day8::part2(D8_INPUT))));
}

criterion_group!(d8, d8p1, d8p2);
criterion_main!(d8);
