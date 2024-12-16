#![feature(array_chunks)]
#![feature(iter_array_chunks)]
#![feature(portable_simd)]
#![feature(ptr_sub_ptr)]
#![feature(slice_as_array)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

aoc_lib! { year = 2024 }
