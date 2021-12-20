use crate::utils::SolverResult;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::collections::HashMap;
use std::fs::read_to_string;

type Polymer = Vec<char>;
type Rules = HashMap<[char; 2], char>;

fn grow(polymer: Polymer, rules: &Rules) -> Polymer {
    polymer
        .as_slice()
        .array_windows()
        .fold(vec![*polymer.first().unwrap()], |mut p, pair| {
            if let Some(&insert) = rules.get(pair) {
                p.push(insert);
                p.push(pair[1]);
            } else {
                p.push(pair[1]);
            }

            p
        })
}

fn part_1(polymer: &Polymer, rules: &Rules) -> usize {
    let final_polymer: Polymer =
        (0..10).fold(polymer.clone(), |polymer, _step| grow(polymer, rules));

    if let MinMax(min, max) = final_polymer.iter().counts().values().minmax() {
        max - min
    } else {
        0
    }
}

fn part_2(polymer: &Polymer, rules: &Rules) -> usize {
    let final_polymer: Polymer = (0..40).fold(polymer.clone(), |polymer, step| {
        println!("{} {}", step, polymer.len());
        grow(polymer, rules)
    });

    if let MinMax(min, max) = final_polymer.iter().counts().values().minmax() {
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

    println!("Part 1: {}", part_1(&polymer, &rules));
    println!("Part 2: {}", part_2(&polymer, &rules));

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
        assert_eq!(part_1(&polymer, &rules), 1588);
    }

    #[test]
    fn part_2_examples() {}
}
