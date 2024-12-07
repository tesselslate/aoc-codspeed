#![feature(core_intrinsics)]

use std::hint::unreachable_unchecked;
use std::intrinsics::assume;
use std::ptr;

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

pub fn unconcat_me(c: &mut Criterion) {
    c.bench_function("unconcat_me", |b| {
        b.iter(|| {
            for i in 1..1000 {
                black_box(me_unconcat(1234, i));
            }
        })
    });
}

pub fn unconcat_ja(c: &mut Criterion) {
    c.bench_function("unconcat_ja", |b| {
        b.iter(|| {
            for i in 1..1000 {
                black_box(ja_unconcat(1234, i));
            }
        })
    });
}

pub fn ja_unconcat(have: u64, concat: u64) -> Option<u64> {
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

fn me_unconcat(have: u64, concat: u64) -> Option<u64> {
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

criterion_group!(benches, d7p1, d7p2);
criterion_group!(d7_etc, unconcat_me, unconcat_ja);
criterion_main!(benches);
