#![feature(core_intrinsics)]

use std::hint::unreachable_unchecked;
use std::intrinsics::assume;
use std::ptr;

use aoc_codspeed::day6;
use aoc_codspeed::day7;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const D6_INPUT: &str = include_str!("../inputs/input6.txt");
const D7_INPUT: &str = include_str!("../inputs/input7.txt");

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

pub fn bench_d7parse(c: &mut Criterion) {
    c.bench_function("7parse", |b| b.iter(|| black_box(unsafe { d7_parse() })));
}

pub fn unconcat_me(c: &mut Criterion) {
    c.bench_function("unconcat_me", |b| {
        b.iter(|| {
            for i in 1..1000 {
                black_box(unsafe { me_unconcat(987, i) });
            }
        })
    });
}

pub fn unconcat2_me(c: &mut Criterion) {
    c.bench_function("unconcat2_me", |b| {
        b.iter(|| {
            for i in 1..1000 {
                black_box(unsafe { me_unconcat2(987, i) });
            }
        })
    });
}

pub fn unconcat_ja(c: &mut Criterion) {
    c.bench_function("unconcat_ja", |b| {
        b.iter(|| {
            for i in 1..1000 {
                black_box(unsafe { ja_unconcat(987, i) });
            }
        })
    });
}

#[target_feature(
    enable = "avx,avx2,bmi1,bmi2,cmpxchg16b,fma,fxsr,lzcnt,movbe,pclmulqdq,popcnt,sse,sse2,sse3,sse4.1,sse4.2,ssse3"
)]
unsafe fn ja_unconcat(have: u64, concat: u64) -> Option<u64> {
    const LOG2_POW10: [u8; 16] = [1, 1, 1, 2, 2, 2, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5];
    const POW10: [u64; 6] = [0, 0, 10, 100, 1000, 10000];

    unsafe {
        let idx = *LOG2_POW10.get_unchecked(64u32.unchecked_sub(concat.leading_zeros()) as usize)
            as usize;

        let pow10: *const u64 = ptr::from_ref(POW10.get_unchecked(idx));
        let less: bool = concat >= *pow10;
        let pow10 = *pow10.add((less as usize) * size_of::<u64>());

        assume(pow10 != 0);

        if have % pow10 == concat {
            Some(have / pow10)
        } else {
            None
        }
    }
}

#[target_feature(
    enable = "avx,avx2,bmi1,bmi2,cmpxchg16b,fma,fxsr,lzcnt,movbe,pclmulqdq,popcnt,sse,sse2,sse3,sse4.1,sse4.2,ssse3"
)]
unsafe fn me_unconcat(have: u64, concat: u64) -> Option<u64> {
    let modulo = match concat {
        ..10 => 10,
        ..100 => 100,
        ..1000 => 1000,
        _ => unsafe { unreachable_unchecked() },
    };

    if have % modulo == concat {
        Some(have / modulo)
    } else {
        None
    }
}

#[target_feature(
    enable = "avx,avx2,bmi1,bmi2,cmpxchg16b,fma,fxsr,lzcnt,movbe,pclmulqdq,popcnt,sse,sse2,sse3,sse4.1,sse4.2,ssse3"
)]
unsafe fn me_unconcat2(have: u64, concat: u64) -> Option<u64> {
    const POW: [u16; 1000] = {
        let mut arr = [0; 1000];

        let mut i = 0;
        while i < arr.len() {
            arr[i] = match i {
                ..10 => 10,
                ..100 => 100,
                ..1000 => 1000,
                _ => unreachable!(),
            };
            i += 1;
        }

        arr
    };

    let modulo = unsafe { *POW.get_unchecked(concat as usize) as u64 };
    unsafe {
        assume(modulo != 0);
    }

    if have % modulo == concat {
        Some(have / modulo)
    } else {
        None
    }
}

#[target_feature(enable = "sse2,sse3,sse4.1,ssse3,avx,avx2")]
unsafe fn d7_parse() -> u64 {
    unsafe fn parse_u64(b: &[u8]) -> u64 {
        b.iter().fold(0, |acc, &b| acc * 10 + (b & 0xF) as u64)
    }

    unsafe fn get_nums<'a>(l: &[u8], storage: &'a mut [u64; 12]) -> (u64, &'a [u64]) {
        let colon = memchr::memchr(b':', l).unwrap_unchecked();
        let target = parse_u64(l.get_unchecked(..colon));

        let mut i = colon + 2;
        let mut j = 0;
        loop {
            match memchr::memchr(b' ', l.get_unchecked(i..)) {
                Some(x) => {
                    *storage.get_unchecked_mut(j) = parse_u64(l.get_unchecked(i..i + x));
                    j += 1;
                    i += x + 1;
                }
                None => {
                    *storage.get_unchecked_mut(j) = parse_u64(l.get_unchecked(i..));
                    break;
                }
            }
        }

        (target, storage.get_unchecked(..j + 1))
    }

    unsafe fn add(l: &[u8], storage: &mut [u64; 12]) -> u64 {
        let (target, nums) = get_nums(l, storage);
        target + nums.iter().sum::<u64>()
    }

    let mut storage = [0u64; 12];

    let bytes = D7_INPUT.as_bytes();
    let mut sum = 0;

    let mut i = 0;
    unsafe {
        loop {
            match memchr::memchr(b'\n', bytes.get_unchecked(i..)) {
                Some(j) => {
                    sum += add(bytes.get_unchecked(i..i + j), &mut storage);
                    i += j + 1;
                }
                None => {
                    sum += add(bytes.get_unchecked(i..), &mut storage);
                    return sum;
                }
            }
        }
    }
}

criterion_group!(d6, d6p1, d6p2);
criterion_group!(d7, d7p1);
criterion_group!(d7_etc, unconcat_me, unconcat2_me, unconcat_ja);
criterion_group!(d7_parse_g, bench_d7parse);
criterion_main!(d7);
