#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum InfoType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl InfoType {
    pub fn from_str(text: &str) -> Self {
        match text.to_lowercase().as_str() {
            "seed" => InfoType::Seed,
            "soil" => InfoType::Soil,
            "fertilizer" => InfoType::Fertilizer,
            "water" => InfoType::Water,
            "light" => InfoType::Light,
            "temperature" => InfoType::Temperature,
            "humidity" => InfoType::Humidity,
            "location" => InfoType::Location,
            _ => panic!("Invalid info type: {}", text),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct InfoMapping {
    pub ranges: Vec<(usize, usize, usize)>,
}

impl InfoMapping {
    pub fn add_range(&mut self, dst_range_start: usize, src_range_start: usize, range_len: usize) {
        let new_range = (dst_range_start, src_range_start, range_len);
        match self
            .ranges
            .binary_search_by_key(&src_range_start, |&(_, src_range_start, _)| src_range_start)
        {
            Ok(index) | Err(index) => self.ranges.insert(index, new_range),
        }
    }

    pub fn transform_range(&self, input_range: (usize, usize)) -> Vec<(usize, usize)> {
        let mut transformed_ranges = Vec::new();

        let mut input_range_stack = vec![input_range];

        for (dst_range_start, src_range_start, range_len) in &self.ranges {
            let mut new_inputs = Vec::new();

            while let Some((input_range_start, input_range_len)) = input_range_stack.pop() {
                let input_range_end = input_range_start + input_range_len - 1;
                let src_range_end = src_range_start + range_len - 1;

                // If the input range does not overlap with the source range, add it back to the stack and skip
                if (input_range_end < *src_range_start) || (input_range_start > src_range_end) {
                    new_inputs.push((input_range_start, input_range_len));
                    continue;
                }

                // We have some overlap, so lets calculate the transformed range
                let overlap_start = input_range_start.max(*src_range_start);
                let overlap_end = input_range_end.min(src_range_end);

                let transformed_range_start = dst_range_start + (overlap_start - *src_range_start);
                let transformed_range_len = overlap_end - overlap_start + 1;

                transformed_ranges.push((transformed_range_start, transformed_range_len));

                // If the input range is fully contained within the source range, we're done
                if (input_range_start >= *src_range_start) && (input_range_end <= src_range_end) {
                    continue;
                }

                // If the input range starts before the source range, add the remaining range back to the stack
                if input_range_start < *src_range_start {
                    let remaining_range_start = input_range_start;
                    let remaining_range_len = *src_range_start - input_range_start;

                    new_inputs.push((remaining_range_start, remaining_range_len));
                }

                // If the input range ends after the source range, add the remaining range back to the stack
                if input_range_end > src_range_end {
                    let remaining_range_start = src_range_end + 1;
                    let remaining_range_len = input_range_end - src_range_end;

                    new_inputs.push((remaining_range_start, remaining_range_len));
                }
            }

            // Add the new inputs back to the stack
            input_range_stack.extend(new_inputs);
        }

        // Add any remaining ranges to the transformed ranges
        transformed_ranges.extend(input_range_stack);

        let mut merged_ranges = Self::merge_ranges(transformed_ranges);
        merged_ranges.sort_by_key(|&(start, _)| start);

        merged_ranges
    }

    /// Merge a set of ranges into a minimal set of ranges
    fn merge_ranges(mut ranges: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        if ranges.is_empty() {
            return ranges;
        }

        // Sort the ranges by their start position.
        ranges.sort_by_key(|&(start, _)| start);

        // Initialize the merged ranges with the first range.
        let mut merged_ranges = vec![ranges[0]];

        // Iterate over the rest of the ranges.
        for &(start, length) in &ranges[1..] {
            // Get the last range in the merged ranges.
            let (last_range_start, last_range_length) = merged_ranges.last_mut().unwrap();

            // Calculate the end of the last range.
            let last_range_end = *last_range_start + *last_range_length;

            // If the current range overlaps with the last range, merge them.
            if start <= last_range_end {
                let current_range_end = start + length;
                *last_range_length = last_range_end.max(current_range_end) - *last_range_start;
            } else {
                // If the current range does not overlap, add it to the merged ranges.
                merged_ranges.push((start, length));
            }
        }

        // Return the merged ranges.
        merged_ranges
    }
}

#[cfg(test)]
mod info_type_tests {
    use super::*;

    #[test]
    fn test_info_type_from_str() {
        assert_eq!(InfoType::from_str("seed"), InfoType::Seed);
        assert_eq!(InfoType::from_str("Soil"), InfoType::Soil);
        assert_eq!(InfoType::from_str("Fertilizer"), InfoType::Fertilizer);
        assert_eq!(InfoType::from_str("water"), InfoType::Water);
        assert_eq!(InfoType::from_str("light"), InfoType::Light);
        assert_eq!(InfoType::from_str("temperature"), InfoType::Temperature);
        assert_eq!(InfoType::from_str("humidity"), InfoType::Humidity);
        assert_eq!(InfoType::from_str("location"), InfoType::Location);
    }

    #[test]
    fn test_merge_ranges() {
        // Empty
        assert_eq!(InfoMapping::merge_ranges(vec![]), vec![]);

        // Non-overlapping
        assert_eq!(InfoMapping::merge_ranges(vec![(1, 1)]), vec![(1, 1)]);
        assert_eq!(
            InfoMapping::merge_ranges(vec![(1, 1), (3, 1)]),
            vec![(1, 1), (3, 1)]
        );

        // Overlapping
        assert_eq!(
            InfoMapping::merge_ranges(vec![(1, 1), (2, 1)]),
            vec![(1, 2)]
        );
        assert_eq!(
            InfoMapping::merge_ranges(vec![(1, 1), (2, 2)]),
            vec![(1, 3)]
        );
        assert_eq!(
            InfoMapping::merge_ranges(vec![(1, 2), (3, 1)]),
            vec![(1, 3)]
        );
        assert_eq!(
            InfoMapping::merge_ranges(vec![(1, 2), (2, 4), (3, 1)]),
            vec![(1, 5)]
        );
    }

    #[test]
    fn test_info_mapping_transform_range() {
        let info_mapping = InfoMapping {
            ranges: vec![(1, 0, 1), (11, 1, 1), (20, 2, 2)],
        };

        // Overlapping ranges
        assert_eq!(
            info_mapping.transform_range((0, 1)),
            vec![(1, 1)],
            "Failed to transform [0] to [(1, 1)]"
        );
        assert_eq!(
            info_mapping.transform_range((1, 1)),
            vec![(11, 1)],
            "Failed to transform [1] to [(11, 1)]"
        );
        assert_eq!(
            info_mapping.transform_range((2, 1)),
            vec![(20, 1)],
            "Failed to transform [2] to [(20, 1)]"
        );
        assert_eq!(
            info_mapping.transform_range((3, 1)),
            vec![(21, 1)],
            "Failed to transform [3] to [(21, 1)]"
        );
        assert_eq!(
            info_mapping.transform_range((0, 2)),
            vec![(1, 1), (11, 1)],
            "Failed to transform [0, 1] to [(1, 1), (11, 1)]"
        );
        assert_eq!(
            info_mapping.transform_range((1, 2)),
            vec![(11, 1), (20, 1)],
            "Failed to transform [1, 2] to [(11, 1), (20, 1)]"
        );
        assert_eq!(
            info_mapping.transform_range((2, 2)),
            vec![(20, 2)],
            "Failed to transform [2, 3] to [(20, 2)]"
        );

        // Partially overlapping ranges
        // Ranges should also be merged if possible
        assert_eq!(
            info_mapping.transform_range((0, 5)),
            vec![(1, 1), (4, 1), (11, 1), (20, 2)],
            "Failed to transform [0, 1, 2, 3, 4] to [(1, 1), (4, 1), (11, 1), (20, 2)]"
        );
        assert_eq!(
            info_mapping.transform_range((3, 2)),
            vec![(4, 1), (21, 1)],
            "Failed to transform [3, 4] to [(4, 1), (21, 1)]"
        );

        // Non-overlapping ranges
        assert_eq!(
            info_mapping.transform_range((4, 1)),
            vec![(4, 1)],
            "Failed to transform [4] to [(4, 1)]"
        );
    }
}
