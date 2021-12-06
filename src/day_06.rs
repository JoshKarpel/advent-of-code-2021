use crate::utils::SolverResult;
use itertools::Itertools;
use std::fs::read_to_string;

fn day(fish: &[u8]) -> Vec<u8> {
    fish.iter().fold(Vec::new(), |mut new_fish, f| {
        match f {
            0 => {
                new_fish.push(6);
                new_fish.push(8);
            }
            fish => new_fish.push(fish - 1),
        }
        new_fish
    })
}

fn part_1(fish: &[u8]) -> usize {
    (0..80).fold(fish.to_vec(), |fish, _f| day(&fish)).len()
}

fn part_2(fish: &[u8]) -> usize {
    (0..256).fold(fish.to_vec(), |fish, _f| day(&fish)).len()
}

pub fn solve() -> SolverResult {
    let fish: Vec<u8> = read_to_string("data/day_06.txt")?
        .trim()
        .split(',')
        .map(&str::parse)
        .try_collect()?;

    println!("Part 1: {}", part_1(&fish));
    println!("Part 2: {}", part_2(&fish));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::day_06::part_1;

    const FISH: [u8; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&FISH), 5934);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_1(&FISH), 26984457539);
    }
}
