use crate::utils::SolverResult;
use ndarray::Array2;
use std::collections::HashSet;
use std::fs::read_to_string;
use std::ops::ControlFlow::{Break, Continue};

type Board = Array2<usize>;

fn wins(board: &Board, draws: &HashSet<&usize>, last_draw: &usize) -> Option<usize> {
    if board
        .rows()
        .into_iter()
        .chain(board.columns().into_iter())
        .any(|x| x.into_iter().all(|e| draws.contains(e)))
    {
        Some(score(board, draws, last_draw))
    } else {
        None
    }
}

fn score(board: &Board, draws: &HashSet<&usize>, last_draw: &usize) -> usize {
    last_draw * board.iter().filter(|x| !draws.contains(x)).sum::<usize>()
}

fn part_1(draws: &[usize], boards: &[Board]) -> usize {
    let mut draw_set = HashSet::new();

    draws
        .iter()
        .find_map(|last_draw| {
            draw_set.insert(last_draw);
            boards.iter().find_map(|b| wins(b, &draw_set, last_draw))
        })
        .unwrap()
}

fn part_2(draws: &[usize], boards: &[Board]) -> usize {
    draws
        .iter()
        .try_fold(
            (boards.to_vec(), HashSet::new()),
            |(boards, mut draw_set), last_draw| {
                draw_set.insert(last_draw);

                if boards.len() == 1 {
                    if let Some(score) = wins(&boards[0], &draw_set, last_draw) {
                        Break(score)
                    } else {
                        Continue((boards, draw_set))
                    }
                } else {
                    Continue((
                        boards
                            .iter()
                            .cloned()
                            .filter(|b| wins(b, &draw_set, last_draw).is_none())
                            .collect(),
                        draw_set,
                    ))
                }
            },
        )
        .break_value()
        .unwrap()
}

pub fn solve() -> SolverResult {
    let (draws, boards): (Vec<usize>, Vec<Board>) = read_to_string("data/day_04.txt")?
        .split_once("\n")
        .map(|(draws, boards)| {
            (
                draws.split(',').map(|d| d.parse().unwrap()).collect(),
                boards
                    .split("\n\n")
                    .map(|chunk| {
                        chunk
                            .split_whitespace()
                            .map(|c| c.parse().unwrap())
                            .collect()
                    })
                    .map(|board| Array2::from_shape_vec([5, 5], board).unwrap())
                    .collect(),
            )
        })
        .unwrap();

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
    const BOARDS: [[[usize; 5]; 5]; 3] = [
        [
            [22, 13, 17, 11, 0],
            [8, 2, 23, 4, 24],
            [21, 9, 14, 16, 7],
            [6, 10, 3, 18, 5],
            [1, 12, 20, 15, 19],
        ],
        [
            [3, 15, 0, 2, 22],
            [9, 18, 13, 17, 5],
            [19, 8, 7, 25, 23],
            [20, 11, 10, 24, 4],
            [14, 21, 16, 12, 6],
        ],
        [
            [14, 21, 17, 24, 4],
            [10, 16, 15, 9, 19],
            [18, 8, 23, 26, 20],
            [22, 11, 13, 6, 5],
            [2, 0, 12, 3, 7],
        ],
    ];

    #[test]
    fn part_1_examples() {
        let boards: Vec<Board> = BOARDS.iter().map(|b| Array2::from(b.to_vec())).collect();
        assert_eq!(part_1(&DRAWS, &boards), 4512)
    }

    #[test]
    fn part_2_examples() {
        let boards: Vec<Board> = BOARDS.iter().map(|b| Array2::from(b.to_vec())).collect();
        assert_eq!(part_2(&DRAWS, &boards), 1924)
    }
}
