use crate::utils::SolverResult;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn track(fish: &[u8], days: usize) -> usize {
    (0..days)
        .fold(fish.iter().cloned().counts(), |counts, _day| {
            let mut new_counts = HashMap::new();

            counts.iter().for_each(|(&k, &v)| {
                if k == 0 {
                    new_counts.insert(8, v);
                    let six = new_counts.entry(6).or_insert(0);
                    *six += v;
                } else {
                    let down_one = new_counts.entry(k - 1).or_insert(0);
                    *down_one += v;
                }
            });

            new_counts
        })
        .values()
        .sum()
}

fn part_1(fish: &[u8]) -> usize {
    track(fish, 80)
}

fn part_2(fish: &[u8]) -> usize {
    track(fish, 256)
}

pub fn solve() -> SolverResult {
    let fish: Vec<u8> = read_to_string("data/day_06.txt")?
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

    const FISH: [u8; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&FISH), 5934);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&FISH), 26984457539);
    }
}
