use crate::utils::SolverResult;
use std::fs::read_to_string;

fn bit_counts(numbers: &[&str]) -> Vec<usize> {
    numbers
        .iter()
        .fold(vec![0; numbers[0].len()], |acc, number| {
            acc.iter()
                .zip(number.chars())
                .map(|(a, c)| a + c.to_digit(10).unwrap() as usize)
                .collect()
        })
}

fn part_1(numbers: &[&str]) -> usize {
    let half_length = numbers.len() / 2;

    let gamma: String = bit_counts(numbers)
        .iter()
        .map(|x| if *x > half_length { '1' } else { '0' })
        .collect();

    let epsilon: String = gamma
        .chars()
        .map(|c| if c == '1' { '0' } else { '1' })
        .collect();

    usize::from_str_radix(&gamma, 2).unwrap() * usize::from_str_radix(&epsilon, 2).unwrap()
}

fn part_2(numbers: &[&str]) -> usize {
    let mut candidates = numbers.to_vec();

    for i in 0..numbers[0].len() {
        let half_length = candidates.len() as f64 / 2.0;
        let counts = bit_counts(&candidates);
        let target = counts
            .iter()
            .map(|x| if *x as f64 >= half_length { '1' } else { '0' })
            .nth(i)
            .unwrap();
        candidates.retain(|c| c.chars().nth(i) == Some(target));
        if candidates.len() == 1 {
            break;
        }
    }
    let oxy = candidates[0];

    let mut candidates = numbers.to_vec();

    for i in 0..numbers[0].len() {
        let half_length = candidates.len() as f64 / 2.0;
        let counts = bit_counts(&candidates);
        let target = counts
            .iter()
            .map(|x| if *x as f64 >= half_length { '0' } else { '1' })
            .nth(i)
            .unwrap();
        candidates.retain(|c| c.chars().nth(i) == Some(target));
        if candidates.len() == 1 {
            break;
        }
    }
    let co2 = candidates[0];

    usize::from_str_radix(oxy, 2).unwrap() * usize::from_str_radix(co2, 2).unwrap()
}

pub fn solve() -> SolverResult {
    let input = read_to_string("data/day_03.txt")?;
    let numbers: Vec<&str> = input.lines().collect();

    println!("Part 1: {}", part_1(&numbers));
    println!("Part 2: {}", part_2(&numbers));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: [&str; 12] = [
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&NUMBERS), 198)
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&NUMBERS), 230)
    }
}
