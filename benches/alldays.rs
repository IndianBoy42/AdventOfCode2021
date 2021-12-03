#![feature(concat_idents)]
use aoc21::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench_day {
    ($n:ident) => {
        let concat_idents!(input, $n) = utils::read_input("input$n.txt").unwrap();
        c.bench_function("day$n_p1", |b| b.iter(|| concat_idents!(day, $n)::part1(black_box(&input1))));
        c.bench_function("day$n_p2", |b| b.iter(|| concat_idents!(day, $n)::part2(black_box(&input1))));
    };
}

pub fn criterion_benchmark(crit: &mut Criterion) {
    let mut c = crit.benchmark_group("Main Benchmarks");
    c.sample_size(100);

    let input1 = utils::read_input("input1.txt").unwrap();
    c.bench_function("d1p1", |b| b.iter(|| day1::part2(black_box(&input1))));
    c.bench_function("d1p2", |b| b.iter(|| day1::part2(black_box(&input1))));

    let input2 = utils::read_input("input2.txt").unwrap();
    c.bench_function("d2p1", |b| b.iter(|| day2::part1(black_box(&input2))));
    c.bench_function("d2p2", |b| b.iter(|| day2::part2(black_box(&input2))));

    let input3 = utils::read_input("input3.txt").unwrap();
    c.bench_function("d3p1", |b| b.iter(|| day3::part1(black_box(&input3))));
    c.bench_function("d3p2", |b| b.iter(|| day3::part2(black_box(&input3))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
