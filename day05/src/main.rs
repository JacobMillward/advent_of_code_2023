mod almanac;

use almanac::Almanac;

fn main() {
    let puzzle_input = include_str!("../input.txt");

    let almanac = Almanac::from_str(puzzle_input);
    let seed_info = almanac.to_seed_info();

    // Part 1
    let min_location_number = seed_info
        .iter()
        .min_by_key(|info| info.location)
        .unwrap()
        .location;

    println!("Minimum Seed Location Number: {}", min_location_number);
}