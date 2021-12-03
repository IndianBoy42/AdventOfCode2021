use std::str::from_utf8;

use crate::utils::*;

pub fn part1(input: &str) -> usize {
    let counts = input.lines().fold([(0, 0); 12], |mut acc, line| {
        line.bytes()
            .zip(&mut acc)
            .for_each(|(byte, (zeros, ones))| {
                if byte == b'0' {
                    *zeros += 1;
                } else {
                    *ones += 1;
                }
            });
        acc
    });
    let (gamma, eps) = counts
        .into_iter()
        .fold((0, 0), |(gamma, eps), (zeros, ones)| {
            (
                gamma << 1 | (if zeros < ones { 1 } else { 0 }),
                eps << 1 | (if zeros < ones { 0 } else { 1 }),
            )
        });
    eps * gamma
}

pub fn part2(input: &str) -> usize {
    let lines = input.lines();
    let n = lines.clone().next().unwrap().len();

    let mut ox = lines.clone().map(str::as_bytes).collect_vec();
    for i in 0..12 {
        let ones = ox.iter().map(|x| x[i]).filter(|&x| x == b'1').count();
        let gamma = if ones >= (ox.len() - ones) {
            b'1'
        } else {
            b'0'
        };
        ox = ox.into_iter().filter(|s| s[i] == gamma).collect_vec();
        if ox.len() == 1 {
            break;
        } else if ox.is_empty() {
            unreachable!();
        }
    }

    let mut co = lines.map(str::as_bytes).collect_vec();
    for i in 0..12 {
        let ones = co.iter().map(|x| x[i]).filter(|&x| x == b'1').count();
        let gamma = if ones >= (co.len() - ones) {
            b'1'
        } else {
            b'0'
        };
        co = co.into_iter().filter(|s| s[i] != gamma).collect_vec();
        if co.len() == 1 {
            break;
        } else if co.is_empty() {
            unreachable!();
        }
    }

    let ox = ox.first().unwrap().iter().fold(0, |gamma, &bit| {
        gamma << 1 | (if bit == b'1' { 1 } else { 0 })
    });
    let co = co.first().unwrap().iter().fold(0, |gamma, &bit| {
        gamma << 1 | (if bit == b'1' { 1 } else { 0 })
    });

    ox * co
}

#[test]
fn test() {
    let input = read_input("input").unwrap();
    assert_eq!(part1(&input), 2_498_354);
    assert_eq!(part2(&input), 3_277_956);
}
