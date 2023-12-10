use petgraph::graphmap::UnGraphMap;

fn main() {
    let input = include_str!("../../input/day10.txt");
    println!("Part 1: {}", part1(input));
}

fn parse_graph(input: &str) -> UnGraphMap<(char, (usize, usize)), u64> {
    let graph = UnGraphMap::from_edges(input.lines().enumerate().flat_map(|(x, line)| {
        line.chars()
            .enumerate()
            .filter_map(move |(y, c)| {
                let left = y.checked_sub(1).map(|y| (x, y)).and_then(|pos| {
                    input
                        .lines()
                        .nth(pos.0)
                        .and_then(|line| line.chars().nth(pos.1))
                        .filter(|&c| matches!(c, 'F' | '-' | 'L'))
                        .map(|c| (c, pos))
                });
                let right = y.checked_add(1).map(|y| (x, y)).and_then(|pos| {
                    input
                        .lines()
                        .nth(pos.0)
                        .and_then(|line| line.chars().nth(pos.1))
                        .filter(|&c| matches!(c, 'J' | '-' | '7'))
                        .map(|c| (c, pos))
                });
                let up = x.checked_sub(1).map(|x| (x, y)).and_then(|pos| {
                    input
                        .lines()
                        .nth(pos.0)
                        .and_then(|line| line.chars().nth(pos.1))
                        .filter(|&c| matches!(c, 'F' | '|' | '7'))
                        .map(|c| (c, pos))
                });
                let down = x.checked_add(1).map(|x| (x, y)).and_then(|pos| {
                    input
                        .lines()
                        .nth(pos.0)
                        .and_then(|line| line.chars().nth(pos.1))
                        .filter(|&c| matches!(c, 'J' | '|' | 'L'))
                        .map(|c| (c, pos))
                });
                let current = (c, (x, y));
                match (c, (left, right, up, down)) {
                    ('.', _) => None,
                    ('|', (_, _, Some(up), Some(down))) => {
                        Some([(current, up, 1), (current, down, 1)])
                    }
                    ('-', (Some(left), Some(right), _, _)) => {
                        Some([(current, left, 1), (current, right, 1)])
                    }
                    ('L', (_, Some(right), Some(up), _)) => {
                        Some([(current, right, 1), (current, up, 1)])
                    }
                    ('J', (Some(left), _, Some(up), _)) => {
                        Some([(current, left, 1), (current, up, 1)])
                    }
                    ('7', (Some(left), _, _, Some(down))) => {
                        Some([(current, left, 1), (current, down, 1)])
                    }
                    ('F', (_, Some(right), _, Some(down))) => {
                        Some([(current, right, 1), (current, down, 1)])
                    }
                    ('S', _) => {
                        return Some(
                            [up, down, left, right]
                                .iter()
                                .flatten()
                                .map(|pos| (current, *pos, 1))
                                .collect::<Vec<_>>()
                                .try_into()
                                .unwrap(),
                        );
                    }
                    _ => None,
                }
            })
            .flatten()
    }));
    return graph;
}

fn part1(input: &str) -> u64 {
    let graph = parse_graph(input);
    let start = graph.nodes().find(|node| node.0 == 'S').unwrap();
    let longest_path = petgraph::algo::dijkstra(&graph, start, None, |_| 1u64)
        .into_values()
        .max()
        .unwrap();
    return longest_path;
}

#[cfg(test)]
mod tests {
    const INPUT: &str = "\
    ..F7.\n\
    .FJ|.\n\
    SJ.L7\n\
    |F--J\n\
    LJ...";

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 8);
    }
}
