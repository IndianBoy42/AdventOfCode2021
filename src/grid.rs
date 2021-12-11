use std::convert::TryInto;
use std::iter::{FromIterator, TrustedLen};

use arrayvec::ArrayVec;
use itertools::{izip, Itertools};
use ndarray::{azip, Array2, ArrayBase};
use num::{Num, Zero};
use rayon::collections::binary_heap::IntoIter;
use rayon::iter::FromParallelIterator;

use crate::utils::FMap;

#[derive(Debug, Clone)]
pub struct Grid2D<T> {
    pub arr: Array2<T>,
}

impl<T: num_traits::Zero + Copy> Grid2D<T> {
    pub fn from_iter_w_shape<I: IntoIterator<Item = T>>(shape: (usize, usize), iter: I) -> Self {
        let mut arr = Array2::zeros(shape);
        izip!(arr.iter_mut(), iter.into_iter()).for_each(|(out, elem)| {
            *out = elem;
        });

        // let arr: ArrayBase<_, _> = iter.into_iter().collect();
        // let arr = arr.into_shape(shape).unwrap();
        Grid2D { arr }
    }
}

impl<T: Default + std::cmp::PartialEq> Grid2D<T> {
    pub fn contains_key(&self, &(x, y): &(usize, usize)) -> bool {
        self.arr.get((x, y)).map_or(false, |x| x != &T::default())
    }
}

impl<T> Grid2D<T> {
    pub fn len(&self) -> usize {
        self.arr.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.arr.iter()
    }

    pub fn get(&self, &(x, y): &(usize, usize)) -> Option<&T> {
        self.arr.get((x, y))
    }

    pub fn get_mut(&mut self, &(x, y): &(usize, usize)) -> Option<&mut T> {
        self.arr.get_mut((x, y))
    }

    pub fn iter<I>(&self) -> impl Iterator<Item = ((I, I), &T)>
    where
        usize: TryInto<I>,
        <usize as std::convert::TryInto<I>>::Error: std::fmt::Debug,
    {
        self.arr
            .indexed_iter()
            .map(|((x, y), z)| ((x.try_into().unwrap(), y.try_into().unwrap()), z))
    }

    pub fn iter_mut<I>(&mut self) -> impl Iterator<Item = ((I, I), &mut T)>
    where
        usize: TryInto<I>,
        <usize as std::convert::TryInto<I>>::Error: std::fmt::Debug,
    {
        self.arr
            .indexed_iter_mut()
            .map(|((x, y), z)| ((x.try_into().unwrap(), y.try_into().unwrap()), z))
    }
}
pub fn adj_neighbours<T: num_traits::CheckedSub + num_traits::CheckedAdd + Num + Copy>(
    (x, y): (T, T),
) -> impl Iterator<Item = (T, T)> {
    let a = x.checked_add(&T::one()).map(|x| (x, y));
    let b = x.checked_sub(&T::one()).map(|x| (x, y));
    let d = y.checked_sub(&T::one()).map(|y| (x, y));
    let c = y.checked_add(&T::one()).map(|y| (x, y));

    [a, b, c, d].into_iter().flatten()
}
pub fn all_neighbours<T>((x, y): (T, T)) -> arrayvec::ArrayVec<(T, T), 9_usize>
where
    T: Num + Copy + std::cmp::PartialOrd,
{
    let mut arr = ArrayVec::<_, 9>::new();
    arr.push((x, y));
    arr.push((x, y + T::one()));
    arr.push((x + T::one(), y));
    arr.push((x + T::one(), y + T::one()));
    if x > T::zero() {
        arr.push((x - T::one(), y));
        arr.push((x - T::one(), y + T::one()));
        if y > T::zero() {
            arr.push((x - T::one(), y - T::one()));
        }
    }
    if y > T::zero() {
        arr.push((x, y - T::one()));
        arr.push((x + T::one(), y - T::one()));
    }
    arr
}

// impl<Inner, T> FromIterator<Inner> for Grid2D<T>
impl<T> Grid2D<T> {
    fn from_iter<Inner, I>(iter: I) -> Self
    where
        Inner: IntoIterator<Item = T> + ExactSizeIterator + TrustedLen,
        I: IntoIterator<Item = Inner>,
        T: Zero + Copy,
    {
        let mut iter = iter.into_iter().peekable();
        let first = iter.peek().unwrap();
        let ymax = first.size_hint().0;
        let iter = iter.collect_vec();
        let xmax = iter.len();

        Self::from_iter_w_shape((xmax, ymax), iter.into_iter().flatten())
    }
}

impl<T: Copy + num_traits::Zero> FromIterator<((usize, usize), T)> for Grid2D<T> {
    fn from_iter<I: IntoIterator<Item = ((usize, usize), T)>>(iter: I) -> Self {
        let map = iter.into_iter().collect::<Vec<_>>();
        let xmax = map.iter().map(|&((x, y), _)| x).max().unwrap() + 1;
        let ymax = map.iter().map(|&((x, y), _)| y).max().unwrap() + 1;

        // let mut arr = Array2::zeros((xmax, ymax));
        // for ((x, y), z) in map {
        //     arr[(x, y)] = z;
        // }
        // Grid2D { arr }

        Self::from_iter_w_shape((xmax, ymax), map.into_iter().map(|(_, z)| z))
    }
}

// TODO: collect par_iter
// impl<T: Send> FromParallelIterator<T> for Grid2D<T> {
//     fn from_par_iter<I>(par_iter: I) -> Self
//     where
//         I: rayon::iter::IntoParallelIterator<Item = T>,
//     {
//         todo!()
//     }
// }
// impl<T: Send> FromParallelIterator<((usize, usize), T)> for Grid2D<T> {
//     fn from_par_iter<I>(par_iter: I) -> Self
//     where
//         I: rayon::iter::IntoParallelIterator<Item = ((usize, usize), T)>,
//     {
//         todo!()
//     }
// }

// TODO: Should I have this?
// impl<T> Deref for Grid2D<T> {
//     type Target = Array2<T>;
//     fn deref(&self) -> &Self::Target {
//         &self.arr
//     }
// }
