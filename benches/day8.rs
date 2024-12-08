use aoc_codspeed::day8::Bitmap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn d8_bitmap_sum(c: &mut Criterion) {
    let bitmap = Bitmap::default();
    c.bench_function("8_bitmap_sum", |b| b.iter(|| black_box(bitmap).sum()));
}

criterion_group!(d8_etc, d8_bitmap_sum);
criterion_main!(d8_etc);
