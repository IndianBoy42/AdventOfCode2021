use crate::utils::*;

fn solve(input: &str, days: usize) -> usize {
    let mut fishes = input
        .trim()
        .split(',')
        .map(str::parse::<u8>)
        .map(Result::unwrap)
        .counts();

    for i in 0..days {
        let new = *fishes.get(&0).unwrap_or(&0);
        fishes = fishes
            .into_iter()
            .map(|(k, v)| if k == 0 { (6, v) } else { (k - 1, v) })
            .into_grouping_map()
            .sum();
        fishes.entry(8).and_modify(|x| *x += new).or_insert(new);
    }

    fishes.values().sum()
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
