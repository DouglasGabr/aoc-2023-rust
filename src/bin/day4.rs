use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space1},
    multi::separated_list1,
};
use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../../input/day4.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug)]
struct Card {
    winning_numbers: BTreeSet<usize>,
    own_numbers: BTreeSet<usize>,
}

fn parse_card(input: &str) -> nom::IResult<&str, Card> {
    let (input, _) = tag("Card")(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = digit1(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, _) = space1(input)?;
    let (input, winning_numbers) = separated_list1(space1, digit1)(input)?;
    let (input, _) = space1(input)?;
    let (input, _) = tag("|")(input)?;
    let (input, _) = space1(input)?;
    let (input, own_numbers) = separated_list1(space1, digit1)(input)?;
    Ok((
        input,
        Card {
            winning_numbers: winning_numbers
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect(),
            own_numbers: own_numbers
                .into_iter()
                .map(|n| n.parse().unwrap())
                .collect(),
        },
    ))
}

fn part1(input: &str) -> usize {
    let cards = input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect::<Vec<_>>();
    let mut sum = 0;
    for card in cards.iter() {
        let winning_numbers_count = card.own_numbers.intersection(&card.winning_numbers).count();
        if winning_numbers_count > 0 {
            sum += 2usize.pow((winning_numbers_count - 1) as u32);
        }
    }
    return sum;
}

fn part2(input: &str) -> usize {
    let cards = input
        .lines()
        .map(|line| parse_card(line).unwrap().1)
        .collect::<Vec<_>>();
    let mut copies: Vec<usize> = cards.iter().map(|_| 1).collect();
    for (i, card) in cards.iter().enumerate() {
        let winning_numbers_count = card.own_numbers.intersection(&card.winning_numbers).count();
        let current_card_copies = copies[i];
        for next_index in (i + 1)..=(i + winning_numbers_count) {
            let Some(c) = copies.get_mut(next_index) else {
                break;
            };
            *c += current_card_copies;
        }
    }
    return copies.iter().sum();
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
                        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
                        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
                        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
                        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
                        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 30);
    }
}
