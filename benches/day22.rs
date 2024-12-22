use aoc_codspeed::day22;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D22_INPUT: &str = include_str!("../inputs/input22.txt");

pub fn d22p1(c: &mut Criterion) {
    c.bench_function("22a", |b| {
        b.iter(|| black_box(day22::part1(black_box(D22_INPUT))))
    });
}

pub fn d22p2(c: &mut Criterion) {
    c.bench_function("22b", |b| {
        b.iter(|| black_box(day22::part2(black_box(D22_INPUT))))
    });
}

criterion_group!(d22, d22p1, d22p2);
criterion_main!(d22);
