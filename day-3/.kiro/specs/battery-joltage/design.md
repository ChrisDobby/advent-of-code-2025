# Design Document

## Overview

The battery joltage calculator is a command-line application written in Rust that processes battery bank data to find optimal power configurations. The system reads a text file containing battery banks (sequences of digits), and can calculate maximum joltage in two modes:

1. **Two-battery mode**: Selects exactly two batteries from each bank to maximize the two-digit joltage
2. **Twelve-battery mode**: Selects exactly twelve batteries from each bank to maximize the twelve-digit joltage

The solution emphasizes simplicity, correctness, and efficiency. The core algorithm examines valid battery combinations within each bank to find the maximum joltage, using straightforward iteration rather than complex optimization techniques.

## Architecture

The application follows a simple pipeline architecture:

```
Input File → Parser → Joltage Calculator → Aggregator → Output
```

**Components:**
1. **File Reader**: Reads input file line by line
2. **Parser**: Validates and converts lines into battery bank representations
3. **Joltage Calculator**: Finds maximum joltage for each bank
4. **Aggregator**: Sums individual bank results
5. **Output Formatter**: Displays results and handles errors

The design uses functional composition where each component transforms data and passes it to the next stage. Error handling uses Rust's `Result` type to propagate errors through the pipeline.

## Components and Interfaces

### BatteryBank

Represents a single bank of batteries as a sequence of digits.

```rust
struct BatteryBank {
    batteries: Vec<u8>, // Each u8 represents a single digit (0-9)
}

impl BatteryBank {
    fn from_line(line: &str) -> Result<Self, ParseError>;
    fn find_max_joltage(&self) -> Result<u32, JoltageError>;
    fn find_max_joltage_n(&self, n: usize) -> Result<u64, JoltageError>;
}
```

**Responsibilities:**
- Validate that input contains only digits
- Store battery sequence efficiently
- Calculate maximum joltage by examining all valid pairs (2-battery mode)
- Calculate maximum joltage by examining all valid n-battery combinations (n-battery mode)

### Parser

Converts raw input into structured battery bank data.

```rust
fn parse_input_file(path: &Path) -> Result<Vec<BatteryBank>, ParseError>;
```

**Responsibilities:**
- Read file line by line
- Skip empty/whitespace lines
- Create BatteryBank instances
- Report parsing errors with line numbers

### JoltageCalculator

Core algorithm for finding maximum joltage.

```rust
fn calculate_max_joltage(batteries: &[u8]) -> Result<u32, JoltageError>;
fn calculate_max_joltage_n(batteries: &[u8], n: usize) -> Result<u64, JoltageError>;
```

**Algorithm (2-battery mode):**
- Iterate through all pairs (i, j) where i < j
- For each pair, compute joltage = batteries[i] * 10 + batteries[j]
- Track maximum value
- Return error if fewer than 2 batteries

**Time Complexity:** O(n²) where n is the number of batteries in a bank

**Algorithm (n-battery mode):**
- Use a greedy approach: scan left-to-right, selecting batteries that maximize the result
- At each position, choose to include or exclude the current battery based on whether it improves the final number
- Strategy: Skip smaller digits at the beginning to make room for larger digits
- Track maximum value as we build the n-digit number
- Return error if fewer than n batteries

**Time Complexity:** O(n) where n is the number of batteries in a bank

**Greedy Strategy Details:**
The key insight is that we want the largest possible digits in the leftmost positions. We can achieve this by:
1. For each position in our result (from left to right), look ahead to find the largest digit we can place there
2. Ensure we leave enough batteries remaining to fill the rest of the positions
3. Once we select a battery, we can only select from batteries to its right

### Aggregator

Combines results from all banks.

```rust
fn calculate_total_joltage(banks: &[BatteryBank]) -> Result<u32, ProcessingError>;
```

**Responsibilities:**
- Process each bank sequentially
- Sum maximum joltages
- Collect and report errors

### Error Types

```rust
enum ParseError {
    FileNotFound(PathBuf),
    InvalidCharacter { line: usize, character: char },
    IoError(std::io::Error),
}

enum JoltageError {
    InsufficientBatteries { count: usize },
}

enum ProcessingError {
    ParseError(ParseError),
    JoltageError { bank_index: usize, error: JoltageError },
}
```

## Data Models

### BatteryBank

```rust
struct BatteryBank {
    batteries: Vec<u8>,
}
```

- `batteries`: Vector of digits (0-9) representing individual battery joltage ratings
- Invariant: All values must be in range 0-9

### ProcessingResult

