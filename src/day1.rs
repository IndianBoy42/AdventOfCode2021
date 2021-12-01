use std::mem::size_of;

use crate::utils::*;

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec()
        .array_windows()
        .filter(|[a, b]| b > a)
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect_vec()
        .array_windows()
        .map(|[a, b, c]| a + b + c)
        .collect_vec()
        .array_windows()
        .filter(|[a, b]| b > a)
        .count()
}

#[test]
fn test() {
    let input = read_input("input1.txt").unwrap();
    assert_eq!(part1(&input), 1581);
    assert_eq!(part2(&input), 1618);
    // assert_eq!(part2fft(&input), 246191688);
}
