use crate::utils::SolverResult;
use itertools::{Itertools, MinMaxResult};
use ndarray::Array1;
use std::fs::read_to_string;
use std::ops::Sub;

fn part_1(positions: &[isize]) -> isize {
    let pos: Array1<isize> = Array1::from_iter(positions.iter().cloned());
    let ones: Array1<isize> = Array1::ones(pos.raw_dim());

    if let MinMaxResult::MinMax(&min, &max) = pos.iter().minmax() {
        (min..=max)
            .map(|p| (&pos).sub(&ones * p).mapv(isize::abs).sum())
            .min()
            .unwrap()
    } else {
        0
    }
}

fn part_2(positions: &[isize]) -> isize {
    let pos: Array1<isize> = Array1::from_iter(positions.iter().cloned());
    let ones: Array1<isize> = Array1::ones(pos.raw_dim());

    if let MinMaxResult::MinMax(&min, &max) = pos.iter().minmax() {
        (min..=max)
            .map(|p| {
                (&pos)
                    .sub(&ones * p)
                    .mapv(isize::abs)
                    .mapv(|dist| dist * (dist + 1) / 2)
                    .sum()
            })
            .min()
            .unwrap()
    } else {
        0
    }
}

pub fn solve() -> SolverResult {
    let positions: Vec<isize> = read_to_string("data/day_07.txt")?
        .trim()
        .split(',')
        .map(&str::parse)
        .try_collect()?;

    println!("Part 1: {}", part_1(&positions));
    println!("Part 2: {}", part_2(&positions));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const POSITIONS: [isize; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&POSITIONS), 37);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&POSITIONS), 168);
    }
}
