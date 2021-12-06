use std::mem::size_of_val;

use ndarray::{Array1, Array2};

use crate::utils::*;

const NEWBORN: usize = 8;
const REFRESH: usize = 6;

fn solve_log(input: &str, days: usize) -> usize {
    let mut fishes = input
        .trim()
        .split(',')
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .fold(ArrayVec::from([0usize; NEWBORN + 1]), |mut acc, num| {
            acc[num as usize] += 1;
            acc
        });

    let steps = successors(Some(2), |x| Some(x * 2)).take_while(|&x| x <= days);
    let first = ndarray::array![
        [0, 1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 1, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 1, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 1, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 1, 0],
        [0, 0, 0, 0, 0, 0, 0, 0, 1],
        [1, 0, 0, 0, 0, 0, 0, 0, 0]
    ];
    let mats = {
        let mut mats: Vec<Array2<usize>> = steps
            .scan(first.clone(), |mat, _| {
                *mat = mat.dot(mat);
                Some(mat.clone())
            })
            .collect_vec();
        mats.insert(0, first);
        dbg!(mats)
    };

    let mat = (0..(size_of_val(&days) * 8))
        .filter(|bitn| days & (1 << bitn) != 0)
        .map(|x| dbg!(x))
        .fold(Array2::eye(NEWBORN + 1), |mat, i| mat.dot(&mats[i]));
    mat.dot(&fishes.into_iter().collect::<Array1<_>>()).sum()
}
fn solve(input: &str, days: usize) -> usize {
    let mut fishes = input
        .trim()
        .split(',')
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .fold(ArrayVec::from([0; NEWBORN + 1]), |mut acc, num| {
            acc[num as usize] += 1;
            acc
        });

    for i in 0..days {
        let new = fishes.remove(0);
        fishes.push(new);
        fishes[REFRESH] += new;
    }

    fishes.into_iter().sum()
}

pub fn part1(input: &str) -> usize {
    solve_log(input, 80)
}

pub fn part2(input: &str) -> usize {
    solve_log(input, 256)
}

#[test]
fn test() {
    let input = read_input("input6.txt").unwrap();
    assert_eq!(part1(&input), 350_917);
    assert_eq!(part2(&input), 1_592_918_715_629);
}
