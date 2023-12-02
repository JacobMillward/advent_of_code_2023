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
        _ => parse_part_one(&contents),
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
