use crate::utils::*;

const NEWBORN: usize = 8;
const REFRESH: usize = 6;

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
        dbg!(&fishes);
        let new = fishes.remove(0);
        fishes[REFRESH] += new;
        fishes.push(new);
    }

    fishes.into_iter().sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, 80)
}

pub fn part2(input: &str) -> usize {
    solve(input, 256)
}

#[test]
fn test() {
    let input = read_input("input6.txt").unwrap();
    assert_eq!(part1(&input), 350_917);
    assert_eq!(part2(&input), 1_592_918_715_629);
}
