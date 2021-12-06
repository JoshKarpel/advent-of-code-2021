use crate::utils::SolverResult;
use itertools::Itertools;
use std::fs::read_to_string;

fn track(fish: &[usize], days: usize) -> usize {
    let mut counts = [0usize; 9];

    fish.iter().for_each(|f| counts[*f] += 1);

    (0..days).for_each(|_day| {
        counts.rotate_left(1);
        // the counts that were in 0 are now in 8 (they now represent the new fish),
        // and should also be added to 6 (the old fish, now reset).
        counts[6] += counts[8];
    });

    counts.iter().sum()
}

fn part_1(fish: &[usize]) -> usize {
    track(fish, 80)
}

fn part_2(fish: &[usize]) -> usize {
    track(fish, 256)
}

pub fn solve() -> SolverResult {
    let fish: Vec<usize> = read_to_string("data/day_06.txt")?
        .trim()
        .split(',')
        .map(&str::parse)
        .try_collect()?;

    println!("Part 1: {}", part_1(&fish));
    println!("Part 2: {}", part_2(&fish));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const FISH: [usize; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&FISH), 5934);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&FISH), 26984457539);
    }
}
