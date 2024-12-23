use aoc_codspeed::day23;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D23_INPUT: &str = include_str!("../inputs/input23.txt");

pub fn d23p1(c: &mut Criterion) {
    c.bench_function("23a", |b| {
        b.iter(|| black_box(day23::part1(black_box(D23_INPUT))))
    });
}

pub fn d23p2(c: &mut Criterion) {
    c.bench_function("23b", |b| {
        b.iter(|| black_box(day23::part2(black_box(D23_INPUT))))
    });
}

criterion_group!(d23, d23p1, d23p2);
criterion_main!(d23);
