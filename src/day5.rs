use std::{mem::size_of_val, ops::ControlFlow, str::from_utf8};

use crate::utils::*;

// TODO: maybe use complex numbers can make the iterations cleaner?

fn parse(input: &str) -> impl Iterator<Item = ((u16, u16), (u16, u16))> + '_ {
    input
        .lines()
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(from, to)| (from.split_once(',').unwrap(), to.split_once(',').unwrap()))
        .map(|((fromx, fromy), (tox, toy))| {
            (
                (fromx.parse::<u16>().unwrap(), fromy.parse::<u16>().unwrap()),
                (tox.parse::<u16>().unwrap(), toy.parse::<u16>().unwrap()),
            )
        })
}

fn incr(x: u16, y: u16, tiles: &mut FMap<(u16, u16), u8>) {
    tiles.entry((x, y)).and_modify(|c| *c += 1).or_insert(1);
}

fn swap_ordered(from: u16, to: u16) -> (u16, u16) {
    (from.min(to), from.max(to))
}

fn straight_line(
    mut tiles: FMap<(u16, u16), u8>,
    line @ ((fromx, fromy), (tox, toy)): ((u16, u16), (u16, u16)),
) -> FMap<(u16, u16), u8> {
    if fromx == tox {
        let (fromy, toy) = swap_ordered(fromy, toy);
        for y in fromy..=toy {
            incr(fromx, y, &mut tiles);
        }
    } else if fromy == toy {
        let (fromx, tox) = swap_ordered(fromx, tox);
        for x in fromx..=tox {
            incr(x, fromy, &mut tiles);
        }
    }
    tiles
}

pub fn part1(input: &str) -> usize {
    let tiles = parse(input).fold(fmap(5000), straight_line);

    tiles.values().filter(|&&x| x > 1).count()
}

// fn diag_line(
//     line @ ((fromx, fromy), (tox, toy)): ((u16, u16), (u16, u16)),
// ) -> impl Iterator<Item = (u16, u16)> {
//     let line @ ((fromx, fromy), (tox, toy)) = if fromx > tox {
//         ((tox, toy), (fromx, fromy))
//     } else {
//         line
//     };
//
//     if fromy > toy {
//         izip!(fromx..=tox, (toy..=fromy).rev())
//     } else {
//         izip!(fromx..=tox, fromy..=toy)
//     }
// }

pub fn part2(input: &str) -> usize {
    let tiles = parse(input).fold(fmap(5000), |tiles, line @ ((fromx, fromy), (tox, toy))| {
        let mut tiles = straight_line(tiles, line);
        if fromx != tox && fromy != toy {
            let line @ ((fromx, fromy), (tox, toy)) = if fromx > tox {
                ((tox, toy), (fromx, fromy))
            } else {
                line
            };
            if fromy > toy {
                for (x, y) in izip!(fromx..=tox, (toy..=fromy).rev()) {
                    incr(x, y, &mut tiles);
                }
            } else {
                for (x, y) in izip!(fromx..=tox, fromy..=toy) {
                    incr(x, y, &mut tiles);
                }
            }
        }
        tiles
    });

    tiles.values().filter(|&&x| x > 1).count()
}

#[test]
fn test() {
    let input = read_input("input5.txt").unwrap();
    assert_eq!(part1(&input), 7436);
    assert_eq!(part2(&input), 21104);
}
