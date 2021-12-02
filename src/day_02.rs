use std::error::Error;
use std::fs::read_to_string;
use std::ops::{AddAssign, SubAssign};
use std::str::FromStr;

use num::Integer;

fn part_1<N: Integer + AddAssign + SubAssign + Copy>(commands: &[Instruction<N>]) -> N {
    let mut h = N::zero();
    let mut d = N::zero();

    commands.iter().for_each(|command| match command {
        Instruction::Forward(n) => h += *n,
        Instruction::Down(n) => d += *n,
        Instruction::Up(n) => d -= *n,
    });

    h * d
}

fn part_2<N: Integer + AddAssign + SubAssign + Copy>(commands: &[Instruction<N>]) -> N {
    let mut h = N::zero();
    let mut d = N::zero();
    let mut aim = N::zero();

    commands.iter().for_each(|command| match command {
        Instruction::Forward(n) => {
            h += *n;
            d += aim * *n;
        }
        Instruction::Down(n) => aim += *n,
        Instruction::Up(n) => aim -= *n,
    });

    h * d
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
