use crate::utils::SolverResult;
use itertools::Itertools;
use lazy_static::lazy_static;
use ndarray::Array2;
use regex::Regex;
use std::fs::read_to_string;

#[derive(Debug, Clone, Copy)]
struct Line {
    start_x: isize,
    start_y: isize,
    end_x: isize,
    end_y: isize,
}

impl Line {
    fn length(&self) -> isize {
        self.start_x
            .abs_diff(self.end_x)
            .max(self.start_y.abs_diff(self.end_y)) as isize
    }

    fn dx(&self) -> isize {
        (self.end_x - self.start_x).signum()
    }

    fn dy(&self) -> isize {
        (self.end_y - self.start_y).signum()
    }
}

fn count_overlaps(lines: &[Line]) -> usize {
    let largest_dim = 1 + lines
        .iter()
        .flat_map(|line| [line.start_x, line.end_x, line.start_y, line.end_y])
        .max()
        .unwrap() as usize;

    lines
        .iter()
        .flat_map(|line| {
            let dx = line.dx();
            let dy = line.dy();
            (0..=line.length()).map(move |d| (line.start_x + (dx * d), line.start_y + (dy * d)))
        })
        .fold(
            Array2::zeros((largest_dim, largest_dim)),
            |mut counts: Array2<u8>, (x, y)| {
                counts[[x as usize, y as usize]] += 1;
                counts
            },
        )
        .iter()
        .filter(|&&v| v >= 2)
        .count()
}

fn part_1(lines: &[Line]) -> usize {
    count_overlaps(
        &lines
            .iter()
            .cloned()
            .filter(|line| line.start_x == line.end_x || line.start_y == line.end_y)
            .collect_vec(),
    )
}

fn part_2(lines: &[Line]) -> usize {
    count_overlaps(lines)
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| {
            let captures = RE.captures(line).unwrap();
            Line {
                start_x: captures[1].parse().unwrap(),
                start_y: captures[2].parse().unwrap(),
                end_x: captures[3].parse().unwrap(),
                end_y: captures[4].parse().unwrap(),
            }
        })
        .collect()
}

pub fn solve() -> SolverResult {
    let lines = parse_input(&read_to_string("data/day_05.txt")?);

    println!("Part 1: {}", part_1(&lines));
    println!("Part 2: {}", part_2(&lines));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn part_1_examples() {
        let lines = parse_input(INPUT);
        assert_eq!(part_1(&lines), 5);
    }

    #[test]
    fn part_2_examples() {
        let lines = parse_input(INPUT);
        assert_eq!(part_2(&lines), 12);
    }
}
