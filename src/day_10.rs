use crate::utils::SolverResult;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;

lazy_static! {
    static ref OPEN_TO_CLOSE: HashMap<char, char> = {
        let mut oc = HashMap::new();

        oc.insert('(', ')');
        oc.insert('[', ']');
        oc.insert('{', '}');
        oc.insert('<', '>');

        oc
    };
    static ref OPEN: HashSet<char> = HashSet::from_iter(OPEN_TO_CLOSE.keys().cloned());
    static ref CLOSE: HashSet<char> = HashSet::from_iter(OPEN_TO_CLOSE.values().cloned());
    static ref SCORES: HashMap<char, usize> = {
        let mut scores = HashMap::new();

        scores.insert(')', 3);
        scores.insert(']', 57);
        scores.insert('}', 1197);
        scores.insert('>', 25137);

        scores
    };
    static ref OTHER_SCORES: HashMap<char, usize> = {
        let mut scores = HashMap::new();

        scores.insert(')', 1);
        scores.insert(']', 2);
        scores.insert('}', 3);
        scores.insert('>', 4);

        scores
    };
}

fn parse(line: &str) -> Result<Vec<char>, char> {
    let mut stack = Vec::new();
    let error = line.chars().find_map(|char| {
        if OPEN.contains(&char) {
            stack.push(char);
            None
        } else if let Some(&closer) = stack
            .last()
            .and_then(|last_open| OPEN_TO_CLOSE.get(last_open))
        {
            if char == closer {
                // right closer
                stack.pop();
                None
            } else {
                // wrong closer
                Some(char)
            }
        } else {
            // empty stack, or the character wasn't a closer
            Some(char)
        }
    });

    if let Some(e) = error {
        Err(e)
    } else {
        Ok(stack)
    }
}

fn check(line: &str) -> Option<char> {
    if let Err(e) = parse(line) {
        Some(e)
    } else {
        None
    }
}

fn complete(line: &str) -> Option<String> {
    parse(line)
        .map(|stack| {
            stack
                .iter()
                .rev()
                .map(|open| OPEN_TO_CLOSE.get(open).unwrap())
                .collect()
        })
        .ok()
}

fn part_1(lines: &[&str]) -> usize {
    lines
        .iter()
        .filter_map(|line| check(line))
        .map(|c| SCORES.get(&c).unwrap())
        .sum()
}

fn part_2(lines: &[&str]) -> usize {
    let scores = lines
        .iter()
        .filter_map(|line| complete(line))
        .map(|completion| {
            completion.chars().fold(0, |score, closer| {
                (score * 5) + OTHER_SCORES.get(&closer).unwrap()
            })
        })
        .sorted()
        .collect_vec();

    scores[(scores.len() / 2)]
}

pub fn solve() -> SolverResult {
    let input = read_to_string("data/day_10.txt")?;
    let lines: Vec<&str> = input.lines().collect();

    println!("Part 1: {}", part_1(&lines));
    println!("Part 2: {}", part_2(&lines));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::Itertools;

    const LINES: &str = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn part_1_examples() {
        assert_eq!(check("()"), None);
        assert_eq!(check("([])"), None);
        assert_eq!(check("{()()()}"), None);
        assert_eq!(check("<([{}])>"), None);
        assert_eq!(check("[<>({}){}[([])<>]]"), None);
        assert_eq!(check("(((((((((())))))))))"), None);

        assert_eq!(check("(]"), Some(']'));
        assert_eq!(check("{()()()>"), Some('>'));
        assert_eq!(check("(((()))}"), Some('}'));
        assert_eq!(check("<([]){()}[{}])"), Some(')'));

        assert_eq!(part_1(&LINES.lines().collect_vec()), 26397);
    }

    #[test]
    fn part_2_examples() {
        assert_eq!(part_2(&LINES.lines().collect_vec()), 288957);
    }
}
