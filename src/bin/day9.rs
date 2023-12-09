fn main() {
    let input = include_str!("../../input/day9.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|starting_numbers| {
            let mut stack = vec![starting_numbers.clone()];
            let mut current = starting_numbers;
            while !current.iter().all(|n| *n == 0) {
                let mut new = Vec::with_capacity(current.len() - 1);
                for i in 0..current.len() - 1 {
                    let a = current[i];
                    let b = current[i + 1];
                    new.push(b - a);
                }
                stack.push(new.clone());
                current = new;
            }
            let mut result = 0;
            for list in stack.into_iter().rev() {
                result += list.last().unwrap();
            }
            return result;
        })
        .sum()
}
fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|starting_numbers| {
            let mut stack = vec![starting_numbers.clone()];
            let mut current = starting_numbers;
            while !current.iter().all(|n| *n == 0) {
                let mut new = Vec::with_capacity(current.len() - 1);
                for i in 0..current.len() - 1 {
                    let a = current[i];
                    let b = current[i + 1];
                    new.push(b - a);
                }
                stack.push(new.clone());
                current = new;
            }
            let mut result = 0;
            for list in stack.into_iter().rev() {
                result = list.first().unwrap() - result;
            }
            return result;
        })
        .sum()
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "\
    0 3 6 9 12 15\n\
    1 3 6 10 15 21\n\
    10 13 16 21 30 45";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 114);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 2);
    }
}
