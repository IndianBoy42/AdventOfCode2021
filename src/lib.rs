#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(incomplete_features)]
#![allow(dead_code)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::inline_always)]
#![allow(clippy::must_use_candidate)]
#![feature(try_blocks)]
#![feature(associated_type_bounds)]
// #![feature(const_generic_impls_guard)]
#![feature(iter_partition_in_place)]
#![feature(map_entry_replace)]
#![feature(specialization)]
#![feature(maybe_uninit_extra)]
#![feature(maybe_uninit_uninit_array)]
#![feature(array_chunks)]
#![feature(array_windows)]
#![feature(test)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub mod day1;

pub mod grid;
pub mod searcher;
pub mod u32set;
pub mod utils;
