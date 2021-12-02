use std::error::Error;
use std::fmt::Debug;
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

impl<N: Integer + FromStr> FromStr for Instruction<N>
where
    <N as FromStr>::Err: Debug,
{
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, dist) = s.split_once(' ').unwrap();

        let distance = dist.parse().unwrap();

        Ok(match dir {
            "forward" => Instruction::Forward(distance),
            "down" => Instruction::Down(distance),
            "up" => Instruction::Up(distance),
            _ => panic!("unrecognized direction"),
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
