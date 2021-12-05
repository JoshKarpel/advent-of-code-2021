use crate::utils::SolverResult;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
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
        if let Some(dx) =
            (self.end_x - self.start_x).checked_div(self.start_x.abs_diff(self.end_x) as isize)
        {
            dx
        } else {
            0
        }
    }

    fn dy(&self) -> isize {
        if let Some(dy) =
            (self.end_y - self.start_y).checked_div(self.start_y.abs_diff(self.end_y) as isize)
        {
            dy
        } else {
            0
        }
    }
}

fn count_overlaps(lines: &Vec<Line>) -> usize {
    let mut counts = HashMap::new();

    lines.iter().for_each(|line| {
        (0..=line.length()).for_each(|d| {
            let x = line.start_x + (line.dx() * d);
            let y = line.start_y + (line.dy() * d);

            let count = counts.entry((x, y)).or_insert(0);
            *count += 1;
        });
    });

    counts.iter().filter(|(_k, v)| **v >= 2).count()
}

fn part_1(lines: &Vec<Line>) -> usize {
    count_overlaps(
        &lines
            .iter()
            .cloned()
            .filter(|line| line.start_x == line.end_x || line.start_y == line.end_y)
            .collect(),
    )
}

fn part_2(lines: &Vec<Line>) -> usize {
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
