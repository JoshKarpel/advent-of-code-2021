use crate::utils::SolverResult;
use std::collections::HashMap;
use std::fs::read_to_string;

type Algorithm = HashMap<usize, usize>;
type Position = (isize, isize);
type Image = HashMap<Position, usize>;

fn window(p: &Position) -> Vec<Position> {
    let (x, y) = *p;
    let x_m = x - 1;
    let x_p = x + 1;
    let y_m = y - 1;
    let y_p = y + 1;

    vec![
        (x_m, y_m),
        (x, y_m),
        (x_p, y_m),
        (x_m, y),
        *p,
        (x_p, y),
        (x_m, y_p),
        (x, y_p),
        (x_p, y_p),
    ]
}

fn apply(algorithm: &Algorithm, image: Image, unset: usize) -> Image {
    image
        .keys()
        .flat_map(window)
        .map(|position| {
            let bits: String = window(&position)
                .iter()
                .map(|p| image.get(p).unwrap_or(&unset).to_string())
                .collect();

            (
                position,
                *algorithm
                    .get(&usize::from_str_radix(&bits, 2).unwrap())
                    .unwrap(),
            )
        })
        .collect()
}

fn part_1(algorithm: &Algorithm, image: &Image) -> usize {
    (0..2)
        .fold(image.clone(), |image, step| {
            apply(algorithm, image, step % 2)
        })
        .values()
        .filter(|&v| v == &1)
        .count()
}

fn part_2(algorithm: &Algorithm, image: &Image) -> usize {
    (0..50)
        .fold(image.clone(), |image, step| {
            apply(algorithm, image, step % 2)
        })
        .values()
        .filter(|&v| v == &1)
        .count()
}

fn parse_input(input: &str) -> (Algorithm, Image) {
    let mut lines = input.lines();

    let algorithm = lines
        .next()
        .unwrap()
        .chars()
        .map(|c| if c == '#' { 1 } else { 0 })
        .enumerate()
        .collect();

    lines.next().unwrap();

    let image = lines
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as isize, y as isize), if c == '#' { 1 } else { 0 }))
        })
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
    #[test]
    fn part_1_examples() {}

    #[test]
    fn part_2_examples() {}
}
