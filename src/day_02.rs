use ndarray::{array, Array1};
use std::error::Error;
use std::fs::read_to_string;
use std::ops::{Add, Sub};
use std::str::FromStr;

use num::Num;

fn part_1<N: Num + Copy>(commands: &[Command<N>]) -> N {
    commands
        .iter()
        .fold(Array1::zeros(2), |position, command| {
            match &command.direction {
                Direction::Forward => position.add(array![command.distance, N::zero()]),
                Direction::Down => position.add(array![N::zero(), command.distance]),
                Direction::Up => position.sub(array![N::zero(), command.distance]),
            }
        })
        .product()
}

fn part_2<N: Num + Copy>(commands: &[Command<N>]) -> N {
    commands
        .iter()
        .fold(
            (Array1::zeros(2), N::zero()),
            |(position, aim), command| match &command.direction {
                Direction::Forward => (
                    position.add(array![command.distance, aim * command.distance]),
                    aim,
                ),
                Direction::Down => (position, aim + command.distance),
                Direction::Up => (position, aim - command.distance),
            },
        )
        .0
        .product()
}

pub fn solve() {
    let commands: Vec<Command<usize>> = read_to_string("data/day_02.txt")
        .unwrap()
        .lines()
        .map(|line| Command::from_str(line).unwrap())
        .collect();

    println!("Part 1: {}", part_1(&commands));
    println!("Part 2: {}", part_2(&commands));
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(format!("unrecognized direction {}", s)),
        }
    }
}

struct Command<N> {
    direction: Direction,
    distance: N,
}

impl<N: Num + FromStr> FromStr for Command<N> {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_once(' ').ok_or("Instruction was missing a space")?;

        Ok(Command {
            direction: Direction::from_str(dir)?,
            distance: dist.parse().map_err(|_e| "Distance couldn't be parsed")?,
        })
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
        let commands: Vec<Command<usize>> = COMMANDS
            .iter()
            .map(|line| Command::from_str(line).unwrap())
            .collect();

        assert_eq!(part_1(&commands), 150);
    }

    #[test]
    fn part_2_examples() {
        let commands: Vec<Command<usize>> = COMMANDS
            .iter()
            .map(|line| Command::from_str(line).unwrap())
            .collect();

        assert_eq!(part_2(&commands), 900);
    }
}
