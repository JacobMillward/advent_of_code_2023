extern crate regex;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, str::Lines};

use super::info_type::{InfoMapping, InfoType};

/// A map of info to a tuple of another info type and a map to translate the info value from one to another
type InfoMap = HashMap<InfoType, (InfoType, InfoMapping)>;

pub struct Almanac {
    pub seeds: Vec<(usize, usize)>,
    pub info_map: InfoMap,
}

impl Almanac {
    pub fn from_str_part1(input: &str) -> Self {
        let mut lines = input.lines();

        let first_line = lines.next().unwrap();
        let seeds = first_line
            .split("seeds: ")
            .nth(1)
            .unwrap()
            .split(' ')
            .map(|s| (s.parse::<usize>().unwrap(), 1))
            .collect::<Vec<_>>();

        let info_map = Self::parse_info(lines);

        Self { seeds, info_map }
    }

    pub fn from_str_part2(input: &str) -> Self {
        let mut lines = input.lines();

        let first_line = lines.next().unwrap();
        let seeds = first_line
            .split("seeds: ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .tuples::<(usize, usize)>()
            .collect::<Vec<_>>();

        let info_map = Self::parse_info(lines);

        Self { seeds, info_map }
    }

    fn parse_info(lines: Lines<'_>) -> InfoMap {
        #[derive(Debug, PartialEq)]
        enum State {
            InDefinition,
            InInfo,
            Blank,
        }

        let mut current_state = State::Blank;

        let mut info_map = InfoMap::new();

        let info_definition_regex = Regex::new(r"(\w+)-to-(\w+) map:").unwrap();
        let mut current_map_type = None;

        for line in lines {
            let current_line_state = match line.chars().next() {
                Some(c) if c.is_ascii_digit() => State::InInfo,
                Some(c) if c.is_whitespace() => State::Blank,
                None => State::Blank,
                _ => State::InDefinition,
            };

            match (&current_state, &current_line_state) {
                (State::Blank, State::InDefinition) => {
                    let captures = info_definition_regex.captures(line).unwrap();
                    let from = InfoType::from_str(captures.get(1).unwrap().as_str());
                    let to = InfoType::from_str(captures.get(2).unwrap().as_str());

                    current_map_type = Some(from);
                    info_map.insert(from, (to, InfoMapping { ranges: Vec::new() }));
                }

                (State::InDefinition, State::InInfo) | (State::InInfo, State::InInfo) => {
                    let (_, map) = info_map.get_mut(&current_map_type.unwrap()).unwrap();

                    let values = line
                        .split(' ')
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect::<Vec<_>>();
                    let range_tuple = (values[0], values[1], values[2]);

                    map.ranges.push(range_tuple);
                }

                (State::InInfo, State::Blank) => {
                    current_map_type = None;
                }

                (State::Blank, State::Blank) => (),

                _ => panic!(
                    "Invalid state transition: {:?} -> {:?}",
                    current_state, current_line_state
                ),
            }

            current_state = current_line_state;
        }

        info_map
    }

    pub fn to_seed_info(&self) -> Vec<SeedInfo> {
        let mut seed_info = Vec::new();

        for seed_range in &self.seeds {
            let mut seed_info_map = HashMap::new();

            // Start with Seed, end with Location
            let mut current_info_type = InfoType::Seed;
            let mut current_info_value = seed_range.0;

            while current_info_type != InfoType::Location {
                let (to_info_type, info_mapping) = match self.info_map.get(&current_info_type) {
                    Some(map) => map,
                    None => break,
                };

                let next_info_value = info_mapping.transform(current_info_value);
                seed_info_map.insert(to_info_type, next_info_value);

                current_info_type = *to_info_type;
                current_info_value = next_info_value;
            }

            let info = SeedInfo {
                seed: seed_range.0,
                soil: *seed_info_map.get(&InfoType::Soil).unwrap(),
                fertilizer: *seed_info_map.get(&InfoType::Fertilizer).unwrap(),
                water: *seed_info_map.get(&InfoType::Water).unwrap(),
                light: *seed_info_map.get(&InfoType::Light).unwrap(),
                temperature: *seed_info_map.get(&InfoType::Temperature).unwrap(),
                humidity: *seed_info_map.get(&InfoType::Humidity).unwrap(),
                location: *seed_info_map.get(&InfoType::Location).unwrap(),
            };

            seed_info.push(info);
        }

        seed_info
    }
}

pub struct SeedInfo {
    pub seed: usize,
    pub soil: usize,
    pub fertilizer: usize,
    pub water: usize,
    pub light: usize,
    pub temperature: usize,
    pub humidity: usize,
    pub location: usize,
}

#[cfg(test)]
mod almanac_tests {
    use super::*;

    #[test]
    fn test_from_str_part1() {
        let input = r#"seeds: 1 2 3 4

seed-to-soil map:
50 98 2"#;

        let almanac = Almanac::from_str_part1(input);

        assert_eq!(almanac.seeds, vec![(1, 1), (2, 1), (3, 1), (4, 1)]);
    }

    #[test]
    fn test_from_str_part2() {
        let input = r#"seeds: 1 2 3 4

seed-to-soil map:
50 98 2"#;

        let almanac = Almanac::from_str_part2(input);

        assert_eq!(almanac.seeds, vec![(1, 2), (3, 4)]);
    }

    #[test]
    fn test_parse_info() {
        let input = r#"seed-to-soil map:
50 98 2
52 50 48"#;

        let info_map = Almanac::parse_info(input.lines());

        let expected_map = {
            let mut map = InfoMap::new();
            map.insert(
                InfoType::Seed,
                (
                    InfoType::Soil,
                    InfoMapping {
                        ranges: vec![(50, 98, 2), (52, 50, 48)],
                    },
                ),
            );
            map
        };

        assert_eq!(info_map, expected_map);
    }

    #[test]
    fn test_to_seed_info() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

        let almanac = Almanac::from_str_part1(input);
        let seed_info = almanac.to_seed_info();

        let expected_soil_numbers = [(79, 81), (14, 14), (55, 57), (13, 13)];

        for (seed, expected_soil) in seed_info.iter().zip(expected_soil_numbers.iter()) {
            assert_eq!(seed.seed, expected_soil.0);
            assert_eq!(seed.soil, expected_soil.1);
        }

        let min_location_number = seed_info
            .iter()
            .min_by_key(|info| info.location)
            .unwrap()
            .location;

        assert_eq!(min_location_number, 35);
    }
}
