use nom::{bytes::complete::tag, character::complete::digit1, multi::separated_list1, IResult};

fn main() {
    let input = include_str!("../../input/day5.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[derive(Default, Debug)]
struct Map {
    destination: (usize, usize),
    source: (usize, usize),
}

#[derive(Default, Debug)]
struct Almanac {
    seeds_part1: Vec<usize>,
    seeds_part2: Vec<(usize, usize)>,
    seed_to_soil: Vec<Map>,
    soil_to_fertilizer: Vec<Map>,
    fertilizer_to_water: Vec<Map>,
    water_to_light: Vec<Map>,
    light_to_temperature: Vec<Map>,
    temperature_to_humidity: Vec<Map>,
    humidity_to_location: Vec<Map>,
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, (destination_start, _, source_start, _, length)) =
        nom::sequence::tuple((digit1, tag(" "), digit1, tag(" "), digit1))(input)?;
    let length: usize = length.parse().unwrap();
    let destination_start = destination_start.parse().unwrap();
    let source_start = source_start.parse().unwrap();
    return Ok((
        input,
        Map {
            destination: (destination_start, destination_start + length),
            source: (source_start, source_start + length),
        },
    ));
}

fn parse_almanac(input: &str) -> IResult<&str, Almanac> {
    let mut almanac = Almanac::default();
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(tag(" "), digit1)(input)?;
    let seeds: Vec<usize> = seeds.into_iter().map(|s| s.parse().unwrap()).collect();
    almanac.seeds_part2 = seeds
        .chunks(2)
        .map(|numbers| {
            let start = numbers[0];
            let length = numbers[1];
            (start, start + length)
        })
        .collect();
    almanac.seeds_part1 = seeds;
    let (input, _) = tag("\n\nseed-to-soil map:\n")(input)?;
    let (input, maps) = separated_list1(tag("\n"), parse_map)(input)?;
    almanac.seed_to_soil = maps;
    let (input, _) = tag("\n\nsoil-to-fertilizer map:\n")(input)?;
    let (input, maps) = separated_list1(tag("\n"), parse_map)(input)?;
    almanac.soil_to_fertilizer = maps;
    let (input, _) = tag("\n\nfertilizer-to-water map:\n")(input)?;
    let (input, maps) = separated_list1(tag("\n"), parse_map)(input)?;
    almanac.fertilizer_to_water = maps;
    let (input, _) = tag("\n\nwater-to-light map:\n")(input)?;
    let (input, maps) = separated_list1(tag("\n"), parse_map)(input)?;
    almanac.water_to_light = maps;
    let (input, _) = tag("\n\nlight-to-temperature map:\n")(input)?;
    let (input, maps) = separated_list1(tag("\n"), parse_map)(input)?;
    almanac.light_to_temperature = maps;
    let (input, _) = tag("\n\ntemperature-to-humidity map:\n")(input)?;
    let (input, maps) = separated_list1(tag("\n"), parse_map)(input)?;
    almanac.temperature_to_humidity = maps;
    let (input, _) = tag("\n\nhumidity-to-location map:\n")(input)?;
    let (input, maps) = separated_list1(tag("\n"), parse_map)(input)?;
    almanac.humidity_to_location = maps;
    return Ok((input, almanac));
}

fn map_source_to_destination<'a>(maps: &'a [Map], source: usize) -> usize {
    let map = maps
        .iter()
        .find(|map| (map.source.0..map.source.1).contains(&source));
    match map {
        Some(map) => source.abs_diff(map.source.0) + map.destination.0,
        None => source,
    }
}

fn part1(input: &str) -> usize {
    let almanac = parse_almanac(input).unwrap().1;
    let lowest_location = almanac
        .seeds_part1
        .iter()
        .map(|&seed| map_source_to_destination(&almanac.seed_to_soil, seed))
        .map(|soil| map_source_to_destination(&almanac.soil_to_fertilizer, soil))
        .map(|fertilizer| map_source_to_destination(&almanac.fertilizer_to_water, fertilizer))
        .map(|water| map_source_to_destination(&almanac.water_to_light, water))
        .map(|light| map_source_to_destination(&almanac.light_to_temperature, light))
        .map(|temperature| map_source_to_destination(&almanac.temperature_to_humidity, temperature))
        .map(|humidity| map_source_to_destination(&almanac.humidity_to_location, humidity))
        .min();
    return lowest_location.unwrap();
}

fn map_destination_to_source<'a>(maps: &'a [Map], destination: usize) -> usize {
    let map = maps
        .iter()
        .find(|map| (map.destination.0..map.destination.1).contains(&destination));
    match map {
        Some(map) => destination.abs_diff(map.destination.0) + map.source.0,
        None => destination,
    }
}

fn part2(input: &str) -> usize {
    let almanac = parse_almanac(input).unwrap().1;
    let mut location = 0;
    loop {
        let humidity = map_destination_to_source(&almanac.humidity_to_location, location);
        let temperature = map_destination_to_source(&almanac.temperature_to_humidity, humidity);
        let light = map_destination_to_source(&almanac.light_to_temperature, temperature);
        let water = map_destination_to_source(&almanac.water_to_light, light);
        let fertilizer = map_destination_to_source(&almanac.fertilizer_to_water, water);
        let soil = map_destination_to_source(&almanac.soil_to_fertilizer, fertilizer);
        let seed = map_destination_to_source(&almanac.seed_to_soil, soil);
        if almanac
            .seeds_part2
            .iter()
            .any(|(start, end)| (*start..*end).contains(&seed))
        {
            return location;
        }
        location += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13\n\
                \n\
                seed-to-soil map:\n\
                50 98 2\n\
                52 50 48\n\
                \n\
                soil-to-fertilizer map:\n\
                0 15 37\n\
                37 52 2\n\
                39 0 15\n\
                \n\
                fertilizer-to-water map:\n\
                49 53 8\n\
                0 11 42\n\
                42 0 7\n\
                57 7 4\n\
                \n\
                water-to-light map:\n\
                88 18 7\n\
                18 25 70\n\
                \n\
                light-to-temperature map:\n\
                45 77 23\n\
                81 45 19\n\
                68 64 13\n\
                \n\
                temperature-to-humidity map:\n\
                0 69 1\n\
                1 0 69\n\
                \n\
                humidity-to-location map:\n\
                60 56 37\n\
                56 93 4";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 46);
    }
}
