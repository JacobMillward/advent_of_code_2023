mod almanac_utils;

use almanac_utils::Almanac;

fn main() {
    let puzzle_input = include_str!("../input.txt");

    // Part 1
    let almanac = Almanac::from_str_part1(puzzle_input);
    let seed_info = almanac.to_seed_info();

    let min_location_number = seed_info
        .iter()
        .flat_map(|info| &info.locations)
        .min_by_key(|(start, _)| *start)
        .unwrap()
        .0;

    println!(
        "Minimum Seed Location Number Part 1: {}",
        min_location_number
    );

    // Part 2
    let almanac = Almanac::from_str_part2(puzzle_input);
    let seed_info = almanac.to_seed_info();
    let min_location_number = seed_info
        .iter()
        .flat_map(|info| &info.locations)
        .min_by_key(|(start, _)| *start)
        .unwrap()
        .0;

    println!(
        "Minimum Seed Location Number Part 2: {}",
        min_location_number
    );
}
