use crate::utils::SolverResult;
use itertools::Itertools;
use num::Num;

use std::fs::read_to_string;

fn part_1<N: Num + PartialOrd>(depths: &[N]) -> usize {
    depths.array_windows().filter(|[a, b]| a < b).count()
}

fn part_2<N: Num + PartialOrd + Copy>(depths: &[N]) -> usize {
    part_1(
        &depths
            .array_windows()
            .map(|[a, b, c]| *a + *b + *c)
            .collect::<Vec<N>>(),
    )
}

pub fn solve() -> SolverResult {
    let depths: Vec<usize> = read_to_string("data/day_01.txt")?
        .lines()
        .map(&str::parse)
        .try_collect()?;

    println!("Part 1: {}", part_1(&depths));
    println!("Part 2: {}", part_2(&depths));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEPTHS: [usize; 10] = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&DEPTHS), 7)
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&DEPTHS), 5)
    }
}
