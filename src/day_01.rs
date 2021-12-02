use std::fs::read_to_string;

fn part_1(depths: &[usize]) -> usize {
    depths
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

fn part_2(depths: &[usize]) -> usize {
    depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|window| window[0] < window[1])
        .count()
}

pub fn solve() {
    let depths: Vec<usize> = read_to_string("data/day_01.txt")
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("Part 1: {}", part_1(&depths));
    println!("Part 2: {}", part_2(&depths));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_examples() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(part_1(&depths), 7)
    }

    #[test]
    fn part_2_examples() {
        let depths = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        assert_eq!(part_2(&depths), 5)
    }
}
