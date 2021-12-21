use crate::utils::SolverResult;
use std::collections::HashMap;
use std::fs::read_to_string;

type Algorithm = HashMap<usize, char>;
type Image = HashMap<(usize, usize), char>;

fn apply(_algorithm: &Algorithm, image: Image) -> Image {
    image
}

fn part_1(algorithm: &Algorithm, image: &Image) -> usize {
    (0..2)
        .fold(image.clone(), |image, _step| apply(algorithm, image))
        .values()
        .filter(|&v| v == &'#')
        .count()
}

fn part_2(_algorithm: &Algorithm, _image: &Image) -> usize {
    0
}

fn parse_input(input: &str) -> (Algorithm, Image) {
    let mut lines = input.lines();

    let algorithm = lines.next().unwrap().chars().enumerate().collect();

    lines.next().unwrap();

    let image = lines
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)))
        .collect();

    (algorithm, image)
}

pub fn solve() -> SolverResult {
    let (algorithm, image) = parse_input(&read_to_string("data/day_20.txt")?);

    println!("Part 1: {}", part_1(&algorithm, &image));
    println!("Part 2: {}", part_2(&algorithm, &image));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn part_1_examples() {
        let (algorithm, image) = parse_input(INPUT);
        assert_eq!(part_1(&algorithm, &image), 35);
    }

    #[test]
    fn part_2_examples() {}
}
