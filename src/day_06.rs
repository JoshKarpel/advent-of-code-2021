use crate::utils::SolverResult;
use itertools::Itertools;
use std::fs::read_to_string;

fn track(fish: &[usize], days: usize) -> usize {
    let mut counts = [0usize; 9];

    fish.iter().for_each(|f| counts[*f] += 1);

    (0..days).for_each(|_day| {
        counts
            .clone()
            .iter()
            .enumerate()
            .rev() // go in reverse order so that we go in the same direction the counts are moving
            .for_each(|(timer, count)| {
                counts[timer] -= count;
                if timer == 0 {
                    counts[8] += count;
                    counts[6] += count;
                } else {
                    counts[timer - 1] += count;
                }
            })
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
