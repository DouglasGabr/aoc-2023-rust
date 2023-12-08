use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("../../input/day7.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(PartialEq, PartialOrd, Clone, Copy, Eq, Hash, Ord, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn parse_day1_card(c: char) -> Card {
    match c {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'J' => Card::Jack,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        _ => panic!("Invalid card"),
    }
}

fn parse_day2_card(c: char) -> Card {
    match c {
        '2' => Card::Two,
        '3' => Card::Three,
        '4' => Card::Four,
        '5' => Card::Five,
        '6' => Card::Six,
        '7' => Card::Seven,
        '8' => Card::Eight,
        '9' => Card::Nine,
        'T' => Card::Ten,
        'Q' => Card::Queen,
        'K' => Card::King,
        'A' => Card::Ace,
        'J' => Card::Joker,
        _ => panic!("Invalid card"),
    }
}

#[derive(PartialEq, Clone, Copy, Eq, Debug)]
struct Hand(Card, Card, Card, Card, Card);

fn parse_day1_hand(input: &str) -> Hand {
    let mut chars = input.chars();
    let card1 = chars.next().expect("Invalid hand");
    let card2 = chars.next().expect("Invalid hand");
    let card3 = chars.next().expect("Invalid hand");
    let card4 = chars.next().expect("Invalid hand");
    let card5 = chars.next().expect("Invalid hand");
    Hand(
        parse_day1_card(card1),
        parse_day1_card(card2),
        parse_day1_card(card3),
        parse_day1_card(card4),
        parse_day1_card(card5),
    )
}
fn parse_day2_hand(input: &str) -> Hand {
    let mut chars = input.chars();
    let card1 = chars.next().expect("Invalid hand");
    let card2 = chars.next().expect("Invalid hand");
    let card3 = chars.next().expect("Invalid hand");
    let card4 = chars.next().expect("Invalid hand");
    let card5 = chars.next().expect("Invalid hand");
    Hand(
        parse_day2_card(card1),
        parse_day2_card(card2),
        parse_day2_card(card3),
        parse_day2_card(card4),
        parse_day2_card(card5),
    )
}

#[derive(PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl From<Hand> for HandType {
    fn from(value: Hand) -> Self {
        let mut seen: HashMap<Card, u64> = HashMap::new();
        for card in [value.0, value.1, value.2, value.3, value.4] {
            seen.entry(card)
                .and_modify(|entry| *entry += 1)
                .or_insert(1);
        }
        match seen {
            s if s.len() == 1 => HandType::FiveOfAKind,
            s if s.len() == 2 => {
                let jokers = s.get(&Card::Joker).unwrap_or(&0);
                if s.values().any(|&count| count == 4) {
                    match jokers {
                        1 | 4 => HandType::FiveOfAKind,
                        _ => HandType::FourOfAKind,
                    }
                } else {
                    match jokers {
                        2 | 3 => HandType::FiveOfAKind,
                        _ => HandType::FullHouse,
                    }
                }
            }
            s if s.len() == 3 => {
                let jokers = s.get(&Card::Joker).unwrap_or(&0);
                if s.values().any(|&count| count == 3) {
                    match jokers {
                        1 | 3 => HandType::FourOfAKind,
                        _ => HandType::ThreeOfAKind,
                    }
                } else {
                    match jokers {
                        1 => HandType::FullHouse,
                        2 => HandType::FourOfAKind,
                        _ => HandType::TwoPair,
                    }
                }
            }
            s if s.len() == 4 => {
                let jokers = s.get(&Card::Joker).unwrap_or(&0);
                match jokers {
                    1 | 2 => HandType::ThreeOfAKind,
                    _ => HandType::OnePair,
                }
            }
            s => {
                let jokers = s.get(&Card::Joker).unwrap_or(&0);
                match jokers {
                    1 => HandType::OnePair,
                    _ => HandType::HighCard,
                }
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type: HandType = self.clone().into();
        let other_type: HandType = other.clone().into();
        self_type
            .cmp(&other_type)
            .then_with(|| self.0.cmp(&other.0))
            .then_with(|| self.1.cmp(&other.1))
            .then_with(|| self.2.cmp(&other.2))
            .then_with(|| self.3.cmp(&other.3))
            .then_with(|| self.4.cmp(&other.4))
    }
}

fn part1(input: &str) -> u64 {
    let mut set_of_hands = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand_str, bid)| {
            (
                parse_day1_hand(hand_str),
                bid.trim().parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    set_of_hands.sort_by_key(|(hand, _)| *hand);
    return set_of_hands
        .into_iter()
        .enumerate()
        .map(|(index, (_, bid))| {
            let index = index + 1;
            bid * index as u64
        })
        .sum();
}

fn part2(input: &str) -> u64 {
    let mut set_of_hands = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(hand_str, bid)| {
            (
                parse_day2_hand(hand_str),
                bid.trim().parse::<u64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    set_of_hands.sort_by_key(|(hand, _)| *hand);
    return set_of_hands
        .into_iter()
        .enumerate()
        .map(|(index, (_, bid))| {
            let index = index + 1;
            bid * index as u64
        })
        .sum();
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "32T3K 765\n\
                        T55J5 684\n\
                        KK677 28\n\
                        KTJJT 220\n\
                        QQQJA 483";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 6440);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 5905);
    }
}
