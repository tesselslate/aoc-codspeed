use aoc_codspeed::day9;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D9_INPUT: &str = include_str!("../inputs/input9.txt");

pub fn d9p1(c: &mut Criterion) {
    c.bench_function("9a", |b| b.iter(|| black_box(day9::part1(D9_INPUT))));
}

pub fn d9p2(c: &mut Criterion) {
    c.bench_function("9b", |b| b.iter(|| black_box(day9::part2(D9_INPUT))));
}

criterion_group!(d9, d9p1, d9p2);
criterion_main!(d9);
