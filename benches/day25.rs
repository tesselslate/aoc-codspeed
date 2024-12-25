use aoc_codspeed::day25;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D25_INPUT: &str = include_str!("../inputs/input25.txt");

pub fn d25p1(c: &mut Criterion) {
    c.bench_function("25a", |b| {
        b.iter(|| black_box(day25::part1(black_box(D25_INPUT))))
    });
}

criterion_group!(d25, d25p1);
criterion_main!(d25);
