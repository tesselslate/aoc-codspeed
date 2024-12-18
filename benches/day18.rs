use aoc_codspeed::day18;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D18_INPUT: &str = include_str!("../inputs/input18.txt");

pub fn d18p1(c: &mut Criterion) {
    c.bench_function("18a", |b| {
        b.iter(|| black_box(day18::part1(black_box(D18_INPUT))))
    });
}

pub fn d18p2(c: &mut Criterion) {
    c.bench_function("18b", |b| {
        b.iter(|| black_box(day18::part2(black_box(D18_INPUT))))
    });
}

criterion_group!(d18, d18p1, d18p2);
criterion_main!(d18);
