use std::cmp::min;
use std::env;
use std::fs::File;
use std::io::Read;

#[derive(Debug)]
enum PuzzleMode {
    PartOne,
    PartTwo,
}

fn main() {
    let input = match parse_args() {
        Ok(input) => input,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    let (contents, puzzle_mode) = input;
    let total_calibration_values = match puzzle_mode {
        PuzzleMode::PartOne => parse_part_one(&contents),
        PuzzleMode::PartTwo => parse_part_two(&contents),
    };

    println!("Running puzzle mode: {:?}... ", puzzle_mode);
    println!("Sum of calibration values: {}", total_calibration_values);
}

fn parse_args() -> Result<(String, PuzzleMode), std::io::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Please provide a file path as the first argument",
        ));
    }

    let file_path = &args[1];

    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => {
            return Err(error);
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let puzzle_mode = match args.len() {
        2 => PuzzleMode::PartOne,
        3 => {
            let mode = &args[2];
            match mode.as_str() {
                "part1" => PuzzleMode::PartOne,
                "part2" => PuzzleMode::PartTwo,
                _ => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Please provide a valid puzzle mode: 'part1' or 'part2'",
                    ));
                }
            }
        }
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Please provide a valid puzzle mode",
            ));
        }
    };

    Ok((contents, puzzle_mode))
}

fn parse_part_one(input: &str) -> i32 {
    let mut total_calibration_values = 0;
    for line in input.lines() {
        let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last_digit = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();

        let calibration_value = (first_digit.to_string() + &last_digit.to_string())
            .parse::<i32>()
            .unwrap();

        total_calibration_values += calibration_value;
    }

    total_calibration_values
}

fn parse_part_two(input: &str) -> i32 {
    const DIGIT_NAMES: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let max_digit_name_length = DIGIT_NAMES
        .iter()
        .map(|digit_name| digit_name.len())
        .max()
        .unwrap();

    let mut total_calibration_values = 0;
    for line in input.lines() {
        let line_chars: Vec<char> = line.chars().collect();
        let mut first_digit = None;
        let mut last_digit = None;

        let max_window_size = min(max_digit_name_length, line_chars.len());

        fn find_digit(window: &str, reverse_search: bool) -> Option<u32> {
            let mut digit_name_index = None;
            let mut digit_index = None;

            // Check if the window contains a digit name
            let name_search_result =
                DIGIT_NAMES
                    .iter()
                    .enumerate()
                    .find_map(|(index, &digit_name)| {
                        if let Some(start_index) = window.find(digit_name) {
                            digit_name_index = Some(start_index);
                            Some(index)
                        } else {
                            None
                        }
                    });

            // Check if the window contains a digit
            let digit_search_result = if reverse_search {
                window.chars().rev().enumerate().find_map(|(index, c)| {
                    if c.is_ascii_digit() {
                        digit_index = Some(window.len() - index - 1);
                        Some(c)
                    } else {
                        None
                    }
                })
            } else {
                window.chars().enumerate().find_map(|(index, c)| {
                    if c.is_ascii_digit() {
                        digit_index = Some(index);
                        Some(c)
                    } else {
                        None
                    }
                })
            };

            match (digit_name_index, digit_index) {
                // If both digit name and digit are found, return the one that is closest to the start of the window
                (Some(digit_name_start_index), Some(digit_start_index)) => {
                    if (reverse_search && digit_name_start_index > digit_start_index)
                        || (!reverse_search && digit_name_start_index < digit_start_index)
                    {
                        name_search_result.map(|index| index as u32 + 1)
                    } else {
                        digit_search_result.and_then(|c| c.to_digit(10))
                    }
                }
                (Some(_), None) => name_search_result.map(|index| index as u32 + 1),
                (None, Some(_)) => digit_search_result.and_then(|c| c.to_digit(10)),
                _ => None,
            }
        }

        let window_indices = 0..=line_chars.len().saturating_sub(max_window_size);

        // Find the first digit
        for i in window_indices.clone() {
            let window = &line[i..i + max_window_size];
            if let Some(digit) = find_digit(window, false) {
                first_digit = Some(digit);
                break;
            }
        }

        // Find the last digit
        for i in window_indices.rev() {
            let window = &line[i..i + max_window_size];
            if let Some(digit) = find_digit(window, true) {
                last_digit = Some(digit);
                break;
            }
        }

        let calibration_value = (first_digit.unwrap().to_string()
            + &last_digit.unwrap().to_string())
            .parse::<i32>()
            .unwrap();

        total_calibration_values += calibration_value;
    }

    total_calibration_values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_part_one() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let total_calibration_values = parse_part_one(input);
        assert_eq!(total_calibration_values, 142);
    }

    #[test]
    fn test_parse_part_two() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let total_calibration_values = parse_part_two(input);
        assert_eq!(total_calibration_values, 281);
    }
}
