use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let input = match read_input_file_from_args() {
        Ok(input) => input,
        Err(error) => {
            println!("Error: {}", error);
            return;
        }
    };

    // Loop over each line in the input
    let mut total_calibration_values = 0;
    for line in input.lines() {
        let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap();
        let last_digit = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
        let calibration_value = (first_digit.to_string() + &last_digit.to_string())
            .parse::<i32>()
            .unwrap();

        total_calibration_values += calibration_value;
    }

    println!("Sum of calibration values: {}", total_calibration_values);
}

fn read_input_file_from_args() -> Result<String, std::io::Error> {
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

    Ok(contents)
}
