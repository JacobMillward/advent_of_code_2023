fn main() {
    let puzzle_input = include_str!("../input.txt");

    // Part 1
    let races = Race::from_str(puzzle_input);
    let product_of_number_of_winning_methods = races
        .iter()
        .map(|r| {
            let (min_speed, max_speed) = r.speed_to_beat_record();
            max_speed - min_speed + 1
        })
        .product::<usize>();

    println!(
        "Part 1 - Product of number of winning methods: {}",
        product_of_number_of_winning_methods
    );

    // Part 2
    let race = Race::from_str_single_race(puzzle_input);
    let (min_winning_speed, max_winning_speed) = race.speed_to_beat_record();
    let num_winning_speeds = max_winning_speed - min_winning_speed + 1;
    println!("Part 2 - Number of winning speeds: {}", num_winning_speeds);
}

struct Race {
    pub time_ms: usize,
    pub distance_record_mm: usize,
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
                distance_record_mm: distance,
            })
            .collect()
    }

    pub fn from_str_single_race(input: &str) -> Self {
        let mut lines = input.lines();

        let time_ms = lines
            .next()
            .unwrap()
            .trim_start_matches("Time:")
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        let distance_record_mm = lines
            .next()
            .unwrap()
            .trim_start_matches("Distance:")
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        Self {
            time_ms,
            distance_record_mm,
        }
    }

    pub fn speed_to_beat_record(&self) -> (usize, usize) {
        let min_new_distance_record_mm = self.distance_record_mm + 1;

        // Lets do the quadratic formula to find the max speed
        // t is Time Held
        // T is Total Race Time
        // D is Distance travelled in the race
        // D(t) = t(T - t)
        // When rearranged into a quadratic equation
        // t^2 - Tt + D = 0
        // a=1, b=-T, c=D
        // So given we know T, and D is the new distance record, we can solve for t
        // t = (T ± sqrt(T^2 - 4D)) / 2
        let discriminant =
            (self.time_ms * self.time_ms) as f64 - 4.0 * min_new_distance_record_mm as f64;
        let sqrt_discriminant = discriminant.sqrt();

        let max_speed = (self.time_ms as f64 + sqrt_discriminant) / 2.0;
        let min_speed = (self.time_ms as f64 - sqrt_discriminant) / 2.0;

        (min_speed.ceil() as usize, max_speed.floor() as usize)
    }
}

#[cfg(test)]
mod day06_tests {
    use super::*;

    const INPUT: &str = r#"Time:      7  15   30
Distance:  9  40  200"#;

    #[test]
    fn test_from_str() {
        let parsed_races = Race::from_str(INPUT);

        assert_eq!(parsed_races.len(), 3);
        assert_eq!(parsed_races[0].time_ms, 7);
        assert_eq!(parsed_races[0].distance_record_mm, 9);
        assert_eq!(parsed_races[1].time_ms, 15);
        assert_eq!(parsed_races[1].distance_record_mm, 40);
        assert_eq!(parsed_races[2].time_ms, 30);
        assert_eq!(parsed_races[2].distance_record_mm, 200);
    }

    #[test]
    fn test_speed_to_beat_record() {
        let parsed_races = Race::from_str(INPUT);

        assert_eq!(parsed_races[0].speed_to_beat_record(), (2, 5));
        assert_eq!(parsed_races[1].speed_to_beat_record(), (4, 11));
        assert_eq!(parsed_races[2].speed_to_beat_record(), (11, 19));
    }
}
