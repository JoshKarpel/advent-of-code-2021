use crate::utils::SolverResult;
use itertools::{iproduct, Itertools};
use std::collections::HashMap;
use std::fs::read_to_string;

type Positions = [usize; 2];
type Scores = [usize; 2];

fn part_1(positions: &Positions) -> usize {
    let mut positions = *positions;

    let mut rolls = (1..=1000).cycle();
    let mut roll_count = 0;

    let mut scores = [0, 0];

    'outer: loop {
        for (position, score) in positions.iter_mut().zip(scores.iter_mut()) {
            let dist: usize = (0..3).map(|_| rolls.next().unwrap()).sum();
            roll_count += 3;
            *position = (*position + dist) % 10;
            *score += if *position == 0 { 10 } else { *position }; // 0 is 10, otherwise they match

            if *score >= 1000 {
                break 'outer;
            }
        }
    }

    roll_count * scores.iter().min().unwrap()
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Game {
    positions: Positions,
    scores: Scores,
    turn: usize,
}

lazy_static! {
    // Precompute each possible distance and how many times it appears
    static ref DISTANCES: HashMap<usize, usize> = iproduct!(1..=3, 1..=3, 1..=3)
        .map(|rolls| rolls.0 + rolls.1 + rolls.2)
        .counts();
}

type Winners = HashMap<Game, [usize; 2]>;

fn play(game: Game, winners: &mut Winners) -> [usize; 2] {
    if let Some(win_counts) = winners.get(&game) {
        *win_counts
    } else {
        let mut win_counts = [0, 0];

        // Recurse down into each new game
        DISTANCES.iter().for_each(|(distance, count)| {
            let position = (game.positions[game.turn] + distance) % 10;
            let score = game.scores[game.turn] + if position == 0 { 10 } else { position }; // 0 is 10, otherwise they match

            if score >= 21 {
                win_counts[game.turn] += count;
            } else {
                let mut new_game = game.clone();

                new_game.positions[game.turn] = position;
                new_game.scores[game.turn] = score;
                new_game.turn = match game.turn {
                    0 => 1,
                    1 => 0,
                    _ => unreachable!(),
                };

                let new_wins = play(new_game, winners);
                win_counts[0] += count * new_wins[0];
                win_counts[1] += count * new_wins[1];
            }
        });

        winners.insert(game, win_counts);

        win_counts
    }
}

fn part_2(positions: &Positions) -> usize {
    let game = Game {
        positions: *positions,
        scores: [0, 0],
        turn: 0,
    };

    // There are ~27^21 games to play, but only
    // 10^2 * 21^2 * 2 game states (positions * scores * turns),
    // so we'll have lots of hits in a game state -> win counts cache.
    let mut winners: Winners = HashMap::new();

    *play(game, &mut winners).iter().max().unwrap()
}

fn parse_input(input: &str) -> Positions {
    input
        .lines()
        .map(|line| line.chars().last().unwrap().to_digit(10).unwrap() as usize)
        .collect::<Vec<usize>>()
        .try_into()
        .unwrap()
}

pub fn solve() -> SolverResult {
    let positions = parse_input(&read_to_string("data/day_21.txt")?);

    println!("Part 1: {}", part_1(&positions));
    println!("Part 2: {}", part_2(&positions));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&parse_input(INPUT)), 739785);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&parse_input(INPUT)), 444356092776315);
    }
}
