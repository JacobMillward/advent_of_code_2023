mod schematic;

use schematic::Schematic;

fn main() {
    let puzzle_input = include_str!("../input.txt");

    let schematic = Schematic::parse_from_contents(puzzle_input);

    // Part 1
    let part_numbers = schematic.get_part_numbers();
    let part_number_sum = part_numbers.iter().sum::<usize>();
    println!("Part number sum: {}", part_number_sum);
}
