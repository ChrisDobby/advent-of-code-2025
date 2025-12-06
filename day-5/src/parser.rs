// Parser module for ingredient freshness checker
// Handles parsing of input file format

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FreshRange {
    pub start: u64,
    pub end: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InventoryData {
    pub fresh_ranges: Vec<FreshRange>,
    pub available_ingredients: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    InvalidFormat(String),
    InvalidNumber(String),
    MissingSection(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ParseError::InvalidNumber(msg) => write!(f, "Invalid number: {}", msg),
            ParseError::MissingSection(msg) => write!(f, "Missing section: {}", msg),
        }
    }
}

impl std::error::Error for ParseError {}

/// Parses the complete input file into InventoryData
///
/// # Arguments
/// * `content` - The full input file content as a string
///
/// # Returns
/// * `Ok(InventoryData)` - Successfully parsed data with ranges and ingredients
/// * `Err(ParseError)` - Missing sections or malformed data
///
/// # Format
/// The input must have two sections separated by a blank line:
/// 1. Fresh ranges (one per line, format "start-end")
/// 2. Available ingredient IDs (one per line)
pub fn parse_input(content: &str) -> Result<InventoryData, ParseError> {
    // Split on double newline (blank line separator)
    let sections: Vec<&str> = content.split("\n\n").collect();

    // Must have exactly 2 sections
    if sections.len() != 2 {
        return Err(ParseError::MissingSection(
            format!("Input must have exactly 2 sections separated by a blank line, found {} sections", sections.len())
        ));
    }

    // Parse first section: fresh ranges
    let ranges_section = sections[0].trim();
    if ranges_section.is_empty() {
        return Err(ParseError::MissingSection(
            "First section (fresh ranges) is empty".to_string()
        ));
    }

    let mut fresh_ranges = Vec::new();
    for (line_num, line) in ranges_section.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue; // Skip empty lines within section
        }

        match parse_range(line) {
            Ok(range) => fresh_ranges.push(range),
            Err(e) => {
                return Err(ParseError::InvalidFormat(
                    format!("Line {}: {}", line_num + 1, e)
                ));
            }
        }
    }

    // Parse second section: available ingredient IDs
    let ingredients_section = sections[1].trim();
    if ingredients_section.is_empty() {
        return Err(ParseError::MissingSection(
            "Second section (available ingredients) is empty".to_string()
        ));
    }

    let mut available_ingredients = Vec::new();
    for (line_num, line) in ingredients_section.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue; // Skip empty lines within section
        }

        match line.parse::<u64>() {
            Ok(id) => available_ingredients.push(id),
            Err(_) => {
                return Err(ParseError::InvalidNumber(
                    format!("Line {} in ingredients section: invalid ingredient ID '{}'",
                            line_num + 1, line)
                ));
            }
        }
    }

    Ok(InventoryData {
        fresh_ranges,
        available_ingredients,
    })
}

