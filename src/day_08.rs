use crate::utils::SolverResult;
use std::collections::HashSet;
use std::fs::read_to_string;

//   0:      1:      2:      3:      4:
//  aaaa    ....    aaaa    aaaa    ....
// b    c  .    c  .    c  .    c  b    c
// b    c  .    c  .    c  .    c  b    c
//  ....    ....    dddd    dddd    dddd
// e    f  .    f  e    .  .    f  .    f
// e    f  .    f  e    .  .    f  .    f
//  gggg    ....    gggg    gggg    ....
//
//   5:      6:      7:      8:      9:
//  aaaa    aaaa    aaaa    aaaa    aaaa
// b    .  b    .  .    c  b    c  b    c
// b    .  b    .  .    c  b    c  b    c
//  dddd    dddd    ....    dddd    dddd
// .    f  e    f  .    f  e    f  .    f
// .    f  e    f  .    f  e    f  .    f
//  gggg    gggg    ....    gggg    gggg

// Digit -> # Segments
//     0 -> 6
//     1 -> 2  (unique)
//     2 -> 5
//     3 -> 5
//     4 -> 4  (unique)
//     5 -> 5
//     6 -> 6
//     7 -> 3  (unique)
//     8 -> 7  (unique)
//     9 -> 6

const UNIQUE_SEGMENT_COUNTS: [usize; 4] = [2, 3, 4, 7];

type Digit = HashSet<char>;
struct Display {
    signal_patterns: Vec<Digit>,
    output_value: Vec<Digit>,
}

fn part_1(inputs: &[Display]) -> usize {
    inputs
        .iter()
        .flat_map(|input| input.output_value.iter())
        .filter(|&digit| UNIQUE_SEGMENT_COUNTS.contains(&digit.len()))
        .count()
}

fn part_2(_inputs: &[Display]) -> usize {
    0
}

fn parse_input(input: &str) -> Vec<Display> {
    input
        .lines()
        .map(|line| {
            let (signal_patterns, output_value) = line.split_once('|').unwrap();
            Display {
                signal_patterns: signal_patterns
                    .split(' ')
                    .map(|chars| chars.chars().collect())
                    .collect(),
                output_value: output_value
                    .split(' ')
                    .map(|chars| chars.chars().collect())
                    .collect(),
            }
        })
        .collect()
}

pub fn solve() -> SolverResult {
    let inputs = parse_input(&read_to_string("data/day_08.txt")?);

    println!("Part 1: {}", part_1(&inputs));
    println!("Part 2: {}", part_2(&inputs));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&parse_input(INPUT)), 26);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&parse_input(INPUT)), 61229);
    }
}
