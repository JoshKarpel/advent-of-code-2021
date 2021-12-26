use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs::read_to_string;

use itertools::iproduct;

use crate::utils::SolverResult;

type Position = (isize, isize);
type Nodes = HashMap<Position, usize>;
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

// https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    position: Position,
    cost: usize,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

fn find_shortest_path(edges: &Edges, weights: &Weights) -> Option<usize> {
    let mut frontier = BinaryHeap::new();
    let mut shortest_paths = HashMap::new();

    let top_left = *edges.keys().min().unwrap();
    let bottom_right = *edges.keys().max().unwrap();

    frontier.push(State {
        position: top_left,
        cost: 0,
    });
    shortest_paths.insert(top_left, 0);

    while let Some(current) = frontier.pop() {
        if current.position == bottom_right {
            return Some(current.cost);
        }
        if let Some(neighbours) = edges.get(&current.position) {
            neighbours.iter().for_each(|&n| {
                let next = State {
                    position: n,
                    cost: current.cost + *weights.get(&(current.position, n)).unwrap(),
                };

                if next.cost < *shortest_paths.get(&n).unwrap_or(&usize::MAX) {
                    frontier.push(next);
                    shortest_paths.insert(n, next.cost);
                }
            });
        }
    }

    None
}

fn parse_nodes(input: &str) -> Nodes {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, w)| ((x as isize, y as isize), w.to_digit(10).unwrap() as usize))
        })
        .collect()
}

fn edges_and_weights(nodes: &Nodes) -> (Edges, Weights) {
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

fn expand(input: &Nodes) -> Nodes {
    let (mx, my) = input.keys().max().unwrap();
    let (mx, my) = (mx + 1, my + 1); // widths of the original chunk

    input
        .iter()
        .flat_map(|((x, y), w)| {
            iproduct!(0..=4isize, 0..=4isize).map(move |(xd, yd)| {
                let mut new_w = w + (xd + yd) as usize;
                if new_w >= 10 {
                    new_w -= 9
                }

                ((x + (xd * mx), y + (yd * my)), new_w)
            })
        })
        .collect()
}

pub fn solve() -> SolverResult {
    let nodes = parse_nodes(&read_to_string("data/day_15.txt")?);

    let (edges, weights) = edges_and_weights(&nodes);
    println!("Part 1: {}", find_shortest_path(&edges, &weights).unwrap());

    let (edges, weights) = edges_and_weights(&expand(&nodes));
    println!("Part 2: {}", find_shortest_path(&edges, &weights).unwrap());

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
        let (edges, weights) = edges_and_weights(&parse_nodes(INPUT));
        assert_eq!(find_shortest_path(&edges, &weights), Some(40));
    }

    #[test]
    fn part_2_examples() {
        let nodes = &parse_nodes(INPUT);
        let expanded = &expand(nodes);
        assert_eq!(nodes.len() * 25, expanded.len());

        let (edges, weights) = edges_and_weights(expanded);
        assert_eq!(find_shortest_path(&edges, &weights), Some(315));
    }
}
