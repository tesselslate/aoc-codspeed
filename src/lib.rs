#![feature(array_chunks)]
#![feature(clone_to_uninit)]
#![feature(fn_align)]
#![feature(iter_array_chunks)]
#![feature(portable_simd)]
#![feature(ptr_sub_ptr)]
#![feature(slice_as_array)]
#![feature(str_from_raw_parts)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day21;
pub mod day22;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

aoc_lib! { year = 2024 }
