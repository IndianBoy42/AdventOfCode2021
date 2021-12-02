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
    c.bench_function("Day 1 Part 1", |b| {
        b.iter(|| day1::part1(black_box(&input1)))
    });
    c.bench_function("Day 1 Part 2", |b| {
        b.iter(|| day1::part2(black_box(&input1)))
    });

    let input1 = utils::read_input("input2.txt").unwrap();
    c.bench_function("Day 2 Part 1", |b| {
        b.iter(|| day2::part1(black_box(&input1)))
    });
    c.bench_function("Day 2 Part 2", |b| {
        b.iter(|| day2::part2(black_box(&input1)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
