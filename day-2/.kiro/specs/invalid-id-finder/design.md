# Design Document

## Overview

The Invalid ID Finder is a command-line Rust application that processes ranges of numeric IDs to identify and sum "invalid IDs" - numbers where a digit sequence is repeated at least twice. The system reads comma-separated ranges from an input file, validates each ID within those ranges, and outputs the sum of all invalid IDs found.

The design emphasizes correctness, efficiency, and simplicity. We use Rust's strong type system and error handling to ensure robust parsing and validation.

## Architecture

The system follows a pipeline architecture with three main stages:

1. **Parsing Stage**: Reads the input file and extracts range specifications
2. **Processing Stage**: Iterates through ranges and validates each ID
3. **Aggregation Stage**: Sums all invalid IDs and outputs the result

```
Input File → Parser → Range Iterator → ID Validator → Sum Accumulator → Output
```

The design is functional and stateless, making it easy to test and reason about.

## Components and Interfaces

### 1. Main Module (`main.rs`)

**Responsibility**: Orchestrates the overall flow and handles I/O

**Interface**:
```rust
fn main() -> Result<(), Box<dyn Error>>
```

**Behavior**:
- Reads "input.txt" from the current directory
- Parses ranges from the file content
- Processes each range to find invalid IDs
- Sums and outputs the result

### 2. Range Parser

**Responsibility**: Parses comma-separated range specifications

**Interface**:
```rust
struct Range {
    start: u64,
    end: u64,
}

fn parse_ranges(input: &str) -> Result<Vec<Range>, ParseError>
```

**Behavior**:
- Splits input by commas
- Parses each "start-end" specification
- Returns a vector of Range structs
- Reports errors for malformed ranges

### 3. ID Validator

**Responsibility**: Determines if an ID is invalid (repeated digit sequence)

**Interface**:
```rust
fn is_invalid_id(id: u64) -> bool
```

**Behavior**:
- Converts the ID to a string representation
- Iterates through all possible sequence lengths that evenly divide the total length
- For each sequence length, checks if repeating that sequence recreates the full ID
- Returns true if any repeating pattern is found (sequence repeated at least twice)

### 4. Range Processor

**Responsibility**: Finds all invalid IDs within a range

**Interface**:
```rust
fn find_invalid_ids_in_range(range: &Range) -> Vec<u64>
```

**Behavior**:
- Iterates from range.start to range.end (inclusive)
- Applies is_invalid_id to each number
- Collects and returns all invalid IDs

## Data Models

### Range
```rust
struct Range {
    start: u64,
    end: u64,
}
```

Represents an inclusive range of IDs to process. Uses `u64` to handle large ID values without overflow.

### ParseError
```rust
enum ParseError {
    InvalidFormat(String),
    InvalidNumber(String),
}
```

Represents errors that can occur during parsing.

##
Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system-essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

After analyzing the acceptance criteria, I've identified the following properties that eliminate redundancy and provide comprehensive validation:

**Property 1: Parse round-trip consistency**
*For any* valid Range, formatting it as "start-end" and then parsing should produce an equivalent Range
**Validates: Requirements 1.2**

**Property 2: Whitespace invariance**
*For any* valid range string, adding or removing whitespace should not change the parsed result
**Validates: Requirements 1.3**

**Property 3: Malformed input rejection**
*For any* malformed range string (missing dash, non-numeric values, empty parts), parsing should return an error
**Validates: Requirements 1.4**

**Property 4: Repeated sequence detection**
*For any* digit sequence and any repetition count >= 2, when that sequence is repeated to form an ID, the ID Validator should classify it as invalid
**Validates: Requirements 2.1**

**Property 5: Non-repeated sequence acceptance**
*For any* ID that cannot be formed by repeating any digit sequence at least twice, the ID Validator should classify it as valid
**Validates: Requirements 2.7**

**Property 6: Multiple repetition detection**
*For any* ID like 565656 (three repetitions) or 2121212121 (five repetitions), the ID Validator should classify it as invalid
**Validates: Requirements 2.4, 2.5**

**Property 7: Range completeness**
*For any* range with start and end values, the Range Processor should examine exactly (end - start + 1) IDs
**Validates: Requirements 3.1**

**Property 8: Invalid ID collection**
*For any* range containing known invalid IDs, all those invalid IDs should appear in the results
**Validates: Requirements 3.3**

**Property 9: Sum correctness**
*For any* collection of IDs, the computed sum should equal the mathematical sum of all values
**Validates: Requirements 4.1**

## Error Handling

The system uses Rust's `Result` type for error handling:

1. **File I/O Errors**: If "input.txt" cannot be read, the program exits with a clear error message
2. **Parse Errors**: If range specifications are malformed, the program reports which specification failed and why
3. **Overflow Protection**: Using `u64` for IDs and `u128` for sums prevents overflow for realistic inputs

Error messages should be descriptive and include context about what failed.

## Testing Strategy

We will use a dual testing approach combining unit tests and property-based tests:

### Unit Tests

Unit tests will verify specific examples and edge cases:
- Parsing specific range formats ("11-22", "1-1000000")
- Known invalid IDs (11, 22, 1010, 123123, 446446)
- Empty input handling
- Single range vs. multiple ranges
- Edge case: range with start == end

### Property-Based Tests

We will use the `proptest` crate for property-based testing. Each property-based test should run a minimum of 100 iterations.

Property-based tests will verify universal properties:
- **Property 1**: Parse round-trip - generate random Ranges, format and parse them
- **Property 2**: Whitespace invariance - generate ranges with random whitespace
- **Property 3**: Malformed input rejection - generate invalid range strings
- **Property 4**: Repeated sequence detection - generate random digit sequences and repeat them 2+ times
- **Property 5**: Non-repeated sequence acceptance - generate random non-repeated IDs
- **Property 6**: Multiple repetition detection - generate sequences repeated 3, 4, 5+ times
- **Property 7**: Range completeness - generate random ranges and count processed IDs
- **Property 8**: Invalid ID collection - generate ranges with known invalid IDs
- **Property 9**: Sum correctness - generate random ID collections and verify sum

Each property-based test must be tagged with a comment in this format:
```rust
// **Feature: invalid-id-finder, Property N: [property description]**
```

Each correctness property must be implemented by a single property-based test.

## Performance Considerations

For large ranges, iterating through every ID could be slow. However, for this problem:
- Most ranges in the input are reasonably sized
- The validation logic is O(log n) where n is the ID value (based on digit count)
- Rust's performance makes this approach practical

If performance becomes an issue, we could optimize by:
- Mathematically generating invalid IDs within a range instead of checking each ID
- Using parallel processing for independent ranges

## Implementation Notes

1. Use `u64` for individual IDs to handle large values
2. Use `u128` for the final sum to prevent overflow
3. The `is_invalid_id` function should convert the ID to a string and check if it can be formed by repeating any substring at least twice
4. Parsing should be tolerant of whitespace but strict about format
5. All functions should be pure (no side effects) except for I/O in main
