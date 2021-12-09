use crate::utils::SolverResult;
use itertools::Itertools;
use std::collections::{BTreeSet, HashMap};
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
//
// Digit -> # Segments
//     1 -> 2  (unique)
//     4 -> 4  (unique)
//     7 -> 3  (unique)
//     8 -> 7  (unique)
//
//     2 -> 5
//     3 -> 5
//     5 -> 5
//
//     0 -> 6
//     6 -> 6
//     9 -> 6

type Digit = BTreeSet<char>;

macro_rules! set {
    ( $( $x:expr ),* ) => {
        {
            let mut set = Digit::new();
            $(
                set.insert($x);
            )*
            set
        }
    };
}

lazy_static! {
    static ref ZERO: Digit = set!['a', 'b', 'c', 'e', 'f', 'g'];
    static ref ONE: Digit = set!['c', 'f'];
    static ref TWO: Digit = set!['a', 'c', 'd', 'e', 'g'];
    static ref THREE: Digit = set!['a', 'c', 'd', 'f', 'g'];
    static ref FOUR: Digit = set!['b', 'c', 'd', 'f'];
    static ref FIVE: Digit = set!['a', 'b', 'd', 'f', 'g'];
    static ref SIX: Digit = set!['a', 'b', 'd', 'e', 'f', 'g'];
    static ref SEVEN: Digit = set!['a', 'c', 'f'];
    static ref EIGHT: Digit = set!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    static ref NINE: Digit = set!['a', 'b', 'c', 'd', 'f', 'g'];
    static ref DIGIT_TO_CHAR: HashMap<&'static Digit, char> = {
        let mut values: HashMap<&'static Digit, char> = HashMap::new();

        values.insert(&ZERO, '0');
        values.insert(&ONE, '1');
        values.insert(&TWO, '2');
        values.insert(&THREE, '3');
        values.insert(&FOUR, '4');
        values.insert(&FIVE, '5');
        values.insert(&SIX, '6');
        values.insert(&SEVEN, '7');
        values.insert(&EIGHT, '8');
        values.insert(&NINE, '9');

        values
    };
}

const UNIQUE_SEGMENT_COUNTS: [usize; 4] = [2, 3, 4, 7];

#[derive(Debug, Clone)]
struct Display {
    signal_patterns: Vec<Digit>,
    output_value: Vec<Digit>,
}

impl Display {
    fn mapping(&self) -> HashMap<Digit, &'static Digit> {
        let mut mapping: HashMap<Digit, &'static Digit> = HashMap::new();

        let mut by_length: HashMap<usize, Vec<&Digit>> = HashMap::new();

        self.signal_patterns.iter().for_each(|pattern| {
            by_length
                .entry(pattern.len())
                .or_insert_with(Vec::new)
                .push(pattern);
        });

        // Some patterns must be for certain numbers based on length.
        let one = (*by_length.get(&2).unwrap().first().unwrap()).clone();
        let four = (*by_length.get(&4).unwrap().first().unwrap()).clone();
        let seven = (*by_length.get(&3).unwrap().first().unwrap()).clone();
        let eight = (*by_length.get(&7).unwrap().first().unwrap()).clone();
        mapping.insert(one.clone(), &ONE);
        mapping.insert(four.clone(), &FOUR);
        mapping.insert(seven.clone(), &SEVEN);
        mapping.insert(eight, &EIGHT);

        // The two characters in 7 that are also in 1 are 'c' and 'f'.
        let cf = &seven & &one;

        // The two characters in 4 that aren't in 7 are 'b' and 'd'.
        let bd = &four - &seven;

        // Look at 0, 6, and 9. 'c' is in 0 and 9, but 'f' is in all three.
        let zero_six_nine = by_length.get(&6).unwrap();

        let f = *cf
            .iter()
            .find_map(|x| {
                zero_six_nine
                    .iter()
                    .all(|digit| digit.contains(x))
                    .then_some(x)
            })
            .unwrap();

        // 'c' is the other one
        let c = *cf.difference(&set![f]).next().unwrap();

        // Now we can find 6 because it is the only digit among 0, 6, and 9 that does not include 'c'.
        let six = zero_six_nine
            .iter()
            .find_map(|digit| (!digit.contains(&c)).then_some(digit))
            .unwrap();
        mapping.insert((**six).clone(), &SIX);

        let zero_nine: Vec<&Digit> = zero_six_nine
            .iter()
            .filter(|digit| digit != &six)
            .cloned()
            .collect();

        // We can differentiate between 0 and 9 because 9 has 'd' but 0 does not,
        // but both have 'b', and we know 'bd'.
        let nine = zero_nine
            .iter()
            .find_map(|digit| bd.is_subset(digit).then_some(digit))
            .unwrap();
        mapping.insert((**nine).clone(), &NINE);

        // 0 is the last of the three.
        let zero = zero_nine.iter().find(|digit| digit != &nine).unwrap();
        mapping.insert((**zero).clone(), &ZERO);

        // Among 2, 3, and 5, the only one that does not have 'c' is 5.
        let two_three_five = by_length.get(&5).unwrap();

        let five = two_three_five
            .iter()
            .find_map(|digit| (!digit.contains(&c)).then_some(digit))
            .unwrap();
        mapping.insert((**five).clone(), &FIVE);

        let two_three: Vec<&Digit> = two_three_five
            .iter()
            .filter(|x| x != &five)
            .cloned()
            .collect();

        // Between 2 and 3, 2 does not have 'f'.
        let two = two_three
            .iter()
            .find_map(|digit| (!digit.contains(&f)).then_some(digit))
            .unwrap();
        mapping.insert((*two).clone(), &TWO);

        // And 3 is whatever is left!
        let three = two_three.iter().find(|x| x != &two).unwrap();
        mapping.insert((**three).clone(), &THREE);

        mapping
    }

    fn deduce_output(&self) -> usize {
        let mapping = self.mapping();

        self.output_value
            .iter()
            .map(|ov| mapping.get(ov).unwrap())
            .map(|digit| DIGIT_TO_CHAR.get(digit).unwrap())
            .join("")
            .parse()
            .unwrap()
    }
}

fn part_1(inputs: &[Display]) -> usize {
    inputs
        .iter()
        .flat_map(|input| input.output_value.iter())
        .filter(|&digit| UNIQUE_SEGMENT_COUNTS.contains(&digit.len()))
        .count()
}

fn part_2(inputs: &[Display]) -> usize {
    inputs.iter().map(Display::deduce_output).sum()
}

fn parse_input(input: &str) -> Vec<Display> {
    input
        .lines()
        .map(|line| {
            let (signal_patterns, output_value) = line.split_once(" | ").unwrap();
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

    const SMALL_INPUT: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    const BIG_INPUT: &str = "\
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
        assert_eq!(part_1(&parse_input(BIG_INPUT)), 26);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&parse_input(SMALL_INPUT)), 5353);
        assert_eq!(part_2(&parse_input(BIG_INPUT)), 61229);
    }
}
