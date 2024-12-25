use aoc_codspeed::day24;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D24_INPUT: &str = include_str!("../inputs/input24.txt");

pub fn d24p1(c: &mut Criterion) {
    c.bench_function("24a", |b| {
        b.iter(|| black_box(day24::part1(black_box(D24_INPUT))))
    });
}

pub fn d24p2(c: &mut Criterion) {
    c.bench_function("24b", |b| {
        b.iter(|| black_box(day24::part2(black_box(D24_INPUT))))
    });
}

criterion_group!(d24, d24p1, d24p2);
criterion_main!(d24);
