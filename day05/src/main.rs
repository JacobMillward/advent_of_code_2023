mod seed_info;
use seed_info::Almanac;

fn main() {
    let puzzle_input = include_str!("../input.txt");

    let almanac = Almanac::from_str(puzzle_input);
}
