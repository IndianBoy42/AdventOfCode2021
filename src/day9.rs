use crate::{
    grid::{self, adj_neighbours, Grid2D},
    searcher::BFSearcher,
    utils::*,
};

type MapGrid2D<T> = FMap<(u8, u8), T>;
type Grid = Grid2D<u8>;
// type Grid = MapGrid2D<u8>;

fn parse(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.bytes()
                .enumerate()
                .map(move |(c, cha)| ((r as _, c as _), cha - b'0'))
        })
        // .collect::<FMap<(usize, usize), _>>();
        .collect::<Grid>()
}

fn minima(map: &Grid) -> impl Iterator<Item = ((usize, usize), u8)> + '_ {
    map.iter()
        .map(|((r, c), &num)| ((r, c), num))
        .filter(|&((r, c), num)| {
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

    minima(&map).map(|(_, num)| 1 + num as usize).sum::<usize>()
}

pub fn part2(input: &str) -> usize {
    let map = parse(input);

    let points = minima(&map).collect_vec();

    points
        .into_iter()
        .map(|((row, col), _)| {
            BFSearcher::<(u8, u8), FSet<(u8, u8)>, _>::new(
                (row as u8, col as u8),
                |p: &(u8, u8)| {
                    adj_neighbours(*p)
                        .filter(|&(r, c)| map.get(&(r as _, c as _)).map_or(false, |&h| h != 9))
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
