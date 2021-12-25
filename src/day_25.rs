use crate::utils::SolverResult;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::ControlFlow::{Break, Continue};

type Herd = HashSet<(usize, usize)>;
type Herds = (Herd, Herd);
type Bounds = (usize, usize);

fn take_step(herds: &Herds, bounds: &Bounds) -> Herds {
    let (x_bound, y_bound) = bounds;
    let (east, south) = herds;
    let (mut new_east, mut new_south) = (Herd::new(), Herd::new());

    east.iter().for_each(|&(x, y)| {
        let target = ((x + 1) % x_bound, y);
        if east.contains(&target) || south.contains(&target) {
            new_east.insert((x, y));
        } else {
            new_east.insert(target);
        }
    });

    south.iter().for_each(|&(x, y)| {
        let target = (x, (y + 1) % y_bound);
        if new_east.contains(&target) || south.contains(&target) {
            new_south.insert((x, y));
        } else {
            new_south.insert(target);
        }
    });

    (new_east, new_south)
}

fn part_1(herds: &Herds, bounds: &Bounds) -> usize {
    (1..)
        .try_fold(herds.clone(), |herds, step| {
            let new_herds = take_step(&herds, bounds);
            if new_herds == herds {
                Break(step)
            } else {
                Continue(new_herds)
            }
        })
        .break_value()
        .unwrap()
}

fn parse_input(input: &str) -> (Herds, Bounds) {
    let herds = input.lines().enumerate().fold(
        (Herd::new(), Herd::new()),
        |(mut east, mut south), (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                match c {
                    '>' => east.insert((x, y)),
                    'v' => south.insert((x, y)),
                    '.' => true,
                    _ => unreachable!(),
                };
            });
            (east, south)
        },
    );

    (
        herds,
        (
            input.lines().next().unwrap().chars().count(),
            input.lines().count(),
        ),
    )
}

pub fn solve() -> SolverResult {
    let (herds, bounds) = parse_input(&read_to_string("data/day_25.txt")?);

    println!("Part 1: {}", part_1(&herds, &bounds));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT: &str = "\
..........
.>v....v..
.......>..
..........";

    const INPUT: &str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn part_1_examples() {
        let (herds, bounds) = parse_input(SMALL_INPUT);
        assert_eq!(
            herds,
            (
                HashSet::from_iter([(1, 1), (7, 2)].into_iter()),
                HashSet::from_iter([(2, 1), (7, 1)].into_iter())
            )
        );
        assert_eq!(
            take_step(&herds, &bounds),
            (
                HashSet::from_iter([(1, 1), (8, 2)].into_iter()),
                HashSet::from_iter([(2, 2), (7, 2)].into_iter())
            )
        );

        let (herds, bounds) = parse_input(INPUT);
        assert_eq!(part_1(&herds, &bounds), 58)
    }
}
