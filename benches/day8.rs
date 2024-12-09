use aoc_codspeed::day8::{self, Bitmap, Points};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D8_INPUT: &str = include_str!("../inputs/input8.txt");

pub fn d8_bitmap_sum(c: &mut Criterion) {
    let bitmap = Bitmap::default();
    c.bench_function("8_bitmap_sum", |b| b.iter(|| black_box(bitmap).sum()));
}

pub fn d8_parse(c: &mut Criterion) {
    c.bench_function("8_parse", |b| {
        b.iter(|| {
            let mut points = Points::default();
            black_box(day8::parse(D8_INPUT, &mut points));
        })
    });
}

pub fn d8p1_inner(c: &mut Criterion) {
    let mut points = Points::default();
    let mut bitmap = Bitmap::default();
    day8::parse(D8_INPUT, &mut points);
    c.bench_function("8a_inner", |b| {
        b.iter(|| black_box(day8::antinodes_1(&points, &mut bitmap)))
    });
}

pub fn d8p2_inner(c: &mut Criterion) {
    let mut points = Points::default();
    let mut bitmap = Bitmap::default();
    day8::parse(D8_INPUT, &mut points);
    c.bench_function("8b_inner", |b| {
        b.iter(|| black_box(day8::antinodes_2(&points, &mut bitmap)))
    });
}

criterion_group!(d8_etc, d8_bitmap_sum, d8_parse, d8p1_inner, d8p2_inner);
criterion_main!(d8_etc);
