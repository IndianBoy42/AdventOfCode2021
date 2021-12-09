use crate::{
    grid::{self, adj_neighbours, Grid2D},
    searcher::BFSearcher,
    utils::*,
};

fn parse(input: &str) -> grid::Grid2D<u8> {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.bytes()
                .enumerate()
                .map(move |(c, cha)| ((r, c), cha - b'0'))
        })
        // .collect::<FMap<(usize, usize), _>>();
        .collect::<Grid2D<_>>()
}

fn minima(map: &Grid2D<u8>) -> impl Iterator<Item = ((usize, usize), &u8)> {
    map.iter().filter(|&((r, c), &num)| {
        let a = map.get(&(r + 1, c)).copied().unwrap_or(10);
        let b = r
            .checked_sub(1)
            .and_then(|r| map.get(&(r, c)))
            .copied()
            .unwrap_or(10);
        let d = c
            .checked_sub(1)
            .and_then(|c| map.get(&(r, c)))
            .copied()
            .unwrap_or(10);
        let c = map.get(&(r, c + 1)).copied().unwrap_or(10);

        (num < a) && (num < b) && (num < c) && (num < d)
    })
}

pub fn part1(input: &str) -> usize {
    let map = parse(input);

    minima(&map)
        .map(|(_, &num)| 1 + num as usize)
        .sum::<usize>()
}

pub fn part2(input: &str) -> usize {
    let map = parse(input);

    let points = minima(&map).collect_vec();

    points
        .into_iter()
        .map(|((row, col), _)| {
            BFSearcher::<(usize, usize), FSet<(usize, usize)>, _>::new(
                (row, col),
                |p: &(usize, usize)| {
                    adj_neighbours(*p).filter(|p| map.get(p).map_or(false, |&h| h != 9))
                },
            )
            .check()
            .count()
        })
        .sorted_by_key(|&visited| Reverse(visited))
        .take(3)
        .product()
}

#[test]
fn test() {
    let input = read_input("input9.txt").unwrap();
    // let input = read_input("test.txt").unwrap();
    assert_eq!(part2(&input), 920_448);
    assert_eq!(part1(&input), 528);
}
