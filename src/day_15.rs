use crate::utils::SolverResult;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

type Position = (isize, isize);
type Edges = HashMap<Position, Vec<Position>>;
type Weights = HashMap<(Position, Position), usize>;

fn neighbours(p: &Position) -> Vec<Position> {
    let (x, y) = p;
    let x_m = x - 1;
    let x_p = x + 1;
    let y_m = y - 1;
    let y_p = y + 1;

    vec![(x_m, *y), (*x, y_m), (*x, y_p), (x_p, *y)]
}

fn part_1(edges: &Edges, weights: &Weights) -> usize {
    let mut unvisited: HashSet<&Position> = edges.keys().collect();
    let mut shortest_paths: HashMap<Position, usize> = HashMap::new();

    let mut current = **unvisited.iter().min().unwrap();
    let destination = **unvisited.iter().max().unwrap();

    shortest_paths.insert(current, 0);

    loop {
        let current_weight = *shortest_paths.get(&current).unwrap();
        if let Some(neighbours) = edges.get(&current) {
            neighbours.iter().for_each(|&n| {
                let mut w = current_weight + *weights.get(&(current, n)).unwrap();
                let entry = shortest_paths.entry(n).or_insert(w);
                *entry = *entry.min(&mut w);
            });
        }

        unvisited.remove(&current);

        if !unvisited.contains(&destination) {
            break;
        }

        current = **unvisited
            .iter()
            .min_by_key(|u| shortest_paths.get(u).unwrap_or(&usize::MAX))
            .unwrap();
    }

    *shortest_paths.get(&destination).unwrap()
}

fn part_2(_edges: &Edges, _weights: &Weights) -> usize {
    0
}

fn parse_input(input: &str) -> (Edges, Weights) {
    let nodes: HashMap<Position, usize> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, w)| ((x as isize, y as isize), w.to_digit(10).unwrap() as usize))
        })
        .collect();

    nodes.iter().fold(
        (Edges::new(), Weights::new()),
        |(mut edges, mut weights), (&p, _)| {
            neighbours(&p).iter().for_each(|&n| {
                if let Some(&w) = nodes.get(&n) {
                    weights.insert((p, n), w);
                    edges.entry(p).or_insert_with(Vec::new).push(n);
                }
            });

            (edges, weights)
        },
    )
}

pub fn solve() -> SolverResult {
    let (edges, weights) = parse_input(&read_to_string("data/day_15.txt")?);

    println!("Part 1: {}", part_1(&edges, &weights));
    println!("Part 2: {}", part_2(&edges, &weights));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn part_1_examples() {
        let (edges, weights) = parse_input(INPUT);
        assert_eq!(part_1(&edges, &weights), 40);
    }

    #[test]
    fn part_2_examples() {}
}
