use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1, u64},
    multi::separated_list1,
    IResult,
};

fn main() {
    let input = include_str!("../../input/day6.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

struct Race {
    time_ms: u64,
    record_mm: u64,
}
impl Race {
    fn hold_for(&self, time: u64) -> u64 {
        time * self.time_ms.checked_sub(time).unwrap_or(0)
    }
}

fn parse_races_part1(input: &str) -> IResult<&str, Vec<Race>> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list1(space1, u64)(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = separated_list1(space1, u64)(input)?;
    return Ok((
        input,
        times
            .into_iter()
            .zip(distances.into_iter())
            .map(|(time, distance)| Race {
                time_ms: time,
                record_mm: distance,
            })
            .collect(),
    ));
}
fn parse_race_part2(input: &str) -> IResult<&str, Race> {
    let (input, _) = tag("Time:")(input)?;
    let (input, _) = space1(input)?;
    let (input, times) = separated_list1(space1, digit1)(input)?;
    let time: u64 = times.into_iter().collect::<String>().parse().unwrap();
    let (input, _) = newline(input)?;
    let (input, _) = tag("Distance:")(input)?;
    let (input, _) = space1(input)?;
    let (input, distances) = separated_list1(space1, digit1)(input)?;
    let distance: u64 = distances.into_iter().collect::<String>().parse().unwrap();
    return Ok((
        input,
        Race {
            time_ms: time,
            record_mm: distance,
        },
    ));
}

fn count_winning_rounds(race: &Race) -> u64 {
    let lose_at_start = (1..=race.time_ms)
        .map(|time| race.hold_for(time))
        .take_while(|distance| race.record_mm >= *distance)
        .count() as u64;
    let lose_at_end = (1..=race.time_ms)
        .rev()
        .map(|time| race.hold_for(time))
        .take_while(|distance| race.record_mm >= *distance)
        .count() as u64;
    return race.time_ms - lose_at_start - lose_at_end;
}

fn part1(input: &str) -> u64 {
    let races = parse_races_part1(input).unwrap().1;
    return races.iter().map(count_winning_rounds).product();
}

fn part2(input: &str) -> u64 {
    let race = parse_race_part2(input).unwrap().1;
    return count_winning_rounds(&race);
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Time:      7  15   30\n\
                        Distance:  9  40  200";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 288)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 71503)
    }
}
