extern crate regex;
use regex::Regex;
use std::{collections::HashMap, str::Lines};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum InfoType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl InfoType {
    pub fn from_str(text: &str) -> Self {
        match text.to_lowercase().as_str() {
            "seed" => InfoType::Seed,
            "soil" => InfoType::Soil,
            "fertilizer" => InfoType::Fertilizer,
            "water" => InfoType::Water,
            "light" => InfoType::Light,
            "temperature" => InfoType::Temperature,
            "humidity" => InfoType::Humidity,
            "location" => InfoType::Location,
            _ => panic!("Invalid info type: {}", text),
        }
    }
}

pub struct SeedInfo {
    pub seed: u32,
    pub soil: u32,
    pub fertilizer: u32,
    pub water: u32,
    pub light: u32,
    pub temperature: u32,
    pub humidity: u32,
    pub location: u32,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct InfoMapping {
    pub ranges: Vec<(u32, u32, u32)>,
}

/// A map of info to a tuple of another info type and a map to translate the info value from one to another
type InfoMap = HashMap<InfoType, (InfoType, InfoMapping)>;

pub struct Almanac {
    pub seeds: Vec<u32>,
    pub info_map: InfoMap,
}

impl Almanac {
    pub fn from_str(input: &str) -> Self {
        let mut lines = input.lines();

        let first_line = lines.next().unwrap();
        let seeds = first_line
            .split("seeds: ")
            .nth(1)
            .unwrap()
            .split(' ')
            .map(|s| s.parse::<u32>().unwrap())
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
                Some(c) if c.is_digit(10) => State::InInfo,
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
                        .map(|s| s.parse::<u32>().unwrap())
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
        todo!()
    }
}

#[cfg(test)]
mod almanac_tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = r#"seeds: 1 2 3 4 5 6 7 8 9 10

seed-to-soil map:
50 98 2"#;

        let almanac = Almanac::from_str(input);

        assert_eq!(almanac.seeds, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
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

        let almanac = Almanac::from_str(input);
        let seed_info = almanac.to_seed_info();

        let expected_soil_numbers = vec![(79, 81), (14, 14), (55, 57), (13, 13)];

        for (seed, expected_soil) in seed_info.iter().zip(expected_soil_numbers.iter()) {
            assert_eq!(seed.seed, expected_soil.0);
            assert_eq!(seed.soil, expected_soil.1);
        }
    }
}
