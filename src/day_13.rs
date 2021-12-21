use itertools::Either::{Left, Right};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

use crate::utils::SolverResult;

type Dots = HashSet<(usize, usize)>;

enum FoldDirection {
    X,
    Y,
}

struct Fold {
    direction: FoldDirection,
    coordinate: usize,
}

fn fold(dots: &Dots, fold: &Fold) -> Dots {
    dots.iter()
        .map(|&(x, y)| match fold.direction {
            FoldDirection::X => {
                if x > fold.coordinate {
                    ((2 * fold.coordinate) - x, y)
                } else {
                    (x, y)
                }
            }
            FoldDirection::Y => {
                if y > fold.coordinate {
                    (x, (2 * fold.coordinate) - y)
                } else {
                    (x, y)
                }
            }
        })
        .collect()
}

fn part_1(dots: &Dots, folds: &[Fold]) -> usize {
    fold(dots, folds.first().unwrap()).len()
}

fn part_2(dots: &Dots, folds: &[Fold]) -> String {
    let dots = folds.iter().fold(dots.clone(), |d, f| fold(&d, f));

    let (x_folds, y_folds): (Vec<usize>, Vec<usize>) =
        folds.iter().partition_map(|f| match f.direction {
            FoldDirection::X => Left(f.coordinate),
            FoldDirection::Y => Right(f.coordinate),
        });

    let x_max = *x_folds.iter().min().unwrap();
    let y_max = *y_folds.iter().min().unwrap();

    let mut letters: String = (0..y_max)
        .map(|y| {
            let mut line: String = (0..x_max)
                .map(|x| if dots.contains(&(x, y)) { '█' } else { ' ' })
                .collect();
            line.push('\n');
            line
        })
        .collect();

    letters.insert(0, '\n');

    letters
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(x|y)=(\d+)").unwrap();
}

fn parse_input(input: &str) -> (Dots, Vec<Fold>) {
    let (dot_lines, fold_lines): (Vec<&str>, Vec<&str>) = input
        .lines()
        .filter(|line| !line.is_empty())
        .partition(|line| !line.starts_with("fold"));

    let dots = dot_lines
        .iter()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();

    let folds = fold_lines
        .iter()
        .map(|line| {
            let captures = RE.captures(line).unwrap();
            Fold {
                direction: if &captures[1] == "x" {
                    FoldDirection::X
                } else {
                    FoldDirection::Y
                },
                coordinate: captures[2].parse().unwrap(),
            }
        })
        .collect();

    (dots, folds)
}

pub fn solve() -> SolverResult {
    let (dots, folds) = parse_input(&read_to_string("data/day_13.txt")?);

    println!("Part 1: {}", part_1(&dots, &folds));
    println!("Part 2: {}", part_2(&dots, &folds));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn part_1_examples() {
        let (dots, folds) = parse_input(INPUT);
        assert_eq!(part_1(&dots, &folds), 17);
    }
}
