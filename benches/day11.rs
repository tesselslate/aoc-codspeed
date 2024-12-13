use aoc_codspeed::day11;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D11_INPUT: &str = include_str!("../inputs/input11.txt");

pub fn d11p1(c: &mut Criterion) {
    c.bench_function("11a", |b| b.iter(|| black_box(day11::part1(D11_INPUT))));
}

pub fn d11p2(c: &mut Criterion) {
    c.bench_function("11b", |b| b.iter(|| black_box(day11::part2(D11_INPUT))));
}

criterion_group!(d11, d11p1, d11p2);
criterion_main!(d11);
