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

    /// Returns a list of part numbers in the schematic.
    /// A part number is a number that is not adjacent to any symbols.
    /// Multiple digits in a row are considered a single part number.
    pub fn get_part_numbers(&self) -> Vec<usize> {
        let mut part_numbers = Vec::new();

        let mut adjacent_indexes_set = std::collections::HashSet::new();

        for (row_index, row) in self.grid.iter().enumerate() {
            let mut is_adjacent_to_symbol = false;
            let mut current_part_number = String::new();
            for (column_index, character) in row.iter().enumerate() {
                if character.is_ascii_digit() {
                    current_part_number.push(*character);
                } else if !current_part_number.is_empty() {
                    if is_adjacent_to_symbol {
                        part_numbers.push(current_part_number.parse().unwrap());
                        is_adjacent_to_symbol = false;
                    }

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

                    let current_index_adjacent = adjacent_indexes_set.iter().fold(
                        false,
                        |acc, (row_index, column_index)| {
                            acc || self.is_symbol(*row_index, *column_index)
                        },
                    );

                    is_adjacent_to_symbol = current_index_adjacent || is_adjacent_to_symbol;
                }
            }

            if !current_part_number.is_empty() && is_adjacent_to_symbol {
                part_numbers.push(current_part_number.parse().unwrap());
            }
        }
        part_numbers
    }

    fn is_symbol(&self, row_index: usize, column_index: usize) -> bool {
        let character = self.grid[row_index][column_index];
        !character.is_ascii_digit() && character != '.'
    }
}

#[cfg(test)]
mod schematic_tests {
    use super::*;

    const TEST_CONTENTS: &str = r#"467..114..
...*......
..35..633.
......#...
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
        assert!(part_numbers.contains(&467));
        assert!(part_numbers.contains(&35));
        assert!(part_numbers.contains(&633));
        assert!(part_numbers.contains(&617));
        assert!(part_numbers.contains(&592));
        assert!(part_numbers.contains(&755));
        assert!(part_numbers.contains(&664));
        assert!(part_numbers.contains(&598));
    }
}
