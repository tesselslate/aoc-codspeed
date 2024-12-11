use aoc_codspeed::day10;
use aoc_codspeed::day11;
use aoc_codspeed::day6;
use aoc_codspeed::day7;
use aoc_codspeed::day8;
use aoc_codspeed::day9;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D6_INPUT: &str = include_str!("../inputs/input6.txt");
const D7_INPUT: &str = include_str!("../inputs/input7.txt");
const D8_INPUT: &str = include_str!("../inputs/input8.txt");
const D9_INPUT: &str = include_str!("../inputs/input9.txt");
const D10_INPUT: &str = include_str!("../inputs/input10.txt");
const D11_INPUT: &str = include_str!("../inputs/input11.txt");

pub fn d6p1(c: &mut Criterion) {
    c.bench_function("6a", |b| b.iter(|| black_box(day6::part1(D6_INPUT))));
}

pub fn d6p2(c: &mut Criterion) {
    c.bench_function("6b", |b| b.iter(|| black_box(day6::part2(D6_INPUT))));
}

pub fn d7p1(c: &mut Criterion) {
    c.bench_function("7a", |b| b.iter(|| black_box(day7::part1(D7_INPUT))));
}

pub fn d7p2(c: &mut Criterion) {
    c.bench_function("7b", |b| b.iter(|| black_box(day7::part2(D7_INPUT))));
}

pub fn d8p1(c: &mut Criterion) {
    c.bench_function("8a", |b| b.iter(|| black_box(day8::part1(D8_INPUT))));
}

pub fn d8p2(c: &mut Criterion) {
    c.bench_function("8b", |b| b.iter(|| black_box(day8::part2(D8_INPUT))));
}

pub fn d9p1(c: &mut Criterion) {
    c.bench_function("9a", |b| b.iter(|| black_box(day9::part1(D9_INPUT))));
}

pub fn d9p2(c: &mut Criterion) {
    c.bench_function("9b", |b| b.iter(|| black_box(day9::part2(D9_INPUT))));
}

pub fn d10p1(c: &mut Criterion) {
    c.bench_function("10a", |b| b.iter(|| black_box(day10::part1(D10_INPUT))));
}

pub fn d10p2(c: &mut Criterion) {
    c.bench_function("10b", |b| b.iter(|| black_box(day10::part2(D10_INPUT))));
}

pub fn d11p1(c: &mut Criterion) {
    c.bench_function("11a", |b| b.iter(|| black_box(day11::part1(D11_INPUT))));
}

pub fn d11p2(c: &mut Criterion) {
    c.bench_function("11b", |b| b.iter(|| black_box(day11::part2(D11_INPUT))));
}

criterion_group!(d6, d6p1, d6p2);
criterion_group!(d7, d7p1, d7p2);
criterion_group!(d8, d8p1, d8p2);
criterion_group!(d9, d9p1, d9p2);
criterion_group!(d10, d10p1, d10p2);
criterion_group!(d11, d11p1, d11p2);
criterion_main!(d11);
