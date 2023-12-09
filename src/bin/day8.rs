use std::collections::BTreeMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace1, newline},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};
use num::integer;

fn main() {
    let input = include_str!("../../input/day8.txt");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone, Copy)]
enum Instruction {
    Left,
    Right,
}

fn parse_instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    return many1(map_res(alt((tag("L"), tag("R"))), |s: &str| match s {
        "L" => Ok(Instruction::Left),
        "R" => Ok(Instruction::Right),
        _ => Err(()),
    }))(input);
}

type Map<'a> = BTreeMap<(&'a str, Instruction), &'a str>;

fn parse_input(input: &str) -> IResult<&str, (Vec<Instruction>, Map)> {
    let (input, instructions) = parse_instructions(input)?;
    let (input, _) = multispace1(input)?;
    let (input, elements) = separated_list1(newline, |element| {
        let (element, value) = alphanumeric1(element)?;
        let (element, _) = tag(" = ")(element)?;
        let (element, (_, left, _, right, _)) =
            tuple((tag("("), alphanumeric1, tag(", "), alphanumeric1, tag(")")))(element)?;
        return Ok((
            element,
            [
                (value, left, Instruction::Left),
                (value, right, Instruction::Right),
            ],
        ));
    })(input)?;
    let mut map = Map::new();
    for element in elements.into_iter().flatten() {
        map.insert((element.0, element.2), element.1);
    }
    return Ok((input, (instructions, map)));
}

fn part1(input: &str) -> u64 {
    let (instructions, map) = parse_input(input).unwrap().1;
    let mut steps = 0;
    let mut current = "AAA";
    for instruction in instructions.iter().cycle() {
        if current == "ZZZ" {
            return steps;
        }
        let next = map.get(&(current, *instruction)).unwrap();
        current = next;
        steps += 1;
    }
    return steps;
}
fn part2(input: &str) -> u64 {
    let (instructions, map) = parse_input(input).unwrap().1;
    let solution = map
        .keys()
        .filter_map(|(a, _)| a.ends_with('A').then_some(a))
        .map(|point| {
            let mut start = point;
            for (index, instruction) in instructions.iter().cycle().enumerate() {
                if start.ends_with('Z') {
                    return index as u64;
                }
                start = map.get(&(start, *instruction)).unwrap();
            }
            unreachable!("No path to Z found")
        })
        .fold(1, integer::lcm);
    return solution;
}

#[cfg(test)]
mod tests {
    const INPUT_PART_1: &str = "\
    LLR\n\
    \n\
    AAA = (BBB, BBB)\n\
    BBB = (AAA, ZZZ)\n\
    ZZZ = (ZZZ, ZZZ)";
    const INPUT_PART_2: &str = "\
    LR\n\
    \n\
    11A = (11B, XXX)\n\
    11B = (XXX, 11Z)\n\
    11Z = (11B, XXX)\n\
    22A = (22B, XXX)\n\
    22B = (22C, 22C)\n\
    22C = (22Z, 22Z)\n\
    22Z = (22B, 22B)\n\
    XXX = (XXX, XXX)";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT_PART_1), 6);
    }
    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT_PART_2), 6);
    }
}
