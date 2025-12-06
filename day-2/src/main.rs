use std::fmt;

/// Represents an inclusive range of IDs to process
#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    /// Formats the range as "start-end" string
    #[cfg(test)]
    fn format(&self) -> String {
        format!("{}-{}", self.start, self.end)
    }
}

/// Represents errors that can occur during parsing
#[derive(Debug, Clone, PartialEq, Eq)]
enum ParseError {
    InvalidFormat(String),
    InvalidNumber(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ParseError::InvalidNumber(msg) => write!(f, "Invalid number: {}", msg),
        }
    }
}

/// Parses comma-separated range specifications from input string
///
/// # Arguments
/// * `input` - A string containing comma-separated ranges in "start-end" format
///
/// # Returns
/// * `Ok(Vec<Range>)` - Successfully parsed ranges
/// * `Err(ParseError)` - Error if any range specification is malformed
///
/// # Examples
/// ```
/// let ranges = parse_ranges("1-10, 20-30").unwrap();
/// assert_eq!(ranges.len(), 2);
/// ```
fn parse_ranges(input: &str) -> Result<Vec<Range>, ParseError> {
    let mut ranges = Vec::new();

    // Split by commas and process each range specification
    for range_spec in input.split(',') {
        let trimmed = range_spec.trim();

        // Skip empty specifications
        if trimmed.is_empty() {
            continue;
        }

        // Split by dash to get start and end
        let parts: Vec<&str> = trimmed.split('-').collect();

        if parts.len() != 2 {
            return Err(ParseError::InvalidFormat(
                format!("Range '{}' must be in 'start-end' format", trimmed)
            ));
        }

        // Parse start value
        let start = parts[0].trim().parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber(
                format!("Cannot parse start value '{}' as number", parts[0].trim())
            ))?;

        // Parse end value
        let end = parts[1].trim().parse::<u64>()
            .map_err(|_| ParseError::InvalidNumber(
                format!("Cannot parse end value '{}' as number", parts[1].trim())
            ))?;

        ranges.push(Range { start, end });
    }

    Ok(ranges)
}

/// Determines if an ID is invalid (composed of a digit sequence repeated at least twice)
///
/// # Arguments
/// * `id` - The ID to validate
///
/// # Returns
/// * `true` if the ID is invalid (repeated sequence)
/// * `false` if the ID is valid
///
/// # Examples
/// ```
/// assert!(is_invalid_id(11));         // "11" -> "1" repeated 2 times
/// assert!(is_invalid_id(123123));     // "123123" -> "123" repeated 2 times
/// assert!(is_invalid_id(565656));     // "565656" -> "56" repeated 3 times
/// assert!(is_invalid_id(1111111));    // "1111111" -> "1" repeated 7 times
/// assert!(!is_invalid_id(123));       // "123" cannot be formed by repeating any sequence
/// assert!(!is_invalid_id(1234));      // "1234" cannot be formed by repeating any sequence
/// ```
fn is_invalid_id(id: u64) -> bool {
    // Convert ID to string representation
    let id_str = id.to_string();
    let len = id_str.len();

    // Iterate through all possible sequence lengths that evenly divide the total length
    // We need at least 2 repetitions, so sequence length can be at most len/2
    for seq_len in 1..=(len / 2) {
        // Check if this sequence length evenly divides the total length
        if len % seq_len == 0 {
            // Extract the first sequence
            let sequence = &id_str[..seq_len];

            // Check if repeating this sequence recreates the full ID
            let repetitions = len / seq_len;
            let reconstructed = sequence.repeat(repetitions);

            if reconstructed == id_str {
                // Found a repeating pattern with at least 2 repetitions
                return true;
            }
        }
    }

    // No repeating pattern found
    false
}

/// Finds all invalid IDs within a given range
///
/// # Arguments
/// * `range` - A reference to a Range specifying the inclusive interval to process
///
/// # Returns
/// * `Vec<u64>` - A vector containing all invalid IDs found in the range
///
/// # Examples
/// ```
/// let range = Range { start: 10, end: 25 };
/// let invalid_ids = find_invalid_ids_in_range(&range);
/// // Should include 11, 22 from this range
/// ```
fn find_invalid_ids_in_range(range: &Range) -> Vec<u64> {
    let mut invalid_ids = Vec::new();

    // Iterate from range.start to range.end inclusive
    for id in range.start..=range.end {
        // Apply is_invalid_id to each ID
        if is_invalid_id(id) {
            // Collect all invalid IDs into a vector
            invalid_ids.push(id);
        }
    }

    invalid_ids
}

