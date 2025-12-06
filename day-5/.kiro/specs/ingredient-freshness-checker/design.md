# Design Document

## Overview

The ingredient freshness checker is a command-line Rust application that processes inventory data to determine which ingredient IDs are fresh based on defined ranges. The system supports two operational modes:

1. **Available Ingredient Mode**: Checks which ingredient IDs from a provided list are fresh
2. **Total Fresh Range Mode**: Calculates the total count of all unique ingredient IDs that the fresh ranges define as fresh

The application follows a simple pipeline architecture: parse input → check freshness → output results. It prioritizes correctness and efficiency, using appropriate data structures for fast range lookups and set operations for deduplication.

## Architecture

The system consists of three main layers:

1. **Input Layer**: Handles file reading and parsing of the input format
2. **Domain Layer**: Contains core business logic for freshness checking
3. **Output Layer**: Formats and displays results

The application follows a functional programming style where possible, with clear separation between parsing, processing, and output stages.

## Components and Interfaces

### Parser Module

**Responsibility**: Parse the input file into structured data

**Key Types**:
```rust
struct FreshRange {
    start: u64,
    end: u64,
}

struct InventoryData {
    fresh_ranges: Vec<FreshRange>,
    available_ingredients: Vec<u64>,
}
```

**Key Functions**:
- `parse_input(content: &str) -> Result<InventoryData, ParseError>`: Parses the entire input file
- `parse_range(line: &str) -> Result<FreshRange, ParseError>`: Parses a single range line
- `parse_ingredient_id(line: &str) -> Result<u64, ParseError>`: Parses a single ingredient ID

### Freshness Checker Module

**Responsibility**: Determine which ingredients are fresh

**Key Functions**:
- `count_fresh_ingredients(data: &InventoryData) -> usize`: Counts fresh ingredients from available list
- `is_fresh(ingredient_id: u64, ranges: &[FreshRange]) -> bool`: Checks if a single ingredient is fresh
- `count_total_fresh_in_ranges(ranges: &[FreshRange]) -> usize`: Counts all unique ingredient IDs across all ranges

### Main Module

**Responsibility**: Orchestrate the application flow

**Key Functions**:
- `main() -> Result<(), Box<dyn Error>>`: Entry point that coordinates parsing, checking, and output
- `read_input_file(path: &str) -> Result<String, io::Error>`: Reads the input file

## Data Models

### FreshRange
Represents an inclusive range of fresh ingredient IDs.

```rust
struct FreshRange {
    start: u64,  // Inclusive lower bound
    end: u64,    // Inclusive upper bound
}
```

**Invariants**:
- `start <= end` (enforced during parsing)

### InventoryData
Contains all parsed data from the input file.

```rust
struct InventoryData {
    fresh_ranges: Vec<FreshRange>,
    available_ingredients: Vec<u64>,
}
```

### ParseError
Custom error type for parsing failures.

```rust
enum ParseError {
    InvalidFormat(String),
    InvalidNumber(String),
    MissingSection(String),
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*


### Property 1: Range parsing completeness
*For any* valid input file with N range definitions in the first section, parsing should extract exactly N ranges with correct start and end values.
**Validates: Requirements 1.1, 1.2**

### Property 2: Ingredient ID parsing completeness
*For any* valid input file with M ingredient IDs in the second section, parsing should extract exactly M ingredient IDs with correct values.
**Validates: Requirements 1.3**

### Property 3: Parse error reporting
*For any* malformed input (invalid number format, missing sections, invalid range format), parsing should return an error rather than succeed.
**Validates: Requirements 1.4**

### Property 4: Freshness classification correctness
*For any* ingredient ID and set of fresh ranges (including overlapping ranges), the ingredient should be classified as fresh if and only if it falls within at least one range's inclusive bounds.
**Validates: Requirements 2.1, 2.2**

### Property 5: Fresh count accuracy
*For any* set of ingredient IDs and fresh ranges, the count of fresh ingredients should equal the number of ingredients that fall within at least one range.
**Validates: Requirements 2.5**

### Property 6: Total fresh range count uniqueness
*For any* set of fresh ranges (including overlapping ranges), the total count of fresh IDs should equal the number of unique IDs across all ranges, with no duplicates.
**Validates: Requirements 4.1, 4.2**

### Property 7: Range expansion completeness
*For any* fresh range with start S and end E, the total fresh count should include all IDs from S to E inclusive.
**Validates: Requirements 4.3, 4.5**

## Error Handling

The system uses Rust's `Result` type for error handling:

- **Parse Errors**: Return `ParseError` with descriptive messages for invalid input
- **IO Errors**: Propagate `std::io::Error` for file reading failures
- **Main Error**: Use `Box<dyn Error>` for flexible error handling in main

Error messages should include:
- Line numbers for parsing errors
- Specific details about what was invalid
- Context about what was expected

## Testing Strategy

The system will use a dual testing approach combining unit tests and property-based tests.

### Unit Testing

Unit tests will cover:
- Specific example cases from the problem description
- Edge cases: empty input sections, single-value ranges, boundary values
- Error conditions: malformed ranges, invalid numbers, missing blank line
- Integration: end-to-end test with the example input

Unit tests provide concrete examples that demonstrate correct behavior and catch specific bugs.

### Property-Based Testing

Property-based tests will use the `proptest` crate for Rust. Each property test will:
- Run a minimum of 100 iterations with randomly generated inputs
- Be tagged with a comment referencing the design document property
- Use the format: `// Feature: ingredient-freshness-checker, Property N: [property text]`

Property tests verify universal properties across all inputs:
- **Property 1**: Generate random valid input files, verify all ranges parsed
- **Property 2**: Generate random ingredient ID lists, verify all IDs parsed
- **Property 3**: Generate various malformed inputs, verify errors returned
- **Property 4**: Generate random ingredients and ranges, verify correct classification
- **Property 5**: Generate random data, verify count matches manual calculation

Each correctness property will be implemented by a single property-based test. Property tests complement unit tests by verifying general correctness across the input space, while unit tests catch concrete bugs in specific scenarios.

## Implementation Notes

### Parsing Strategy

The parser will:
1. Split input on double newline to separate sections
2. Parse first section line-by-line for ranges
3. Parse second section line-by-line for ingredient IDs
4. Validate that ranges have start <= end

### Freshness Checking Strategy

**Available Ingredient Mode**:
For efficiency with large datasets:
- Iterate through each available ingredient once
- For each ingredient, check against all ranges until a match is found
- Count matches as we go

**Total Fresh Range Mode**:
For calculating all unique fresh IDs:
- Use a HashSet to track unique ingredient IDs
- Iterate through each range and add all IDs from start to end
- Return the size of the set

Optimization considerations:
- For very large ranges, consider merging overlapping/adjacent ranges first
- Use interval merging algorithm to reduce redundant counting
- Sort ranges and merge before expansion for better performance

### File Structure

```
ingredient-freshness-checker/
├── Cargo.toml
├── input.txt
└── src/
    ├── main.rs          # Entry point and orchestration
    ├── parser.rs        # Input parsing logic
    └── checker.rs       # Freshness checking logic
```
