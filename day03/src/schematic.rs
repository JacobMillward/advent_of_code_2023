use std::collections::HashSet;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Symbol {
    character: char,
    row_index: usize,
    column_index: usize,
}

pub struct Schematic {
    grid: Vec<Vec<char>>,
}

impl Schematic {
    pub fn new(grid: Vec<Vec<char>>) -> Self {
        Self { grid }
    }

    pub fn parse_from_contents(contents: &str) -> Self {
        let grid = contents
            .lines()
            .map(|line| line.chars().collect::<_>())
            .collect::<_>();

        Self::new(grid)
    }

    /// Returns a list of part numbers in the schematic, alongside the symbols that they are adjacent to.
    /// A part number is a number that is not adjacent to any symbols.
    /// Multiple digits in a row are considered a single part number.
    pub fn get_part_numbers(&self) -> Vec<(usize, Vec<Symbol>)> {
        let mut part_numbers = Vec::new();

        let mut adjacent_indexes_set = HashSet::new();
        let mut adjacent_symbols = HashSet::new();

        for (row_index, row) in self.grid.iter().enumerate() {
            let mut is_adjacent_to_symbol = false;
            let mut current_part_number = String::new();

            adjacent_symbols.clear();
            current_part_number.clear();

            for (column_index, character) in row.iter().enumerate() {
                if character.is_ascii_digit() {
                    current_part_number.push(*character);
                } else if !current_part_number.is_empty() {
                    if is_adjacent_to_symbol {
                        part_numbers.push((
                            current_part_number.parse().unwrap(),
                            adjacent_symbols.clone().into_iter().collect::<Vec<_>>(),
                        ));
                        is_adjacent_to_symbol = false;
                    }

                    adjacent_symbols.clear();
                    current_part_number.clear();
                    continue;
                }

                // If we have a current part number, we should check grid adjacency for any non-digit non-'.' characters.
                // We need to check all 8 adjacent cells, referenced as follows:
                // TL T TR
                //  L C R
                // BL B BR
                if !current_part_number.is_empty() {
                    let adjacent_indexes = vec![
                        (row_index.saturating_sub(1), column_index.saturating_sub(1)), // TL
                        (row_index.saturating_sub(1), column_index),                   // T
                        (row_index.saturating_sub(1), column_index.saturating_add(1)), // TR
                        (row_index, column_index.saturating_sub(1)),                   // L
                        (row_index, column_index.saturating_add(1)),                   // R
                        (row_index.saturating_add(1), column_index.saturating_sub(1)), // BL
                        (row_index.saturating_add(1), column_index),                   // B
                        (row_index.saturating_add(1), column_index.saturating_add(1)), // BR
                    ]
                    .into_iter()
                    .filter(|(row_index, column_index)| {
                        *row_index < self.grid.len() && *column_index < self.grid[*row_index].len()
                    });

                    // if any of these overlap with the current index, or each other, we should omit them.
                    adjacent_indexes_set.clear();
                    adjacent_indexes_set.extend(adjacent_indexes);
                    adjacent_indexes_set.remove(&(row_index, column_index));

                    let current_adjacent_symbols = adjacent_indexes_set
                        .iter()
                        .filter(|(row_index, column_index)| {
                            let character = self.grid[*row_index][*column_index];
                            !character.is_ascii_digit() && character != '.'
                        })
                        .map(|(row_index, column_index)| Symbol {
                            character: self.grid[*row_index][*column_index],
                            row_index: *row_index,
                            column_index: *column_index,
                        })
                        .collect::<Vec<_>>();

                    is_adjacent_to_symbol =
                        !current_adjacent_symbols.is_empty() || is_adjacent_to_symbol;

                    adjacent_symbols.extend(current_adjacent_symbols);
                }
            }

            if !current_part_number.is_empty() && is_adjacent_to_symbol {
                part_numbers.push((
                    current_part_number.parse().unwrap(),
                    adjacent_symbols.clone().into_iter().collect::<Vec<_>>(),
                ));
            }
        }
        part_numbers
    }