fn main() {
    use std::fs;
    use std::process;

    // Read input.txt from the current directory
    let input_content = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input.txt: {}", e);
            process::exit(1);
        }
    };

    // Parse ranges from file content
    let ranges = match parse_ranges(&input_content) {
        Ok(ranges) => ranges,
        Err(e) => {
            eprintln!("Error parsing ranges: {}", e);
            process::exit(1);
        }
    };

    // Process each range to find invalid IDs and sum them
    let mut sum: u128 = 0;

    for range in &ranges {
        let invalid_ids = find_invalid_ids_in_range(range);
        for id in invalid_ids {
            sum += id as u128;
        }
    }

    // Output the sum to standard output
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_single_range() {
        let result = parse_ranges("1-10").unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].start, 1);
        assert_eq!(result[0].end, 10);
    }

    #[test]
    fn test_parse_multiple_ranges() {
        let result = parse_ranges("1-10, 20-30, 100-200").unwrap();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], Range { start: 1, end: 10 });
        assert_eq!(result[1], Range { start: 20, end: 30 });
        assert_eq!(result[2], Range { start: 100, end: 200 });
    }

    #[test]
    fn test_parse_with_whitespace() {
        let result = parse_ranges("  1 - 10  ,  20 - 30  ").unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], Range { start: 1, end: 10 });
        assert_eq!(result[1], Range { start: 20, end: 30 });
    }

    #[test]
    fn test_parse_invalid_format_no_dash() {
        let result = parse_ranges("1 10");
        assert!(result.is_err());
        match result {
            Err(ParseError::InvalidFormat(msg)) => {
                assert!(msg.contains("must be in 'start-end' format"));
            }
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_parse_invalid_format_too_many_dashes() {
        let result = parse_ranges("1-10-20");
        assert!(result.is_err());
        match result {
            Err(ParseError::InvalidFormat(_)) => {}
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_parse_invalid_number() {
        let result = parse_ranges("abc-10");
        assert!(result.is_err());
        match result {
            Err(ParseError::InvalidNumber(msg)) => {
                assert!(msg.contains("Cannot parse"));
            }
            _ => panic!("Expected InvalidNumber error"),
        }
    }

    #[test]
    fn test_parse_error_display() {
        let err1 = ParseError::InvalidFormat("test format".to_string());
        assert_eq!(format!("{}", err1), "Invalid format: test format");

        let err2 = ParseError::InvalidNumber("test number".to_string());
        assert_eq!(format!("{}", err2), "Invalid number: test number");
    }

    #[test]
    fn test_is_invalid_id_single_digits() {
        // Single digit repeated: 11, 22, 33, etc.
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(22));
        assert!(is_invalid_id(33));
        assert!(is_invalid_id(44));
        assert!(is_invalid_id(55));
        assert!(is_invalid_id(66));
        assert!(is_invalid_id(77));
        assert!(is_invalid_id(88));
        assert!(is_invalid_id(99));
    }

    #[test]
    fn test_is_invalid_id_multi_digit_sequences() {
        // Multi-digit sequences repeated twice
        assert!(is_invalid_id(1010));
        assert!(is_invalid_id(123123));
        assert!(is_invalid_id(446446));
        assert!(is_invalid_id(12341234));
    }

    #[test]
    fn test_is_invalid_id_multiple_repetitions() {
        // Sequences repeated more than twice
        assert!(is_invalid_id(565656));      // "56" repeated 3 times
        assert!(is_invalid_id(824824824));   // "824" repeated 3 times
        assert!(is_invalid_id(2121212121));  // "21" repeated 5 times
        assert!(is_invalid_id(1111111));     // "1" repeated 7 times
    }

    #[test]
    fn test_is_invalid_id_valid_ids() {
        // Odd length IDs are valid
        assert!(!is_invalid_id(101));
        assert!(!is_invalid_id(123));
        assert!(!is_invalid_id(456));

        // Even length but not repeated
        assert!(!is_invalid_id(12));
        assert!(!is_invalid_id(1234));
        assert!(!is_invalid_id(5678));
    }

    #[test]
    fn test_is_invalid_id_edge_cases() {
        // Single digit is valid (odd length)
        assert!(!is_invalid_id(1));
        assert!(!is_invalid_id(9));

        // Zero is valid (single digit)
        assert!(!is_invalid_id(0));
    }

    #[test]
    fn test_find_invalid_ids_in_range_basic() {
        // Range containing 11 and 22
        let range = Range { start: 10, end: 25 };
        let invalid_ids = find_invalid_ids_in_range(&range);
        assert_eq!(invalid_ids, vec![11, 22]);
    }

    #[test]
    fn test_find_invalid_ids_in_range_no_invalid() {
        // Range with no invalid IDs
        let range = Range { start: 1, end: 10 };
        let invalid_ids = find_invalid_ids_in_range(&range);
        assert!(invalid_ids.is_empty());
    }

    #[test]
    fn test_find_invalid_ids_in_range_single_id() {
        // Range with start == end, and it's invalid
        let range = Range { start: 11, end: 11 };
        let invalid_ids = find_invalid_ids_in_range(&range);
        assert_eq!(invalid_ids, vec![11]);
    }

    // Property-based tests
    use proptest::prelude::*;

    // **Feature: invalid-id-finder, Property 1: Parse round-trip consistency**
    // **Validates: Requirements 1.2**
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn prop_parse_roundtrip(start in 0u64..1_000_000u64, end in 0u64..1_000_000u64) {
            // Ensure start <= end for valid range
            let (start, end) = if start <= end { (start, end) } else { (end, start) };

            let original = Range { start, end };
            let formatted = original.format();
            let parsed = parse_ranges(&formatted).expect("Should parse successfully");

            // Should parse to exactly one range
            prop_assert_eq!(parsed.len(), 1);

            // The parsed range should equal the original
            prop_assert_eq!(&parsed[0], &original);
        }
    }

    // Integration tests
    #[test]
    fn test_integration_example_input() {
        // Test with the provided example input (same as input.txt)
        // The correct sum with updated rules (detecting 2+ repetitions) is 11323661261
        let example_input = "199617-254904,7682367-7856444,17408-29412,963327-1033194,938910234-938964425,3207382-3304990,41-84,61624-105999,1767652-1918117,492-749,85-138,140-312,2134671254-2134761843,2-23,3173-5046,16114461-16235585,3333262094-3333392446,779370-814446,26-40,322284296-322362264,6841-12127,290497-323377,33360-53373,823429-900127,17753097-17904108,841813413-841862326,518858-577234,654979-674741,773-1229,2981707238-2981748769,383534-468118,587535-654644,1531-2363";

        // Parse ranges
        let ranges = parse_ranges(example_input).expect("Should parse example input successfully");

        // Process each range to find invalid IDs and sum them
        let mut sum: u128 = 0;
        for range in &ranges {
            let invalid_ids = find_invalid_ids_in_range(range);
            for id in invalid_ids {
                sum += id as u128;
            }
        }

        // Verify the expected sum
        assert_eq!(sum, 11323661261, "Example input should produce sum of 11323661261");
    }

    #[test]
    fn test_integration_actual_input_file() {
        use std::fs;

        // Test with the actual input.txt file
        let input_content = fs::read_to_string("input.txt")
            .expect("Should be able to read input.txt for integration test");

        // Parse ranges
        let ranges = parse_ranges(&input_content)
            .expect("Should parse input.txt successfully");

        // Verify we got some ranges
        assert!(!ranges.is_empty(), "input.txt should contain at least one range");

        // Process each range to find invalid IDs and sum them
        let mut sum: u128 = 0;
        for range in &ranges {
            let invalid_ids = find_invalid_ids_in_range(range);
            for id in invalid_ids {
                sum += id as u128;
            }
        }

        // Verify the expected sum matches the computed value
        // The correct sum for input.txt with updated rules (detecting 2+ repetitions) is 11323661261
        assert_eq!(sum, 11323661261, "input.txt should produce sum of 11323661261");
    }

    #[test]
    fn test_integration_small_example() {
        // Test with a small, manually verifiable example
        let input = "10-25, 100-110";

        let ranges = parse_ranges(input).expect("Should parse small example");

        let mut sum: u128 = 0;
        for range in &ranges {
            let invalid_ids = find_invalid_ids_in_range(range);
            for id in invalid_ids {
                sum += id as u128;
            }
        }

        // Range 10-25 contains: 11, 22
        // Range 100-110 contains: none (101, 102, ... 110 are all valid)
        // Expected sum: 11 + 22 = 33
        assert_eq!(sum, 33, "Small example should produce sum of 33");
    }

    #[test]
    fn test_integration_no_invalid_ids() {
        // Test with ranges that contain no invalid IDs
        let input = "1-10, 12-21, 23-32";

        let ranges = parse_ranges(input).expect("Should parse input");

        let mut sum: u128 = 0;
        for range in &ranges {
            let invalid_ids = find_invalid_ids_in_range(range);
            for id in invalid_ids {
                sum += id as u128;
            }
        }

        // No invalid IDs in these ranges
        assert_eq!(sum, 0, "Should produce sum of 0 when no invalid IDs found");
    }
}
