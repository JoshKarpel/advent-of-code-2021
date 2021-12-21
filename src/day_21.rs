use crate::utils::SolverResult;
use std::fs::read_to_string;

fn part_1(positions: &Vec<usize>) -> usize {
    let mut positions = positions.clone();

    let mut rolls = (1..=1000).cycle();
    let mut roll_count = 0;

    let mut scores = vec![0; positions.len()];

    'outer: loop {
        for (position, score) in positions.iter_mut().zip(scores.iter_mut()) {
            let dist: usize = (0..3).map(|_| rolls.next().unwrap()).sum();
            roll_count += 3;
            *position = (*position + dist) % 10;
            *score += if *position == 0 { 10 } else { *position }; // 0 is 10, otherwise they match

            if *score >= 1000 {
                break 'outer;
            }
        }
    }

    roll_count * scores.iter().min().unwrap()
}

fn part_2(_positions: &Vec<usize>) -> usize {
    0
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|line| line.chars().last().unwrap().to_digit(10).unwrap() as usize)
        .collect()
}

pub fn solve() -> SolverResult {
    let positions = parse_input(&read_to_string("data/day_21.txt")?);

    println!("Part 1: {}", part_1(&positions));
    println!("Part 2: {}", part_2(&positions));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&parse_input(INPUT)), 739785);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&parse_input(INPUT)), 444356092776315);
    }
}
