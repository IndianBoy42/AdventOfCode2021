use std::{ops::ControlFlow, str::from_utf8};

use crate::utils::*;

fn check_board(board: &[u8]) -> bool {
    if board
        .chunks_exact(5)
        .any(|row| row.iter().copied().all(|x| x == 0))
    {
        return true;
    }
    if (0..5).any(|i| (i..25).step_by(5).map(|i| board[i]).all(|x| x == 0)) {
        return true;
    }

    // Diagonal 1
    if (0..25).step_by(6).map(|i| board[i]).all(|x| x == 0) {
        return true;
    }
    // Diagonal 2
    if (4..25).step_by(4).map(|i| board[i]).all(|x| x == 0) {
        return true;
    }

    return false;
}

pub fn part1(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let drawn = blocks
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap);
    let mut boards = blocks
        .map(|board| {
            board
                .split_ascii_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec();

    let mut winner = None;
    let mut called = 0;
    for number in drawn {
        called = number;
        let spaces = boards.iter_mut().flat_map(|board| board.iter_mut());
        for space in spaces {
            if *space == number {
                *space = 0;
            }
        }

        winner = boards.iter().find(|board| check_board(board));
        if winner.is_some() {
            break;
        }
    }
    dbg!(&winner, called);

    winner
        .expect("Must have winner")
        .iter()
        .copied()
        .map(|x| x as usize)
        .sum::<usize>()
        * called as usize
}

pub fn part2(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let drawn = blocks
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap);
    let mut boards = blocks
        .map(|board| {
            board
                .split_ascii_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec();

    let mut winners = vec![];
    let mut called = 0;
    for number in drawn {
        called = number;
        let spaces = boards.iter_mut().flat_map(|board| board.iter_mut());
        for space in spaces {
            if *space == number {
                *space = 0;
            }
        }

        winners.extend(boards.drain_filter(|board| check_board(board)));
        if boards.is_empty() {
            break;
        }
    }
    let winner = winners.last();
    dbg!(&winner, called);

    winner
        .expect("Must have winner")
        .iter()
        .copied()
        .map(|x| x as usize)
        .sum::<usize>()
        * called as usize
}

#[test]
fn test() {
    let input = read_input("input4.txt").unwrap();
    assert_eq!(part1(&input), 60368);
    assert_eq!(part2(&input), 0);
}
