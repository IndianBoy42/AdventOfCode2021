use crate::utils::*;

pub fn part1(input: &str) -> isize {
    let pos = input
        .trim()
        .split(',')
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        .sorted()
        .collect_vec();

    let median = dbg!(pos[pos.len() / 2]);

    pos.iter().map(|x| (x - median).abs()).sum::<isize>() as isize
}

pub fn part2(input: &str) -> isize {
    let pos = input
        .trim()
        .split(',')
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        // .sorted()
        .collect_vec();

    let min = *pos.iter().min().unwrap();
    let max = *pos.iter().max().unwrap();

    let target = (min..max)
        .min_by_key(|tar| {
            pos.iter()
                .map(|x| (x - tar).abs())
                .map(|d| d * (d + 1) / 2)
                .sum::<isize>() as isize
        })
        .unwrap();

    pos.iter()
        .map(|x| (x - target).abs())
        .map(|d| d * (d + 1) / 2)
        .sum::<isize>() as isize
}

#[test]
fn test() {
    let input = read_input("input7.txt").unwrap();
    assert_eq!(part1(&input), 364_898);
    assert_eq!(part2(&input), 104_149_091);
}
