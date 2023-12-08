fn main() {
    let puzzle_input = include_str!("../input.txt");

    let races = Race::from_str(puzzle_input);
}

struct Race {
    pub time_ms: usize,
    pub distance_mm: usize,
}

impl Race {
    pub fn from_str(input: &str) -> Vec<Race> {
        let mut lines = input.lines();

        let times = lines
            .next()
            .unwrap()
            .trim_start_matches("Time:")
            .split_ascii_whitespace()
            .map(|time| time.parse::<usize>().unwrap());

        let distances = lines
            .next()
            .unwrap()
            .trim_start_matches("Distance:")
            .split_ascii_whitespace()
            .map(|distance| distance.parse::<usize>().unwrap());

        times
            .zip(distances)
            .map(|(time, distance)| Race {
                time_ms: time,
                distance_mm: distance,
            })
            .collect()
    }
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        let parsed_races = Race::from_str(input);

        assert_eq!(parsed_races.len(), 3);
        assert_eq!(parsed_races[0].time_ms, 7);
        assert_eq!(parsed_races[0].distance_mm, 9);
        assert_eq!(parsed_races[1].time_ms, 15);
        assert_eq!(parsed_races[1].distance_mm, 40);
        assert_eq!(parsed_races[2].time_ms, 30);
        assert_eq!(parsed_races[2].distance_mm, 200);
    }
}
