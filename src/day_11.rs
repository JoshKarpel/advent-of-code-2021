use crate::utils::SolverResult;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

type Position = (isize, isize);
type Levels = HashMap<Position, u8>;

fn neighbours(p: &Position) -> Vec<Position> {
    let (x, y) = p;
    let x_m = x - 1;
    let x_p = x + 1;
    let y_m = y - 1;
    let y_p = y + 1;

    vec![
        (x_m, y_m),
        (x_m, *y),
        (x_m, y_p),
        (*x, y_m),
        (*x, y_p),
        (x_p, y_m),
        (x_p, *y),
        (x_p, y_p),
    ]
}

fn find_flashers(levels: &mut Levels) -> HashSet<Position> {
    levels.values_mut().for_each(|level| *level += 1);

    let mut flashed = HashSet::new();
    while levels
        .iter()
        .any(|(p, &level)| level > 9 && !flashed.contains(p))
    {
        levels.clone().iter().for_each(|(p, &level)| {
            if level <= 9 || flashed.contains(p) {
                return;
            }

            flashed.insert(*p);

            neighbours(p).iter().for_each(|neighbour| {
                if let Some(l) = levels.get_mut(neighbour) {
                    *l += 1
                }
            })
        })
    }

    flashed.iter().for_each(|p| {
        levels.insert(*p, 0);
    });

    flashed
}

fn part_1(levels: &Levels) -> usize {
    (0..100)
        .scan(levels.clone(), |levels, _| {
            let flashers = find_flashers(levels);

            Some(flashers.len())
        })
        .sum()
}

fn part_2(levels: &Levels) -> isize {
    (1..)
        .scan(levels.clone(), |levels, step| {
            Some((find_flashers(levels), step))
        })
        .find_map(|(flashers, step)| (flashers.len() == levels.len()).then_some(step))
        .unwrap()
}

fn parse_input(input: &str) -> Levels {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, level)| (x, y, level.to_digit(10).unwrap() as u8))
        })
        .fold(HashMap::new(), move |mut levels, (x, y, level)| {
            levels.insert((x as isize, y as isize), level);
            levels
        })
}

pub fn solve() -> SolverResult {
    let levels = parse_input(&read_to_string("data/day_11.txt")?);

    println!("Part 1: {}", part_1(&levels));
    println!("Part 2: {}", part_2(&levels));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&parse_input(INPUT)), 1656);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&parse_input(INPUT)), 195);
    }
}
