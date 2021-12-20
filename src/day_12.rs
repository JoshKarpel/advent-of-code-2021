use crate::utils::SolverResult;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

type Node = String;
type Edges = HashMap<Node, Vec<Node>>;
type Path = Vec<Node>;

fn is_small(node: &str) -> bool {
    node.to_ascii_lowercase() == node
}

fn find_paths(edges: &Edges, path: Path, is_path_good: fn(path: &Path) -> bool) -> Vec<Path> {
    if let Some(nexts) = edges.get(path.last().unwrap()) {
        nexts
            .iter()
            .flat_map(|next| {
                if next == "end" {
                    let mut new_path: Path = path.clone();
                    new_path.push(next.clone());
                    vec![new_path]
                } else {
                    let mut new_path: Path = path.clone();
                    new_path.push(next.clone());
                    if is_path_good(&new_path) {
                        find_paths(edges, new_path, is_path_good)
                    } else {
                        vec![]
                    }
                }
            })
            .collect()
    } else {
        vec![]
    }
}

fn part_1(edges: &Edges) -> usize {
    find_paths(edges, vec!["start".to_owned()], |path| {
        path.iter()
            .filter(|node| is_small(node))
            .counts()
            .values()
            .all(|count| count == &1)
    })
    .len()
}

fn part_2(edges: &Edges) -> usize {
    find_paths(edges, vec!["start".to_owned()], |path| {
        let node_visit_counts = path.iter().filter(|node| is_small(node)).counts();

        (path.last().unwrap() != "start")
            && (node_visit_counts.values().max().unwrap() <= &2)
            && (node_visit_counts
                .values()
                .filter(|&count| count > &1)
                .count()
                <= 1)
    })
    .len()
}

fn parse_input(input: &str) -> Edges {
    input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .fold(HashMap::new(), |mut edges, (start, end)| {
            edges
                .entry(start.to_owned())
                .or_insert_with(Vec::new)
                .push(end.to_owned());
            edges
                .entry(end.to_owned())
                .or_insert_with(Vec::new)
                .push(start.to_owned());
            edges
        })
}

pub fn solve() -> SolverResult {
    let edges = parse_input(&read_to_string("data/day_12.txt")?);

    println!("Part 1: {}", part_1(&edges));
    println!("Part 2: {}", part_2(&edges));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";
    const MEDIUM_INPUT: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
    const LARGE_INPUT: &str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&parse_input(SMALL_INPUT)), 10);
        assert_eq!(part_1(&parse_input(MEDIUM_INPUT)), 19);
        assert_eq!(part_1(&parse_input(LARGE_INPUT)), 226);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&parse_input(SMALL_INPUT)), 36);
        assert_eq!(part_2(&parse_input(MEDIUM_INPUT)), 103);
        assert_eq!(part_2(&parse_input(LARGE_INPUT)), 3509);
    }
}
