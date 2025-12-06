// Freshness checker module
// Contains core business logic for determining ingredient freshness

use crate::parser::{FreshRange, InventoryData};

/// Counts the number of fresh ingredients from the available ingredients
///
/// # Arguments
/// * `data` - The InventoryData containing fresh ranges and available ingredients
///
/// # Returns
/// * The total count of fresh ingredients
///
/// # Requirements
/// * Iterates through all available ingredients
/// * Counts ingredients that fall within any fresh range
/// * Requirements: 2.5
pub fn count_fresh_ingredients(data: &InventoryData) -> usize {
    data.available_ingredients
        .iter()
        .filter(|&&ingredient_id| is_fresh(ingredient_id, &data.fresh_ranges))
        .count()
}

/// Checks if an ingredient ID is fresh based on the provided fresh ranges
///
/// # Arguments
/// * `ingredient_id` - The ingredient ID to check
/// * `ranges` - A slice of FreshRange structs defining fresh ingredient ranges
///
/// # Returns
/// * `true` if the ingredient ID falls within any range (inclusive boundaries)
/// * `false` if the ingredient ID does not fall within any range
///
/// # Requirements
/// * Uses inclusive boundary checking: start <= id <= end
/// * Handles overlapping ranges correctly (returns true if in ANY range)
/// * Requirements: 2.1, 2.2, 2.4
pub fn is_fresh(ingredient_id: u64, ranges: &[FreshRange]) -> bool {
    ranges.iter().any(|range| {
        ingredient_id >= range.start && ingredient_id <= range.end
    })
}

