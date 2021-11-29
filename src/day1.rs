use std::mem::size_of;

use crate::utils::*;

pub fn part1(input: &str) -> u32 {
    unimplemented!()
}

pub fn part2(input: &str) -> u32 {
    unimplemented!()
}

fn round_pow2(n: usize) -> usize {
    1 << (size_of::<usize>() * 8 - n.leading_zeros() as usize)
}
pub fn part2fft(input: &str) -> usize {
    let nums: BitSet = input
        .lines()
        .map(FromStr::from_str)
        .map(Result::unwrap)
        .filter(|&x| x < 2020)
        .collect();
    let _n = round_pow2(nums.capacity());

    unimplemented!()
}

#[test]
fn test() {
    let input = read_input("input1.txt").unwrap();
    assert_eq!(part1(&input), 646_779);
    assert_eq!(part2(&input), 246_191_688);
    // assert_eq!(part2fft(&input), 246191688);
}
