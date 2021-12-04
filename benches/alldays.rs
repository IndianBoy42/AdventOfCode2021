#![feature(concat_idents)]
use aoc21::*;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench_day {
    ($c:ident, $n:ident, $i:tt) => {{
        let fname = concat!("input", $i, ".txt");
        let input = utils::read_input(fname).unwrap();
        $c.bench_function(concat!("day", $i, "p1"), |b| {
            b.iter(|| $n::part1(black_box(&input)))
        });
        $c.bench_function(concat!("day", $i, "p2"), |b| {
            b.iter(|| $n::part2(black_box(&input)))
        });
        input
    }};
}

macro_rules! bench_all_days {
    ($n:ident) => { {
        $n::part1(black_box(&$n));
        $n::part1(black_box(&$n));
    } };
    ($n:ident, $($ns:ident),+) => { {
        bench_all_days!($n);
        bench_all_days!($($ns),+);
    } };
}

pub fn criterion_benchmark(crit: &mut Criterion) {
    let mut c = crit.benchmark_group("Main Benchmarks");
    c.sample_size(100);

    let day1 = bench_day!(c, day1, 1);
    let day2 = bench_day!(c, day2, 2);
    let day3 = bench_day!(c, day3, 3);
    let day4 = bench_day!(c, day4, 4);
    c.bench_function("alldays", |b| {
        b.iter(|| bench_all_days!(day1, day2, day3, day4))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
