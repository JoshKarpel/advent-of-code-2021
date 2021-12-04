use crate::utils::SolverResult;
use itertools::Itertools;
use std::fs::read_to_string;

#[derive(Clone)]
struct Board(Vec<usize>);

impl Board {
    fn row(&self, n: usize) -> [usize; 5] {
        let start = n * 5;
        [
            self.0[start],
            self.0[start + 1],
            self.0[start + 2],
            self.0[start + 3],
            self.0[start + 4],
        ]
    }

    fn col(&self, n: usize) -> [usize; 5] {
        [
            self.0[n],
            self.0[n + 5],
            self.0[n + 10],
            self.0[n + 15],
            self.0[n + 20],
        ]
    }

    fn wins(&self, draws: &[usize]) -> Option<usize> {
        let wins = (0..5).any(|i| {
            self.row(i).iter().all(|r| draws.contains(r))
                || self.col(i).iter().all(|c| draws.contains(c))
        });

        if wins {
            Some(self.score(draws))
        } else {
            None
        }
    }

    fn score(&self, draws: &[usize]) -> usize {
        self.0.iter().filter(|x| !draws.contains(x)).sum::<usize>() * draws.last().unwrap()
    }
}

fn part_1(draws: &[usize], boards: &[Board]) -> usize {
    (1..draws.len())
        .find_map(|d| boards.iter().find_map(|b| b.wins(&draws[..d])))
        .unwrap()
}

fn part_2(draws: &[usize], boards: &[Board]) -> usize {
    let mut boards = boards.to_vec();
    (1..draws.len())
        .find_map(|d| {
            let current_draws = &draws[..d];

            if boards.len() == 1 {
                return boards[0].wins(current_draws);
            }

            boards.retain(|b| b.wins(current_draws).is_none());

            None
        })
        .unwrap()
}

pub fn solve() -> SolverResult {
    let input = read_to_string("data/day_04.txt")?;
    let draws: Vec<usize> = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(&str::parse)
        .try_collect()?;
    let boards: Vec<Board> = input
        .lines()
        .skip(1)
        .join("\n")
        .split("\n\n")
        .map(|b| {
            println!("{}", b);
            Board(b.split_whitespace().map(|c| c.parse().unwrap()).collect())
        })
        .collect();

    println!("Part 1: {}", part_1(&draws, &boards));
    println!("Part 2: {}", part_2(&draws, &boards));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DRAWS: [usize; 27] = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    #[test]
    fn part_1_examples() {
        let boards: [Board; 3] = [
            Board(vec![
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19,
            ]),
            Board(vec![
                3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16,
                12, 6,
            ]),
            Board(vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ]),
        ];
        assert_eq!(part_1(&DRAWS, &boards), 4512)
    }

    #[test]
    fn part_2_examples() {
        let boards: [Board; 3] = [
            Board(vec![
                22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20,
                15, 19,
            ]),
            Board(vec![
                3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16,
                12, 6,
            ]),
            Board(vec![
                14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0,
                12, 3, 7,
            ]),
        ];
        assert_eq!(part_2(&DRAWS, &boards), 1924)
    }
}