    /// Returns a list of gear ratios in the schematic.
    /// A gear ratio is any '*' character adjacent to two part numbers, multiplied together.
    pub fn get_gear_ratios(part_numbers: &[(usize, Vec<Symbol>)]) -> Vec<usize> {
        let mut gear_ratios = Vec::new();

        let gears = part_numbers
            .iter()
            .flat_map(|(_, symbols)| symbols.iter())
            .filter(|symbol| symbol.character == '*')
            .collect::<Vec<_>>();

        for gear in gears {
            let gear_row_index = gear.row_index;
            let gear_column_index = gear.column_index;

            // Find all part numbers that reference this gear
            let adjacent_part_numbers = part_numbers
                .iter()
                .filter(|(_, symbols)| {
                    symbols.iter().any(|symbol| {
                        symbol.row_index == gear_row_index
                            && symbol.column_index == gear_column_index
                    })
                })
                .map(|(part_number, _)| *part_number)
                .collect::<Vec<_>>();

            if adjacent_part_numbers.len() == 2 {
                let gear_ratio = adjacent_part_numbers[0] * adjacent_part_numbers[1];
                if !gear_ratios
                    .iter()
                    .any(|((x, y), _)| *x == gear_row_index && *y == gear_column_index)
                {
                    gear_ratios.push(((gear_row_index, gear_column_index), gear_ratio));
                }
            }
        }

        gear_ratios
            .iter()
            .map(|(_, gear_ratio)| *gear_ratio)
            .collect::<Vec<_>>()
    }
}

#[cfg(test)]
mod schematic_tests {
    use super::*;

    const TEST_CONTENTS: &str = r#"467..114..
...*......
..35...633
.......#..
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    #[test]
    fn test_parse_from_contents() {
        let schematic = Schematic::parse_from_contents(TEST_CONTENTS);

        assert_eq!(schematic.grid.len(), 10);
        for row in schematic.grid {
            assert_eq!(row.len(), 10);
        }
    }

    #[test]
    fn test_get_part_numbers() {
        let schematic = Schematic::parse_from_contents(TEST_CONTENTS);

        let part_numbers = schematic.get_part_numbers();

        assert_eq!(part_numbers.len(), 8);

        let expected = vec![
            (
                467,
                vec![Symbol {
                    character: '*',
                    row_index: 1,
                    column_index: 3,
                }],
            ),
            (
                35,
                vec![Symbol {
                    character: '*',
                    row_index: 1,
                    column_index: 3,
                }],
            ),
            (
                633,
                vec![Symbol {
                    character: '#',
                    row_index: 3,
                    column_index: 7,
                }],
            ),
            (
                617,
                vec![Symbol {
                    character: '*',
                    row_index: 4,
                    column_index: 3,
                }],
            ),
            (
                592,
                vec![Symbol {
                    character: '+',
                    row_index: 5,
                    column_index: 5,
                }],
            ),
            (
                755,
                vec![Symbol {
                    character: '*',
                    row_index: 8,
                    column_index: 5,
                }],
            ),
            (
                664,
                vec![Symbol {
                    character: '$',
                    row_index: 8,
                    column_index: 3,
                }],
            ),
            (
                598,
                vec![Symbol {
                    character: '*',
                    row_index: 8,
                    column_index: 5,
                }],
            ),
        ];

        for (part_number, symbols) in &part_numbers {
            assert!(expected.contains(&(*part_number, symbols.clone())));
        }

        assert_eq!(part_numbers, expected);
    }

    #[test]
    fn test_get_gear_ratios() {
        let schematic = Schematic::parse_from_contents(TEST_CONTENTS);

        let part_numbers = schematic.get_part_numbers();
        let gear_ratios = Schematic::get_gear_ratios(&part_numbers);

        assert_eq!(gear_ratios.len(), 2);
        assert_eq!(gear_ratios[0], 467 * 35);
        assert_eq!(gear_ratios[1], 755 * 598);
    }
}
