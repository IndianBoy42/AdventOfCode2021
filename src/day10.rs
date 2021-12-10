use std::ops::Not;

use crate::utils::*;

const OPEN: [u8; 4] = [b'(', b'[', b'{', b'<'];
const CLOSE: [u8; 4] = [b')', b']', b'}', b'>'];
const SCORES: [usize; 4] = [3, 57, 1197, 25137];

pub fn part1(input: &str) -> usize {
    let mut stack = Vec::with_capacity(1024);
    input
        .lines()
        .filter_map(|line| {
            // stack.clear(); // Not technically necessary for the puzzle input
            line.bytes().find_map(|sym| {
                if let Some(pos) = OPEN.iter().position(|&b| b == sym) {
                    stack.push(pos);
                    None
                } else if let Some(pos) = CLOSE.iter().position(|&b| b == sym) {
                    if let Some(&top) = stack.last() {
                        if pos == top {
                            stack.pop();
                            None
                        } else {
                            // Error
                            Some(pos)
                        }
                    } else {
                        // Error // Never happens in puzzle input
                        Some(pos)
                    }
                } else {
                    unreachable!("Invalid Input")
                }
            })
        })
        .map(|i| SCORES[i])
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut stack = Vec::with_capacity(1024);
    let mut scores = input
        .lines()
        .filter_map(|line| {
            stack.clear();
            line.bytes()
                .any(|sym| {
                    if let Some(pos) = OPEN.iter().position(|&b| b == sym) {
                        stack.push(pos);
                        false
                    } else if let Some(pos) = CLOSE.iter().position(|&b| b == sym) {
                        if let Some(&top) = stack.last() {
                            if pos == top {
                                stack.pop();
                                false
                            } else {
                                // Error
                                true
                            }
                        } else {
                            // Error
                            true
                        }
                    } else {
                        unreachable!("Invalid Input")
                    }
                })
                .not()
                .then_some(&stack)
                .map(|stack| {
                    stack
                        .iter()
                        .rev()
                        .fold(0, |score, close| score * 5 + close + 1)
                })
        })
        .collect_vec();

    let med = &scores.len() / 2;
    *scores.select_nth_unstable(med).1
}

#[test]
fn test() {
    let input = read_input("input10.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part1(&input), 374_061);
    assert_eq!(part2(&input), 2_116_639_949);
}
