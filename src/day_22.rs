use crate::utils::SolverResult;

use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    On,
    Off,
}

type Bounds = (isize, isize);

#[derive(Debug, Copy, Clone)]
struct Cuboid {
    x: Bounds,
    y: Bounds,
    z: Bounds,
}

#[derive(Debug, Copy, Clone)]
struct Step {
    instruction: Instruction,
    cuboid: Cuboid,
}

type Grid = HashSet<(isize, isize, isize)>;

fn part_1(_steps: &[Step]) -> usize {
    0
}

fn part_2(_steps: &[Step]) -> usize {
    0
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"(on|off) x=(-?\d+..-?\d+),y=(-?\d+..-?\d+),z=(-?\d+..-?\d+)").unwrap();
}

fn parse_range(range: &str) -> Bounds {
    let (lower, upper) = range.split_once("..").unwrap();
    (lower.parse().unwrap(), upper.parse().unwrap())
}

fn parse_input(input: &str) -> Vec<Step> {
    input
        .lines()
        .map(|line| {
            let captures = RE.captures(line).unwrap();

            let x = parse_range(&captures[2]);
            let y = parse_range(&captures[3]);
            let z = parse_range(&captures[4]);

            Step {
                instruction: match &captures[1] {
                    "on" => Instruction::On,
                    "off" => Instruction::Off,
                    _ => unreachable!(),
                },
                cuboid: Cuboid { x, y, z },
            }
        })
        .collect()
}

pub fn solve() -> SolverResult {
    let steps = parse_input(&read_to_string("data/day_22.txt")?);

    println!("Part 1: {}", part_1(&steps));
    println!("Part 2: {}", part_2(&steps));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_INPUT: &str = "\
on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";

    const LARGE_INPUT: &str = "\
on x=-20..26,y=-36..17,z=-47..7
on x=-20..33,y=-21..23,z=-26..28
on x=-22..28,y=-29..23,z=-38..16
on x=-46..7,y=-6..46,z=-50..-1
on x=-49..1,y=-3..46,z=-24..28
on x=2..47,y=-22..22,z=-23..27
on x=-27..23,y=-28..26,z=-21..29
on x=-39..5,y=-6..47,z=-3..44
on x=-30..21,y=-8..43,z=-13..34
on x=-22..26,y=-27..20,z=-29..19
off x=-48..-32,y=26..41,z=-47..-37
on x=-12..35,y=6..50,z=-50..-2
off x=-48..-32,y=-32..-16,z=-15..-5
on x=-18..26,y=-33..15,z=-7..46
off x=-40..-22,y=-38..-28,z=23..41
on x=-16..35,y=-41..10,z=-47..6
off x=-32..-23,y=11..30,z=-14..3
on x=-49..-5,y=-3..45,z=-29..18
off x=18..30,y=-20..-8,z=-3..13
on x=-41..9,y=-7..43,z=-33..15
on x=-54112..-39298,y=-85059..-49293,z=-27449..7877
on x=967..23432,y=45373..81175,z=27513..53682";

    #[test]
    fn part_1_examples() {
        assert_eq!(part_1(&parse_input(SMALL_INPUT)), 39);
        assert_eq!(part_1(&parse_input(LARGE_INPUT)), 590784);
    }

    #[test]
    fn part_2_examples() {}
}
