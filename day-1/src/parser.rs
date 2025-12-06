// Parser module for rotation instructions

use std::fmt;

/// Direction of rotation on the dial
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

/// A single rotation instruction with direction and distance
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rotation {
    pub direction: Direction,
    pub distance: u32,
}

/// Errors that can occur during parsing
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Line contains invalid direction character (not L or R)
    InvalidDirection(String),
    /// Line has direction but no distance value
    MissingDistance,
    /// Distance value is not a valid unsigned integer
    InvalidDistance(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidDirection(line) => {
                write!(f, "Invalid direction in line: '{}'. Expected 'L' or 'R'.", line)
            }
            ParseError::MissingDistance => {
                write!(f, "Missing distance value after direction")
            }
            ParseError::InvalidDistance(value) => {
                write!(f, "Invalid distance value: '{}'. Expected a positive integer.", value)
            }
        }
    }
}

impl std::error::Error for ParseError {}

/// Parse a single rotation instruction line
///
/// Expected format: "[L|R][distance]" where L/R is the direction and distance is a positive integer
///
/// # Examples
///
/// ```
/// # use safe_dial_rotation::parser::{parse_rotation_line, Direction, Rotation};
/// let rotation = parse_rotation_line("R25").unwrap();
/// assert_eq!(rotation.direction, Direction::Right);
/// assert_eq!(rotation.distance, 25);
/// ```
pub fn parse_rotation_line(line: &str) -> Result<Rotation, ParseError> {
    let line = line.trim();

    if line.is_empty() {
        return Err(ParseError::InvalidDirection(line.to_string()));
    }

    // Extract first character as direction
    let first_char = line.chars().next().unwrap();
    let direction = match first_char {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => return Err(ParseError::InvalidDirection(line.to_string())),
    };

    // Extract remaining characters as distance
    let distance_str = &line[1..];

    if distance_str.is_empty() {
        return Err(ParseError::MissingDistance);
    }

    // Parse distance as u32
    let distance = distance_str.parse::<u32>()
        .map_err(|_| ParseError::InvalidDistance(distance_str.to_string()))?;

    Ok(Rotation { direction, distance })
}

/// Parse multiple rotation instruction lines
///
/// Processes a multi-line string, parsing each non-empty line into a Rotation.
/// Empty lines are skipped. Returns an error with line number if parsing fails.
///
/// # Examples
///
/// ```
/// # use safe_dial_rotation::parser::parse_rotations;
/// let input = "R25\nL10\n\nR5";
/// let rotations = parse_rotations(input).unwrap();
/// assert_eq!(rotations.len(), 3);
/// ```
pub fn parse_rotations(input: &str) -> Result<Vec<Rotation>, String> {
    let mut rotations = Vec::new();

    for (line_num, line) in input.lines().enumerate() {
        let trimmed = line.trim();

        // Skip empty lines
        if trimmed.is_empty() {
            continue;
        }

        // Parse the line and add context about line number on error
        match parse_rotation_line(trimmed) {
            Ok(rotation) => rotations.push(rotation),
            Err(e) => {
                return Err(format!("Error on line {}: {}", line_num + 1, e));
            }
        }
    }

    Ok(rotations)
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // Feature: safe-dial-rotation, Property 1: Parsing round trip
    // For any valid rotation instruction string in the format "[L|R][distance]",
    // parsing the string and then formatting it back should preserve the direction and distance values.
    // Validates: Requirements 1.1, 1.2, 1.3, 1.4
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn test_parsing_round_trip(
            direction in prop::bool::ANY,
            distance in 0u32..1000000u32
        ) {
            // Generate a valid rotation string
            let direction_char = if direction { 'R' } else { 'L' };
            let input = format!("{}{}", direction_char, distance);

            // Parse the string
            let parsed = parse_rotation_line(&input).expect("Should parse valid rotation");

            // Verify direction is preserved
            let expected_direction = if direction { Direction::Right } else { Direction::Left };
            assert_eq!(parsed.direction, expected_direction,
                "Direction should be preserved: input={}", input);

            // Verify distance is preserved
            assert_eq!(parsed.distance, distance,
                "Distance should be preserved: input={}", input);
        }
    }
}
