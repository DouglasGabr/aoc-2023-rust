use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input/day3.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Debug, Clone, Copy)]
struct Number {
    value: usize,
    row: usize,
    cols: (usize, usize),
}

#[derive(Debug)]
struct Engine {
    numbers: Vec<Number>,
    symbols: HashMap<(usize, usize), char>,
    gears: Vec<(usize, usize)>,
}

impl From<&str> for Engine {
    fn from(value: &str) -> Self {
        let mut engine = Engine {
            numbers: Vec::new(),
            symbols: HashMap::new(),
            gears: Vec::new(),
        };
        for (row, line) in value.lines().enumerate() {
            let mut col = 0;
            while col < line.len() {
                let c = line.as_bytes()[col] as char;
                if c.is_digit(10) {
                    let num_str = line[col..]
                        .chars()
                        .take_while(|c| c.is_digit(10))
                        .collect::<String>();
                    let num = num_str.parse::<usize>().unwrap();
                    engine.numbers.push(Number {
                        value: num,
                        row,
                        cols: (col, col + num_str.len() - 1),
                    });
                    col += num_str.len();
                    continue;
                } else if c != '.' {
                    engine.symbols.insert((row, col), c);
                    if c == '*' {
                        engine.gears.push((row, col));
                    }
                }
                col += 1;
            }
        }
        return engine;
    }
}

fn part1(input: &str) -> usize {
    let engine = Engine::from(input);
    let mut sum = 0;
    'num_loop: for number in engine.numbers.iter() {
        for row in number.row.checked_sub(1).unwrap_or(0)..=(number.row + 1) {
            for col in number.cols.0.checked_sub(1).unwrap_or(0)..=(number.cols.1 + 1) {
                if let Some(_) = engine.symbols.get(&(row, col)) {
                    sum += number.value;
                    continue 'num_loop;
                }
            }
        }
    }
    return sum;
}

fn part2(input: &str) -> usize {
    let engine = Engine::from(input);
    let mut sum = 0;
    for &(row, col) in engine.gears.iter() {
        let adjacent_numbers = engine
            .numbers
            .iter()
            .filter(|num| {
                num.row.abs_diff(row) <= 1
                    && (num.cols.0.abs_diff(col) <= 1 || num.cols.1.abs_diff(col) <= 1)
            })
            .collect::<Vec<_>>();
        if adjacent_numbers.len() == 2 {
            sum += adjacent_numbers[0].value * adjacent_numbers[1].value;
        }
    }
    return sum;
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "467..114..\n\
                        ...*......\n\
                        ..35..633.\n\
                        ......#...\n\
                        617*......\n\
                        .....+.58.\n\
                        ..592.....\n\
                        ......755.\n\
                        ...$.*....\n\
                        .664.598..";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 4361);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 467835);
    }
}
