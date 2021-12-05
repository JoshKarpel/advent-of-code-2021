use crate::utils::SolverResult;
use itertools::Itertools;
use std::fs::read_to_string;
use std::ops::ControlFlow::{Break, Continue};

fn bit_counts(numbers: &[usize], bit_width: usize) -> Vec<usize> {
    numbers.iter().fold(vec![0; bit_width], |counts, number| {
        counts
            .iter()
            .enumerate()
            .map(|(bit_position, count)| {
                count + ((number & (1 << bit_position)) >> bit_position) // "is there a 1 in bit_position"
            })
            .collect()
    })
}

fn part_1(numbers: &[usize], bit_width: usize) -> usize {
    let gamma: usize = bit_counts(numbers, bit_width)
        .iter()
        .enumerate()
        .map(|(bit_position, &count)| {
            ((count as f64 > (numbers.len() as f64 / 2.0)) as usize) << bit_position
        })
        .sum();

    let mask: usize = (1 << bit_width) - 1; // 0b111111 - 0b1 = 0b011111 ; it turns the first 1 into a zero and makes every digit below it a 1
    let epsilon = !gamma & mask;

    epsilon * gamma
}

fn find_target(
    numbers: &[usize],
    bit_width: usize,
    target_comparison: fn(count: usize, half_length: f64) -> usize,
) -> usize {
    (0..bit_width)
        .try_fold(numbers.to_vec(), |candidates, bit_position| {
            let target: usize = bit_counts(&candidates, bit_width)
                .iter()
                .enumerate()
                .map(|(bit_position, &count)| {
                    target_comparison(count, candidates.len() as f64 / 2.0) << bit_position
                })
                .sum();

            let new_candidates: Vec<usize> = candidates
                .iter()
                .cloned()
                .filter(|candidate| {
                    // is the target bit a zero after xor (i.e., are they the same)?
                    (candidate ^ target) & (1 << (bit_width - bit_position - 1)) == 0
                })
                .collect();

            if new_candidates.len() == 1 {
                Break(new_candidates)
            } else {
                Continue(new_candidates)
            }
        })
        .break_value()
        .unwrap()[0]
}

fn part_2(numbers: &[usize], bit_width: usize) -> usize {
    let oxy = find_target(numbers, bit_width, |count, half_length| {
        (count as f64 >= half_length) as usize
    });

    let co2 = find_target(numbers, bit_width, |count, half_length| {
        ((count as f64) < half_length) as usize
    });

    oxy * co2
}

pub fn solve() -> SolverResult {
    let input = read_to_string("data/day_03.txt")?;
    let numbers: Vec<usize> = input
        .lines()
        .map(|n| usize::from_str_radix(n, 2))
        .try_collect()?;

    println!("Part 1: {}", part_1(&numbers, 12));
    println!("Part 2: {}", part_2(&numbers, 12));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const WIDTH: usize = 5;
    const NUMBERS: [usize; 12] = [
        0b00100usize,
        0b11110usize,
        0b10110usize,
        0b10111usize,
        0b10101usize,
        0b01111usize,
        0b00111usize,
        0b11100usize,
        0b10000usize,
        0b11001usize,
        0b00010usize,
        0b01010usize,
    ];

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&NUMBERS, WIDTH), 198)
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&NUMBERS, WIDTH), 230)
    }
}
