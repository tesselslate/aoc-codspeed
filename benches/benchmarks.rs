use aoc_codspeed::day6;
use aoc_codspeed::day7;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D6_INPUT: &str = include_str!("../inputs/input6.txt");
const D7_INPUT: &str = include_str!("../inputs/input7.txt");

// pub fn d6p1(c: &mut Criterion) {
//     c.bench_function("6a", |b| b.iter(|| black_box(day6::part1(D6_INPUT))));
// }
//
// pub fn d6p2(c: &mut Criterion) {
//     c.bench_function("6b", |b| b.iter(|| black_box(day6::part2(D6_INPUT))));
// }

pub fn d7p1(c: &mut Criterion) {
    c.bench_function("7a", |b| b.iter(|| black_box(day7::part1(D7_INPUT))));
}

pub fn d7p2(c: &mut Criterion) {
    c.bench_function("7b", |b| b.iter(|| black_box(day7::part2(D7_INPUT))));
}

criterion_group!(benches, d7p1, d7p2);
criterion_main!(benches);
