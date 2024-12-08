#![feature(generic_arg_infer)]
#![feature(iter_array_chunks)]
#![feature(let_chains)]
#![feature(portable_simd)]
#![feature(ptr_sub_ptr)]

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day6;
pub mod day7;

aoc_lib! { year = 2024 }