```rust
struct ProcessingResult {
    bank_results: Vec<BankResult>,
    total_joltage: u32,
}

struct BankResult {
    bank_index: usize,
    max_joltage: u32,
    selected_indices: (usize, usize),
}
```

##
Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Line parsing correctness

*For any* input file content, the number of parsed battery banks should equal the number of non-empty, non-whitespace lines, and each bank should contain only the digit characters from its corresponding line.

**Validates: Requirements 1.1, 1.2, 1.3**

### Property 2: Maximum joltage correctness

*For any* battery bank with at least two batteries, the maximum joltage returned should be greater than or equal to the joltage produced by any pair of batteries (i, j) where i < j, calculated as batteries[i] * 10 + batteries[j].

**Validates: Requirements 2.1, 2.2, 2.5**

### Property 3: Total joltage is sum of maximums

*For any* collection of valid battery banks, the total output joltage should equal the sum of the maximum joltage from each individual bank.

**Validates: Requirements 3.1**

### Property 4: Invalid input rejection

*For any* input line containing at least one non-digit character (excluding whitespace), the parser should return an error identifying that line.

**Validates: Requirements 1.5**

### Property 5: Error handling for valid banks with invalid banks

*For any* collection of battery banks where some banks are valid and some are invalid (fewer than 2 batteries), the total joltage should equal the sum of only the valid banks' maximum joltages.

**Validates: Requirements 3.3**

### Property 6: N-battery maximum joltage correctness

*For any* battery bank with at least n batteries, the maximum n-digit joltage returned should be greater than or equal to the joltage produced by any valid selection of n batteries that maintains their original order.

**Validates: Requirements 6.1, 6.2**

### Property 7: N-battery insufficient batteries error

*For any* battery bank with fewer than n batteries, calling the n-battery joltage function should return an error indicating insufficient batteries.

**Validates: Requirements 6.3**

### Property 8: N-battery result structure

*For any* battery bank with at least n batteries, the n-digit joltage result should form a valid n-digit number where the selected batteries maintain their relative order from the original bank.

**Validates: Requirements 6.4**

## Error Handling

The system uses Rust's `Result` type for error propagation. Errors are categorized into:

1. **Parse Errors**: File not found, invalid characters, I/O errors
2. **Joltage Errors**: Insufficient batteries in a bank
3. **Processing Errors**: Wrapper for errors during the full pipeline

Error messages include:
- File path for file-not-found errors
- Line number and invalid character for parsing errors
- Bank index for joltage calculation errors

The system continues processing remaining banks when individual banks fail, collecting all errors for final reporting.

## Testing Strategy

The testing approach combines unit tests for specific scenarios with property-based tests for comprehensive validation.

### Unit Testing

Unit tests will cover:
- Specific examples from the problem statement (the four example banks)
- Edge cases: empty files, single-battery banks, all-zero banks
- Error conditions: missing files, invalid characters
- Boundary values: maximum digit values (9), minimum values (0)

### Property-Based Testing

We will use the `proptest` crate for property-based testing in Rust. Each property-based test will:
- Run a minimum of 100 iterations with randomly generated inputs
- Be tagged with a comment referencing the corresponding correctness property
- Use the format: `// Feature: battery-joltage, Property {number}: {property_text}`

Property-based tests will verify:
1. **Parsing properties**: Generate random valid/invalid input strings and verify parsing behavior
2. **Maximum joltage properties**: Generate random battery banks and verify the result is truly maximal
3. **Aggregation properties**: Generate collections of banks and verify sum correctness
4. **Error handling properties**: Generate inputs with various error conditions and verify appropriate handling

Each correctness property listed above will be implemented by a single property-based test.

### Test Organization

Tests will be organized as:
- `src/lib.rs`: Core library functions with inline unit tests in `#[cfg(test)]` modules
- `tests/property_tests.rs`: Property-based tests using proptest
- `tests/integration_tests.rs`: End-to-end tests with sample input files

## Implementation Notes

### Performance Considerations

- The O(n²) algorithm for finding maximum joltage is acceptable given typical bank sizes
- File reading uses buffered I/O to handle large files efficiently
- No need for parallel processing given the simplicity of the problem

### Rust-Specific Design Choices

- Use `&str` for parsing to avoid unnecessary allocations
- Use `Vec<u8>` for battery storage (compact and efficient)
- Leverage iterator chains for functional composition
- Use `?` operator for clean error propagation

### Future Enhancements

Potential improvements not in current scope:
- Parallel processing of banks for very large files
- Support for different input formats (CSV, JSON)
- Optimization to O(n) algorithm if needed
- Detailed statistics (average joltage, distribution, etc.)
