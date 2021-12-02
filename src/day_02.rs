use ndarray::{array, Array1};
use std::error::Error;
use std::fs::read_to_string;
use std::ops::{Add, Sub};
use std::str::FromStr;

use num::Integer;

fn part_1(commands: &[Instruction<usize>]) -> usize {
    commands
        .iter()
        .fold(Array1::zeros(2), |position, command| match command {
            Instruction::Forward(distance) => position.add(array![*distance, 0]),
            Instruction::Down(distance) => position.add(array![0, *distance]),
            Instruction::Up(distance) => position.sub(array![0, *distance]),
        })
        .product()
}

fn part_2(commands: &[Instruction<usize>]) -> usize {
    commands
        .iter()
        .fold(
            (Array1::zeros(2), 0),
            |(position, aim), command| match command {
                Instruction::Forward(n) => (position.add(array![*n, aim * *n]), aim),
                Instruction::Down(n) => (position, aim + *n),
                Instruction::Up(n) => (position, aim - *n),
            },
        )
        .0
        .product()
}

pub fn solve() {
    let commands: Vec<Instruction<usize>> = read_to_string("data/day_02.txt")
        .unwrap()
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect();

    println!("Part 1: {}", part_1(&commands));
    println!("Part 2: {}", part_2(&commands));
}

enum Instruction<N: Integer> {
    Forward(N),
    Down(N),
    Up(N),
}

impl<N: Integer + FromStr> FromStr for Instruction<N> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_once(' ').ok_or("Instruction was missing a space")?;

        let distance = dist.parse().map_err(|_e| "Distance couldn't be parsed")?;

        match dir {
            "forward" => Ok(Instruction::Forward(distance)),
            "down" => Ok(Instruction::Down(distance)),
            "up" => Ok(Instruction::Up(distance)),
            _ => Err("unrecognized direction".into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static COMMANDS: [&str; 6] = [
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[test]
    fn part_1_examples() {
        let commands: Vec<Instruction<usize>> = COMMANDS
            .iter()
            .map(|line| Instruction::from_str(line).unwrap())
            .collect();

        assert_eq!(part_1(&commands), 150);
    }

    #[test]
    fn part_2_examples() {
        let commands: Vec<Instruction<usize>> = COMMANDS
            .iter()
            .map(|line| Instruction::from_str(line).unwrap())
            .collect();

        assert_eq!(part_2(&commands), 900);
    }
}
