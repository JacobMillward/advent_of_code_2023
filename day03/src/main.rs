mod schematic;

use schematic::Schematic;

fn main() {
    let puzzle_input = include_str!("../input.txt");

    let schematic = Schematic::parse_from_contents(puzzle_input);

    // Part 1
    let part_numbers = schematic.get_part_numbers();
    let part_number_sum = part_numbers
        .iter()
        .map(|part_number| part_number.0)
        .sum::<usize>();
    println!("Part number sum: {}", part_number_sum);

    // Part 2
    let gear_ratios = Schematic::get_gear_ratios(&part_numbers);
    let gear_ratio_sum = gear_ratios.iter().sum::<usize>();
    println!("Gear ratio sum: {}", gear_ratio_sum);
}