/// Parses a range string in the format "start-end" into a FreshRange
///
/// # Arguments
/// * `line` - A string slice containing the range in "start-end" format
///
/// # Returns
/// * `Ok(FreshRange)` - Successfully parsed range with start <= end
/// * `Err(ParseError)` - Invalid format or start > end
///
/// # Examples
/// * "100-200" -> FreshRange { start: 100, end: 200 }
/// * "42-42" -> FreshRange { start: 42, end: 42 } (single-value range)
pub fn parse_range(line: &str) -> Result<FreshRange, ParseError> {
    let line = line.trim();

    // Split on the dash separator
    let parts: Vec<&str> = line.split('-').collect();

    // Must have exactly 2 parts (start and end)
    if parts.len() != 2 {
        return Err(ParseError::InvalidFormat(
            format!("Range must be in format 'start-end', got: '{}'", line)
        ));
    }

    // Parse start value
    let start = parts[0].trim().parse::<u64>()
        .map_err(|_| ParseError::InvalidNumber(
            format!("Invalid start value: '{}'", parts[0])
        ))?;

    // Parse end value
    let end = parts[1].trim().parse::<u64>()
        .map_err(|_| ParseError::InvalidNumber(
            format!("Invalid end value: '{}'", parts[1])
        ))?;

    // Validate that start <= end
    if start > end {
        return Err(ParseError::InvalidFormat(
            format!("Range start ({}) must be <= end ({})", start, end)
        ));
    }

    Ok(FreshRange { start, end })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range_valid() {
        let result = parse_range("100-200");
        assert!(result.is_ok());
        let range = result.unwrap();
        assert_eq!(range.start, 100);
        assert_eq!(range.end, 200);
    }

    #[test]
    fn test_parse_range_single_value() {
        let result = parse_range("208521390563908-208521390563908");
        assert!(result.is_ok());
        let range = result.unwrap();
        assert_eq!(range.start, 208521390563908);
        assert_eq!(range.end, 208521390563908);
    }

    #[test]
    fn test_parse_range_with_whitespace() {
        let result = parse_range("  100 - 200  ");
        assert!(result.is_ok());
        let range = result.unwrap();
        assert_eq!(range.start, 100);
        assert_eq!(range.end, 200);
    }

    #[test]
    fn test_parse_range_invalid_start_greater_than_end() {
        let result = parse_range("200-100");
        assert!(result.is_err());
        match result {
            Err(ParseError::InvalidFormat(msg)) => {
                assert!(msg.contains("start"));
                assert!(msg.contains("end"));
            }
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_parse_range_invalid_format_no_dash() {
        let result = parse_range("100");
        assert!(result.is_err());
        match result {
            Err(ParseError::InvalidFormat(_)) => {}
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_parse_range_invalid_format_too_many_dashes() {
        let result = parse_range("100-200-300");
        assert!(result.is_err());
        match result {
            Err(ParseError::InvalidFormat(_)) => {}
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_parse_range_invalid_number() {
        let result = parse_range("abc-200");
        assert!(result.is_err());
        match result {
            Err(ParseError::InvalidNumber(_)) => {}
            _ => panic!("Expected InvalidNumber error"),
        }
    }

    #[test]
    fn test_parse_range_invalid_end_number() {
        let result = parse_range("100-xyz");
        assert!(result.is_err());
        match result {
            Err(ParseError::InvalidNumber(_)) => {}
            _ => panic!("Expected InvalidNumber error"),
        }
    }

    #[test]
    fn test_parse_input_valid() {
        let input = "100-200\n300-400\n\n150\n350\n500";
        let result = parse_input(input);
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.fresh_ranges.len(), 2);
        assert_eq!(data.fresh_ranges[0].start, 100);
        assert_eq!(data.fresh_ranges[0].end, 200);
        assert_eq!(data.fresh_ranges[1].start, 300);
        assert_eq!(data.fresh_ranges[1].end, 400);

        assert_eq!(data.available_ingredients.len(), 3);
        assert_eq!(data.available_ingredients[0], 150);
        assert_eq!(data.available_ingredients[1], 350);
        assert_eq!(data.available_ingredients[2], 500);
    }

    #[test]
    fn test_parse_input_single_value_range() {
        let input = "208521390563908-208521390563908\n\n208521390563908";
        let result = parse_input(input);
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.fresh_ranges.len(), 1);
        assert_eq!(data.fresh_ranges[0].start, 208521390563908);
        assert_eq!(data.fresh_ranges[0].end, 208521390563908);

        assert_eq!(data.available_ingredients.len(), 1);
        assert_eq!(data.available_ingredients[0], 208521390563908);
    }

    #[test]
    fn test_parse_input_missing_blank_line() {
        let input = "100-200\n300-400\n150\n350";
        let result = parse_input(input);
        assert!(result.is_err());
        match result {
            Err(ParseError::MissingSection(_)) => {}
            _ => panic!("Expected MissingSection error"),
        }
    }

    #[test]
    fn test_parse_input_empty_ranges_section() {
        let input = "\n\n150\n350";
        let result = parse_input(input);
        assert!(result.is_err());
        match result {
            Err(ParseError::MissingSection(msg)) => {
                assert!(msg.contains("First section"));
            }
            _ => panic!("Expected MissingSection error for first section"),
        }
    }

    #[test]
    fn test_parse_input_empty_ingredients_section() {
        let input = "100-200\n300-400\n\n";
        let result = parse_input(input);
        assert!(result.is_err());
        match result {
            Err(ParseError::MissingSection(msg)) => {
                assert!(msg.contains("Second section"));
            }
            _ => panic!("Expected MissingSection error for second section"),
        }
    }

    #[test]
    fn test_parse_input_malformed_range() {
        let input = "100-200\ninvalid-range\n\n150\n350";
        let result = parse_input(input);
        assert!(result.is_err());
        match result {
            Err(ParseError::InvalidFormat(_)) => {}
            _ => panic!("Expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_parse_input_malformed_ingredient() {
        let input = "100-200\n300-400\n\n150\nabc\n350";
        let result = parse_input(input);
        assert!(result.is_err());
        match result {
            Err(ParseError::InvalidNumber(_)) => {}
            _ => panic!("Expected InvalidNumber error"),
        }
    }

    #[test]
    fn test_parse_input_with_whitespace() {
        let input = "  100-200  \n  300-400  \n\n  150  \n  350  ";
        let result = parse_input(input);
        assert!(result.is_ok());

        let data = result.unwrap();
        assert_eq!(data.fresh_ranges.len(), 2);
        assert_eq!(data.available_ingredients.len(), 2);
    }
}
