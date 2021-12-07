use crate::utils::*;

fn median<T>(data: &[T]) -> Option<T>
where
    T: std::ops::Add<Output = T> + std::cmp::PartialOrd + Copy,
{
    use std::cmp::Ordering;
    fn partition<T>(data: &[T]) -> Option<(Vec<T>, T, Vec<T>)>
    where
        T: std::cmp::PartialOrd + Copy,
    {
        (!data.is_empty()).then(|| {
            let (pivot_slice, tail) = data.split_at(1);
            let pivot = pivot_slice[0];
            let (left, right) = tail.iter().fold((vec![], vec![]), |mut splits, next| {
                {
                    let (ref mut left, ref mut right) = &mut splits;
                    if next < &pivot {
                        left.push(*next);
                    } else {
                        right.push(*next);
                    }
                }
                splits
            });

            (left, pivot, right)
        })
    }
    fn select<T>(data: &[T], k: usize) -> Option<T>
    where
        T: std::cmp::PartialOrd + Copy,
    {
        let part = partition(data);

        match part {
            None => None,
            Some((left, pivot, right)) => {
                let pivot_idx = left.len();

                match pivot_idx.cmp(&k) {
                    Ordering::Equal => Some(pivot),
                    Ordering::Greater => select(&left, k),
                    Ordering::Less => select(&right, k - (pivot_idx + 1)),
                }
            }
        }
    }

    let size = data.len();

    match size {
        even if even % 2 == 0 => select(data, even / 2),
        odd => select(data, odd / 2),
    }
}

pub fn part1(input: &str) -> isize {
    let pos = input
        .trim()
        .split(',')
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        // .sorted()
        .collect_vec();

    if true {
        // This has O(n) complexity but is slower
        // let median = median(&pos).unwrap();

        let mut pos = pos;
        pos.sort_unstable();
        let median = pos[pos.len() / 2];

        pos.iter().map(|x| (x - median).abs()).sum::<isize>() as isize
    } else {
        let test = |tar: isize| pos.iter().map(|x| (x - tar).abs()).sum::<isize>() as isize;
        pos.iter().copied().map(test).min().unwrap()
    }
}

pub fn part2(input: &str) -> isize {
    let pos = input
        .trim()
        .split(',')
        .map(str::parse::<isize>)
        .map(Result::unwrap)
        // .sorted()
        .collect_vec();

    let sum: isize = pos.iter().copied().sum();
    let len = pos.len() as isize;
    let hi = (sum + len - 1) / len;
    let lo = sum / len;

    let test = |tar: isize| {
        pos.iter()
            .map(|x| (x - tar).abs())
            .map(|d| d * (d + 1) / 2)
            .sum::<isize>() as isize
    };

    [lo, hi].into_iter().map(test).min().unwrap()
}

#[test]
fn test() {
    let input = read_input("input7.txt").unwrap();
    assert_eq!(part2(&input), 104_149_091);
    assert_eq!(part1(&input), 364_898);
}
