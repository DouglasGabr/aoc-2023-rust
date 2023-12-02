fn main() {
    let input = include_str!("../../input/day1.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            let first = line.chars().find(|c| c.is_digit(10)).expect("first digit");
            let last = line
                .chars()
                .rev()
                .find(|c| c.is_digit(10))
                .expect("last digit");
            return format!("{}{}", first, last).parse::<i32>().expect("parse");
        })
        .sum()
}

fn part2(input: &str) -> i32 {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    return input
        .lines()
        .map(|line| -> i32 {
            let mut first = None;
            'char_loop: for i in 0..line.len() {
                let char = line.as_bytes()[i] as char;
                if char.is_digit(10) {
                    first = Some(char);
                    break;
                }
                for (index, spelled) in numbers.iter().enumerate() {
                    if line[i..].starts_with(spelled) {
                        first = Some(std::char::from_digit((index + 1) as u32, 10).unwrap());
                        break 'char_loop;
                    }
                }
            }
            let mut last = None;
            'char_loop: for i in (0..line.len()).rev() {
                let char = line.as_bytes()[i] as char;
                if char.is_digit(10) {
                    last = Some(char);
                    break;
                }
                for (index, spelled) in numbers.iter().enumerate() {
                    if line[i..].starts_with(spelled) {
                        last = Some(std::char::from_digit((index + 1) as u32, 10).unwrap());
                        break 'char_loop;
                    }
                }
            }
            let first = first.expect("first");
            let last = last.expect("last");
            return format!("{}{}", first, last).parse::<i32>().expect("parse");
        })
        .sum();
}

#[cfg(test)]
mod tests {
    const TEST_INPUT_PART_1: &str = "1abc2\n\
                            pqr3stu8vwx\n\
                            a1b2c3d4e5f\n\
                            treb7uchet";
    const TEST_INPUT_PART_2: &str = "two1nine\n\
                                    eightwothree\n\
                                    abcone2threexyz\n\
                                    xtwone3four\n\
                                    4nineeightseven2\n\
                                    zoneight234\n\
                                    7pqrstsixteen";
    #[test]
    fn part1() {
        assert_eq!(super::part1(TEST_INPUT_PART_1), 142);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(TEST_INPUT_PART_2), 281);
    }
}
