use nom::{bytes::complete::tag, character::complete::digit1, combinator::map_res};

fn main() {
    let input = include_str!("../../input/day2.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

enum Color {
    Red,
    Green,
    Blue,
}

#[derive(PartialEq, Debug)]
struct Cubes {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(PartialEq, Debug)]
struct Game {
    id: i32,
    sets: Vec<Cubes>,
}

fn parse_cubes(input: &str) -> nom::IResult<&str, Cubes> {
    let mut cubes = Cubes {
        red: 0,
        green: 0,
        blue: 0,
    };
    let (input, _) = nom::multi::separated_list1(tag(", "), |color| {
        let (input, count) = map_res(digit1, str::parse::<i32>)(color)?;
        let (input, _) = tag(" ")(input)?;
        let (input, color) = nom::branch::alt((
            map_res(tag("red"), |_| {
                Ok::<Color, nom::Err<nom::error::Error<&str>>>(Color::Red)
            }),
            map_res(tag("green"), |_| {
                Ok::<Color, nom::Err<nom::error::Error<&str>>>(Color::Green)
            }),
            map_res(tag("blue"), |_| {
                Ok::<Color, nom::Err<nom::error::Error<&str>>>(Color::Blue)
            }),
        ))(input)?;
        match color {
            Color::Red => cubes.red = count,
            Color::Green => cubes.green = count,
            Color::Blue => cubes.blue = count,
        }
        Ok((input, ()))
    })(input)?;
    Ok((input, cubes))
}

impl<'a> TryFrom<&'a str> for Game {
    type Error = nom::Err<nom::error::Error<&'a str>>;
    fn try_from(input: &'a str) -> anyhow::Result<Self, Self::Error> {
        let (input, _) = tag("Game ")(input)?;
        let (input, id) = map_res(digit1, str::parse::<i32>)(input)?;
        let (input, _) = tag(": ")(input)?;
        let (_, sets) = nom::multi::separated_list1(tag("; "), parse_cubes)(input)?;
        Ok(Game { id, sets })
    }
}

fn part1(input: &str) -> i32 {
    let games = input
        .lines()
        .map(Game::try_from)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let possibility_rule = Cubes {
        red: 12,
        green: 13,
        blue: 14,
    };
    let mut total = 0;
    for game in games {
        let valid = game.sets.iter().all(|set| {
            set.red <= possibility_rule.red
                && set.green <= possibility_rule.green
                && set.blue <= possibility_rule.blue
        });
        if valid {
            total += game.id;
        }
    }
    return total;
}
fn part2(input: &str) -> i32 {
    let games = input
        .lines()
        .map(Game::try_from)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let mut total = 0;
    for game in games {
        let mut power = Cubes {
            red: 0,
            green: 0,
            blue: 0,
        };
        for set in game.sets {
            power.red = power.red.max(set.red);
            power.green = power.green.max(set.green);
            power.blue = power.blue.max(set.blue);
        }
        total += power.red * power.green * power.blue;
    }
    return total;
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
                        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
                        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
                        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
                        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 8);
    }

    #[test]
    fn parse_input() {
        assert_eq!(
            Game::try_from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Ok(Game {
                id: 1,
                sets: vec![
                    Cubes {
                        red: 4,
                        green: 0,
                        blue: 3,
                    },
                    Cubes {
                        red: 1,
                        green: 2,
                        blue: 6,
                    },
                    Cubes {
                        red: 0,
                        green: 2,
                        blue: 0,
                    },
                ],
            })
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2286);
    }
}