/// Counts the total number of unique ingredient IDs across all fresh ranges
///
/// # Arguments
/// * `ranges` - A slice of FreshRange structs defining fresh ingredient ranges
///
/// # Returns
/// * The total count of unique ingredient IDs that fall within any range
///
/// # Requirements
/// * Merges overlapping/adjacent ranges to avoid counting duplicates
/// * Calculates count by summing the size of each merged range
/// * Handles overlapping ranges by deduplicating IDs
/// * Requirements: 4.1, 4.2, 4.3, 4.5
pub fn count_total_fresh_in_ranges(ranges: &[FreshRange]) -> usize {
    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start position
    let mut sorted_ranges: Vec<FreshRange> = ranges.to_vec();
    sorted_ranges.sort_by_key(|r| r.start);

    // Merge overlapping and adjacent ranges
    let mut merged: Vec<FreshRange> = Vec::new();
    let mut current = sorted_ranges[0].clone();

    for range in sorted_ranges.iter().skip(1) {
        // Check if ranges overlap or are adjacent
        // Adjacent means end + 1 == start (e.g., 1-3 and 4-6)
        if range.start <= current.end + 1 {
            // Merge by extending the current range
            current.end = current.end.max(range.end);
        } else {
            // No overlap, save current and start new range
            merged.push(current);
            current = range.clone();
        }
    }
    merged.push(current);

    // Calculate total count by summing the size of each merged range
    merged.iter()
        .map(|range| (range.end - range.start + 1) as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_fresh_within_range() {
        let ranges = vec![
            FreshRange { start: 100, end: 200 },
            FreshRange { start: 300, end: 400 },
        ];

        // Test ingredient within first range
        assert!(is_fresh(150, &ranges));
        // Test ingredient within second range
        assert!(is_fresh(350, &ranges));
    }

    #[test]
    fn test_is_fresh_at_boundaries() {
        let ranges = vec![FreshRange { start: 100, end: 200 }];

        // Test inclusive boundaries (Requirements 2.4)
        assert!(is_fresh(100, &ranges)); // Start boundary
        assert!(is_fresh(200, &ranges)); // End boundary
    }

    #[test]
    fn test_is_fresh_outside_range() {
        let ranges = vec![
            FreshRange { start: 100, end: 200 },
            FreshRange { start: 300, end: 400 },
        ];

        // Test ingredient outside all ranges (Requirements 2.3)
        assert!(!is_fresh(50, &ranges));   // Before first range
        assert!(!is_fresh(250, &ranges));  // Between ranges
        assert!(!is_fresh(500, &ranges));  // After last range
    }

    #[test]
    fn test_is_fresh_overlapping_ranges() {
        let ranges = vec![
            FreshRange { start: 100, end: 200 },
            FreshRange { start: 150, end: 250 }, // Overlaps with first range
        ];

        // Test ingredient in overlapping section (Requirements 2.2)
        assert!(is_fresh(175, &ranges)); // In both ranges
        assert!(is_fresh(125, &ranges)); // Only in first range
        assert!(is_fresh(225, &ranges)); // Only in second range
    }

    #[test]
    fn test_is_fresh_single_value_range() {
        let ranges = vec![FreshRange { start: 42, end: 42 }];

        // Test single-value range (Requirements 1.5)
        assert!(is_fresh(42, &ranges));
        assert!(!is_fresh(41, &ranges));
        assert!(!is_fresh(43, &ranges));
    }

    #[test]
    fn test_is_fresh_empty_ranges() {
        let ranges = vec![];

        // No ranges means nothing is fresh
        assert!(!is_fresh(100, &ranges));
    }

    #[test]
    fn test_count_fresh_ingredients_example() {
        // Example from problem description
        let data = InventoryData {
            fresh_ranges: vec![
                FreshRange { start: 100, end: 200 },
                FreshRange { start: 300, end: 400 },
            ],
            available_ingredients: vec![150, 350, 500],
        };

        // 150 is in range 100-200 (fresh)
        // 350 is in range 300-400 (fresh)
        // 500 is not in any range (spoiled)
        // Expected: 2 fresh ingredients
        assert_eq!(count_fresh_ingredients(&data), 2);
    }

    #[test]
    fn test_count_fresh_ingredients_all_fresh() {
        let data = InventoryData {
            fresh_ranges: vec![FreshRange { start: 1, end: 1000 }],
            available_ingredients: vec![10, 50, 100, 500, 999],
        };

        // All ingredients are within the range
        assert_eq!(count_fresh_ingredients(&data), 5);
    }

    #[test]
    fn test_count_fresh_ingredients_none_fresh() {
        let data = InventoryData {
            fresh_ranges: vec![FreshRange { start: 100, end: 200 }],
            available_ingredients: vec![50, 250, 300],
        };

        // No ingredients are within the range (Requirements 3.3)
        assert_eq!(count_fresh_ingredients(&data), 0);
    }

    #[test]
    fn test_count_fresh_ingredients_at_boundaries() {
        let data = InventoryData {
            fresh_ranges: vec![FreshRange { start: 100, end: 200 }],
            available_ingredients: vec![99, 100, 150, 200, 201],
        };

        // 100 and 200 are at boundaries (inclusive), 150 is inside
        // 99 and 201 are outside
        assert_eq!(count_fresh_ingredients(&data), 3);
    }

    #[test]
    fn test_count_fresh_ingredients_overlapping_ranges() {
        let data = InventoryData {
            fresh_ranges: vec![
                FreshRange { start: 100, end: 200 },
                FreshRange { start: 150, end: 250 },
            ],
            available_ingredients: vec![125, 175, 225, 300],
        };

        // 125 is in first range only
        // 175 is in both ranges (overlapping)
        // 225 is in second range only
        // 300 is in no range
        assert_eq!(count_fresh_ingredients(&data), 3);
    }

    #[test]
    fn test_count_fresh_ingredients_empty_ingredients() {
        let data = InventoryData {
            fresh_ranges: vec![FreshRange { start: 100, end: 200 }],
            available_ingredients: vec![],
        };

        // No ingredients to count
        assert_eq!(count_fresh_ingredients(&data), 0);
    }

    #[test]
    fn test_count_fresh_ingredients_empty_ranges() {
        let data = InventoryData {
            fresh_ranges: vec![],
            available_ingredients: vec![100, 200, 300],
        };

        // No ranges means nothing is fresh
        assert_eq!(count_fresh_ingredients(&data), 0);
    }

    #[test]
    fn test_count_total_fresh_in_ranges_non_overlapping() {
        let ranges = vec![
            FreshRange { start: 3, end: 5 },
            FreshRange { start: 10, end: 14 },
        ];

        // Range 3-5: 3, 4, 5 (3 IDs)
        // Range 10-14: 10, 11, 12, 13, 14 (5 IDs)
        // Total: 8 unique IDs
        assert_eq!(count_total_fresh_in_ranges(&ranges), 8);
    }

    #[test]
    fn test_count_total_fresh_in_ranges_overlapping() {
        let ranges = vec![
            FreshRange { start: 3, end: 5 },
            FreshRange { start: 10, end: 14 },
            FreshRange { start: 16, end: 20 },
            FreshRange { start: 12, end: 18 },
        ];

        // Range 3-5: 3, 4, 5 (3 IDs)
        // Range 10-14: 10, 11, 12, 13, 14 (5 IDs)
        // Range 16-20: 16, 17, 18, 19, 20 (5 IDs)
        // Range 12-18: 12, 13, 14, 15, 16, 17, 18 (7 IDs, overlaps with previous ranges)
        // Unique IDs: 3, 4, 5, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
        // Total: 14 unique IDs (15 bridges the gap between ranges 10-14 and 16-20)
        assert_eq!(count_total_fresh_in_ranges(&ranges), 14);
    }

    #[test]
    fn test_count_total_fresh_in_ranges_adjacent() {
        let ranges = vec![
            FreshRange { start: 1, end: 3 },
            FreshRange { start: 4, end: 6 },
        ];

        // Range 1-3: 1, 2, 3 (3 IDs)
        // Range 4-6: 4, 5, 6 (3 IDs)
        // Total: 6 unique IDs (adjacent but not overlapping)
        assert_eq!(count_total_fresh_in_ranges(&ranges), 6);
    }

    #[test]
    fn test_count_total_fresh_in_ranges_single_value() {
        let ranges = vec![
            FreshRange { start: 42, end: 42 },
            FreshRange { start: 100, end: 100 },
        ];

        // Range 42-42: 42 (1 ID)
        // Range 100-100: 100 (1 ID)
        // Total: 2 unique IDs
        assert_eq!(count_total_fresh_in_ranges(&ranges), 2);
    }

    #[test]
    fn test_count_total_fresh_in_ranges_empty() {
        let ranges = vec![];

        // No ranges means no fresh IDs
        assert_eq!(count_total_fresh_in_ranges(&ranges), 0);
    }

    #[test]
    fn test_count_total_fresh_in_ranges_complete_overlap() {
        let ranges = vec![
            FreshRange { start: 10, end: 20 },
            FreshRange { start: 12, end: 18 },
        ];

        // Range 10-20: 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20 (11 IDs)
        // Range 12-18: completely contained in first range
        // Total: 11 unique IDs
        assert_eq!(count_total_fresh_in_ranges(&ranges), 11);
    }
}
