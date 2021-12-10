use crate::utils::SolverResult;
use itertools::Itertools;
use ndarray::{s, Array2};
use std::collections::{BinaryHeap, HashMap};
use std::fs::read_to_string;

fn part_1(heights: &Array2<usize>) -> usize {
    heights
        .windows((3, 3))
        .into_iter()
        .filter_map(|window| {
            let center = window[(1, 1)];
            let edges = window
                .iter()
                .enumerate()
                .filter_map(|(idx, element)| (idx % 2 == 1).then_some(element))
                .cloned()
                .collect_vec();
            edges.iter().all(|&o| o > center).then_some(center + 1)
        })
        .sum()
}

fn part_2(heights: &Array2<usize>) -> usize {
    let mut new_basin: usize = 0;
    let mut basins = HashMap::new();

    heights.indexed_iter().for_each(|((x, y), h)| {
        if h >= &9 {
            return;
        }

        if basins.contains_key(&(x, y)) {
            return;
        }

        let mut candidates = vec![(x, y)];
        while !candidates.is_empty() {
            let (x, y) = candidates.pop().unwrap();

            if heights[[x, y]] >= 9 {
                continue;
            }

            basins.insert((x, y), new_basin);

            if let Some(new_x) = x.checked_sub(1) {
                candidates.push((new_x, y));
            }
            if let Some(new_y) = y.checked_sub(1) {
                candidates.push((x, new_y));
            }
            if (x + 1) < heights.shape()[0] {
                candidates.push((x + 1, y));
            }
            if (y + 1) < heights.shape()[1] {
                candidates.push((x, y + 1));
            }

            candidates.retain(|xy| !basins.contains_key(xy));
        }

        new_basin += 1;
    });

    basins
        .values()
        .counts()
        .values()
        .cloned()
        .collect::<BinaryHeap<usize>>()
        .drain_sorted()
        .take(3)
        .product()
}

fn parse_input(input: &str) -> Array2<usize> {
    let h = input.lines().next().unwrap().len();
    let w = input.lines().collect_vec().len();

    let heights = Array2::from_shape_vec(
        (w, h),
        input
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect(),
    )
    .unwrap();

    let mut outer: Array2<usize> = Array2::ones((w + 2, h + 2)) * 10;
    outer.slice_mut(s![1..=w, 1..=h]).assign(&heights);

    outer
}

pub fn solve() -> SolverResult {
    let heights = parse_input(&read_to_string("data/day_09.txt")?);

    println!("Part 1: {}", part_1(&heights));
    println!("Part 2: {}", part_2(&heights));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&parse_input(INPUT)), 15);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&parse_input(INPUT)), 1134);
    }
}
