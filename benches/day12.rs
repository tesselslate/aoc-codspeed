use aoc_codspeed::day12;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D12_INPUT: &str = include_str!("../inputs/input12.txt");

pub fn d12p1(c: &mut Criterion) {
    c.bench_function("12a", |b| b.iter(|| black_box(day12::part1(D12_INPUT))));
}

pub fn d12p2(c: &mut Criterion) {
    c.bench_function("12b", |b| b.iter(|| black_box(day12::part2(D12_INPUT))));
}

criterion_group!(d12, d12p1, d12p2);
criterion_main!(d12);
