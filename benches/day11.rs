use criterion::{black_box, criterion_group, criterion_main, Criterion};

// https://da-data.blogspot.com/2023/02/integer-log10-in-rust-and-c.html
// adapted for u64
fn fast_ilog10(x: u64) -> usize {
    #[rustfmt::skip]
    const LUT: [u64; 15] = [
        9,                  99,                 999,
        9_999,              99_999,             999_999,
        9_999_999,          99_999_999,         999_999_999,
        9_999_999_999,      99_999_999_999,     999_999_999_999,
        9_999_999_999_999,  99_999_999_999_999, 999_999_999_999_999,
    ];

    const MASK: u64 = 0b0001001001000100100100010010010001001001000100100100010010010000;

    let guess = (MASK << x.leading_zeros()).count_ones() as usize;
    guess + (x > LUT[guess]) as usize
}

pub fn log10_std(c: &mut Criterion) {
    c.bench_function("log10_std", |b| {
        b.iter(|| {
            let x = black_box(9999u64);
            unsafe { std::hint::assert_unchecked(x > 0) };

            black_box(x.ilog10())
        })
    });
}

pub fn log10_faster(c: &mut Criterion) {
    c.bench_function("log10_faster", |b| {
        b.iter(|| {
            let x = black_box(9999u64);
            unsafe { std::hint::assert_unchecked(x > 0) };

            black_box(fast_ilog10(x))
        })
    });
}

criterion_group!(d11_etc, log10_std, log10_faster);
criterion_main!(d11_etc);
