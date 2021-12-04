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

    false
}

fn board_score(winner: impl IntoIterator<Item = u8>, called: usize) -> usize {
    winner.into_iter().map(|x| x as usize).sum::<usize>() * called
}

pub fn play_bingo(
    drawn: impl Iterator<Item = u8>,
    mut boards: Vec<Vec<u8>>,
    mut f: impl FnMut(Vec<u8>, u8) -> bool,
) {
    for number in drawn {
        let spaces = boards.iter_mut().flat_map(|board| board.iter_mut());
        for space in spaces {
            if *space == number {
                *space = 0;
            }
        }

        if boards
            .drain_filter(|board| check_board(board))
            .any(|board| f(board, number))
        {
            break;
        }
        if boards.is_empty() {
            break;
        }
    }
}

pub fn drawn(input: &str) -> impl Iterator<Item = u8> + '_ {
    input.split(',').map(str::parse).map(Result::unwrap)
}
pub fn boards<'a>(input: impl Iterator<Item = &'a str>) -> Vec<Vec<u8>> {
    input
        .map(|board| {
            board
                .split_ascii_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect_vec()
        })
        .collect_vec()
}

pub fn part1(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let drawn = drawn(blocks.next().unwrap());
    let boards = boards(blocks);

    let mut winner = None;
    let mut called = 0;
    play_bingo(drawn, boards, |board, number| {
        winner = Some(board);
        called = number;
        true
    });

    board_score(winner.expect("Must have winner"), called as usize)
}

pub fn part2(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let drawn = drawn(blocks.next().unwrap());
    let boards = boards(blocks);

    let mut winner = None;
    let mut called = 0;
    play_bingo(drawn, boards, |board, number| {
        winner = Some(board);
        called = number;
        false
    });

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
    assert_eq!(part2(&input), 17435);
}
