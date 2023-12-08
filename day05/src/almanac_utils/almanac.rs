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
    /// Parses the almanac from a string
    /// Assumes each sead in the "seeds" line is a single value
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

    /// Parses the almanac from a string
    /// Assumes each sead in the "seeds" line is a pair of values describing a range of seeds in the format
    /// `<start> <length>`
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

                    map.add_range(values[0], values[1], values[2]);
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

    /// Calculates the seed info for each seed in the almanac
    pub fn to_seed_info(&self) -> Vec<SeedRangeInfo> {
        let mut seed_info = Vec::new();

        for seed_range in &self.seeds {
            let mut seed_info_map: HashMap<InfoType, Vec<(usize, usize)>> = HashMap::new();

            // Start with Seed, end with Location
            let mut current_info_type = InfoType::Seed;
            let mut current_info_values = vec![*seed_range];

            while current_info_type != InfoType::Location {
                let (to_info_type, info_mapping) = match self.info_map.get(&current_info_type) {
                    Some(map) => map,
                    None => break,
                };

                let mut next_info_values = Vec::new();
                current_info_values.iter().for_each(|(start, len)| {
                    let transformed_ranges = info_mapping.transform_range((*start, *len));
                    next_info_values.extend(transformed_ranges);
                });

                seed_info_map.insert(*to_info_type, next_info_values.clone());
                current_info_type = *to_info_type;
                current_info_values = next_info_values;
            }

            let info = SeedRangeInfo {
                seeds: *seed_range,
                soils: seed_info_map.get(&InfoType::Soil).unwrap().clone(),
                fertilizers: seed_info_map.get(&InfoType::Fertilizer).unwrap().clone(),
                waters: seed_info_map.get(&InfoType::Water).unwrap().clone(),
                lights: seed_info_map.get(&InfoType::Light).unwrap().clone(),
                temperatures: seed_info_map.get(&InfoType::Temperature).unwrap().clone(),
                humidities: seed_info_map.get(&InfoType::Humidity).unwrap().clone(),
                locations: seed_info_map.get(&InfoType::Location).unwrap().clone(),
            };

            seed_info.push(info);
        }

        seed_info
    }
}

pub struct SeedRangeInfo {
    pub seeds: (usize, usize),
    pub soils: Vec<(usize, usize)>,
    pub fertilizers: Vec<(usize, usize)>,
    pub waters: Vec<(usize, usize)>,
    pub lights: Vec<(usize, usize)>,
    pub temperatures: Vec<(usize, usize)>,
    pub humidities: Vec<(usize, usize)>,
    pub locations: Vec<(usize, usize)>,
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
                        ranges: vec![(52, 50, 48), (50, 98, 2)],
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

        let min_location_number = seed_info
            .iter()
            .flat_map(|info| &info.locations)
            .min_by_key(|(start, _)| *start)
            .unwrap()
            .0;

        assert_eq!(min_location_number, 35);
    }
}
