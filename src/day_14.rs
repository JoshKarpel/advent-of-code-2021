use crate::utils::SolverResult;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::HashMap;
use std::fs::read_to_string;

type Pair = [char; 2];
type Polymer = Vec<char>;
type Rules = HashMap<[char; 2], char>;

fn grow(polymer: &Polymer, rules: &Rules, steps: usize) -> usize {
    let pair_counts = (0..steps).fold(
        polymer.as_slice().array_windows().cloned().counts(),
        |counts, step| {
            println!("{} {:?}", step, counts);
            let mut new_counts = HashMap::new();
            counts.iter().for_each(|(pair, count)| {
                if let Some(&insert) = rules.get(pair) {
                    let left = new_counts.entry([pair[0], insert]).or_insert(0);
                    *left += count;

                    let right = new_counts.entry([insert, pair[1]]).or_insert(0);
                    *right += count;
                } else {
                    let orig = new_counts.entry(*pair).or_insert(0);
                    *orig += count;
                }
            });
            new_counts
        },
    );

    // Now count the left character in each pair.
    // The right character is the left character of some other pair, so it will be counted then...
    let mut char_counts = HashMap::new();
    pair_counts.iter().for_each(|(pair, count)| {
        let left = char_counts.entry(pair[0]).or_insert(0);
        *left += count;
    });
    // ... except for the very last character, which is not the left character in any pair.
    let last = char_counts.entry(*polymer.last().unwrap()).or_insert(0);
    *last += 1;

    if let MinMax(min, max) = char_counts.values().minmax() {
        max - min
    } else {
        0
    }
}

fn parse_input(input: &str) -> (Polymer, Rules) {
    let mut lines = input.lines();
    let polymer: Polymer = lines.next().unwrap().chars().collect();

    lines.next().unwrap();

    let rules: Rules = lines
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(left, right)| {
            let mut left_chars = left.chars();
            (
                [left_chars.next().unwrap(), left_chars.next().unwrap()],
                right.chars().next().unwrap(),
            )
        })
        .collect();

    (polymer, rules)
}

pub fn solve() -> SolverResult {
    let (polymer, rules) = parse_input(&read_to_string("data/day_14.txt")?);

    println!("Part 1: {}", grow(&polymer, &rules, 10));
    println!("Part 2: {}", grow(&polymer, &rules, 40));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part_1_examples() {
        let (polymer, rules) = parse_input(INPUT);
        assert_eq!(grow(&polymer, &rules, 10), 1588);
    }

    #[test]
    fn part_2_examples() {
        let (polymer, rules) = parse_input(INPUT);
        assert_eq!(grow(&polymer, &rules, 40), 2188189693529);
    }
}
