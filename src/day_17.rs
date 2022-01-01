use crate::utils::SolverResult;
use regex::Regex;
use std::fs::read_to_string;
use std::iter;
use std::ops::ControlFlow::{Break, Continue};
use std::ops::RangeInclusive;

type Point = (isize, isize);
type Width = RangeInclusive<isize>;
type Target = (Width, Width);

fn path(v_x_initial: isize, v_y_initial: isize, target: &Target) -> Vec<Point> {
    let (x_target, y_target) = target;

    (0..=v_x_initial)
        .rev()
        .chain(iter::repeat(0))
        .zip((isize::MIN..=v_y_initial).rev())
        .try_fold(vec![(0, 0)], |mut path, (v_x, v_y)| {
            let &(x, y) = path.last().unwrap();
            if x > *x_target.end() || (y < *y_target.end().min(y_target.start()) && v_y < 0) {
                Break(path)
            } else {
                path.push((x + v_x, y + v_y));
                Continue(path)
            }
        })
        .break_value()
        .unwrap()
}

fn paths(target: &Target) -> Vec<Vec<Point>> {
    let (x_target, y_target) = target;

    (0..=*x_target.end())
        .flat_map(|v_x_initial| {
            // There's probably a good way to bound v_y_initial but this is plenty fast
            (-1000..=1000).filter_map(move |v_y_initial| {
                let path = path(v_x_initial, v_y_initial, target);
                path.iter()
                    .any(|(x, y)| x_target.contains(x) && y_target.contains(y))
                    .then_some(path)
            })
        })
        .collect()
}

fn part_1(target: &Target) -> isize {
    paths(target)
        .iter()
        .map(|path| *path.iter().map(|(_, y)| y).max().unwrap())
        .max()
        .unwrap()
}

fn part_2(target: &Target) -> usize {
    paths(target).len()
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
}

fn parse_input(input: &str) -> Target {
    let captures = RE.captures(input).unwrap();

    let lower_x = captures[1].parse().unwrap();
    let upper_x = captures[2].parse().unwrap();
    let lower_y = captures[3].parse().unwrap();
    let upper_y = captures[4].parse().unwrap();

    (lower_x..=upper_x, lower_y..=upper_y)
}

pub fn solve() -> SolverResult {
    let target = parse_input(&read_to_string("data/day_17.txt")?);

    println!("Part 1: {}", part_1(&target));
    println!("Part 2: {}", part_2(&target));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part_1_examples() {
        let target = parse_input(INPUT);
        assert_eq!(target, (20..=30, -10..=-5));

        assert_eq!(part_1(&target), 45);
    }

    #[test]
    fn part_2_examples() {
        let target = parse_input(INPUT);

        assert_eq!(part_2(&target), 112);
    }
}
