# Design Document

## Overview

The math worksheet parser is a Rust application that reads a specially formatted text file where math problems are arranged in columns. The system supports two parsing modes:

1. **Horizontal Mode**: Complete numbers are stacked vertically in columns (original behavior)
2. **Vertical Mode**: Each number occupies its own column with digits stacked vertically, most significant digit at top

The system parses the columnar structure, identifies individual problems, solves them according to their operation symbols, and computes a grand total.

The solution follows a pipeline architecture: parse → solve → aggregate. This design emphasizes clarity and testability, with each stage having well-defined inputs and outputs.

## Architecture

The system consists of three main stages:

1. **Parsing Stage**: Reads the input file and converts the columnar text format into structured problem representations
2. **Solving Stage**: Evaluates each problem according to its operation
3. **Aggregation Stage**: Sums all individual results to produce the grand total

Data flows unidirectionally through these stages, making the system easy to reason about and test.

## Components and Interfaces

### Problem Structure

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    pub numbers: Vec<i64>,
    pub operation: Operation,
}
```

### Parser Module

```rust
pub enum ParsingMode {
    Horizontal,  // Original: complete numbers stacked vertically
    Vertical,    // Extended: digits stacked vertically, one number per column
}

pub fn parse_worksheet(input: &str, mode: ParsingMode) -> Result<Vec<Problem>, ParseError>
pub fn parse_worksheet_horizontal(input: &str) -> Result<Vec<Problem>, ParseError>
pub fn parse_worksheet_vertical(input: &str) -> Result<Vec<Problem>, ParseError>
```

The parser converts the raw text into a vector of `Problem` instances.

**Horizontal Mode** (original):
- Transposes the input to work with columns
- Identifies column separators (all-whitespace columns)
- Extracts complete numbers and operation symbols from each problem column
- Handles varying number alignments within columns

**Vertical Mode** (extended):
- Transposes the input to work with columns
- Identifies column separators (all-whitespace columns)
- Reads each column as a single number (digits stacked vertically, top = most significant)
- Groups columns right-to-left into problems
- Extracts operation symbol from the bottom of the problem group

### Solver Module

```rust
pub fn solve_problem(problem: &Problem) -> i64
```

The solver evaluates a single problem by applying the operation to all numbers in sequence.

### Aggregator Module

```rust
pub fn compute_grand_total(problems: &[Problem]) -> i64
```

The aggregator sums all individual problem results.

## Data Models

### Problem

Represents a single math problem with:
- `numbers`: A vector of integers to be operated on
- `operation`: The mathematical operation to apply

### Operation

An enum representing supported operations:
- `Add`: Sum all numbers
- `Multiply`: Multiply all numbers together

### ParseError

An error type for parsing failures:
```rust
#[derive(Debug)]
pub enum ParseError {
    InvalidOperation(char),
    EmptyProblem,
    InvalidNumber(String),
}
```


## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

Property 1: Column separator detection
*For any* worksheet with problems separated by all-whitespace columns, the parser should identify exactly the correct number of distinct problems based on these separators.
**Validates: Requirements 1.1, 1.2**

Property 2: Complete number extraction
*For any* valid problem column, parsing should extract all numbers present in the column regardless of their alignment or digit length.
**Validates: Requirements 1.3, 1.4, 4.3**

Property 3: Addition correctness
*For any* problem with addition operation, solving the problem should produce a result equal to the sum of all numbers in the problem.
**Validates: Requirements 2.1**

Property 4: Multiplication correctness
*For any* problem with multiplication operation, solving the problem should produce a result equal to the product of all numbers in the problem.
**Validates: Requirements 2.2**

Property 5: Invalid operation error handling
*For any* problem with an invalid operation symbol, the parser should return an error rather than producing a problem.
**Validates: Requirements 2.4**

Property 6: Grand total aggregation
*For any* list of problems, the grand total should equal the sum of solving each individual problem.
**Validates: Requirements 3.1**

Property 7: Parse-format round trip
*For any* valid problem, formatting it back to the columnar text format and then parsing should produce an equivalent problem.
**Validates: Requirements 1.3, 1.5**

Property 8: Vertical digit stacking
*For any* column in vertical mode, the parser should construct a number where the topmost digit is the most significant and the bottommost digit (excluding operation) is the least significant.
**Validates: Requirements 4.1, 4.2**

Property 9: Vertical problem grouping
*For any* worksheet in vertical mode, problems should be grouped right-to-left with each column representing one number, separated by whitespace columns.
**Validates: Requirements 4.3**

Property 10: Vertical mode correctness
*For any* worksheet parsed in vertical mode, the grand total should equal the sum of all problems where each problem is computed from numbers read as vertical digit sequences.
**Validates: Requirements 4.4, 4.5**

## Error Handling

The system uses Rust's `Result` type for error handling:

- **ParseError::InvalidOperation**: Returned when an unrecognized operation symbol is encountered
- **ParseError::EmptyProblem**: Returned when a problem column contains no numbers
- **ParseError::InvalidNumber**: Returned when a string that should be a number cannot be parsed

The main function propagates errors to the caller, allowing the application to fail fast with clear error messages.

## Testing Strategy

### Unit Testing

Unit tests will verify:
- Individual parsing functions (column splitting, number extraction)
- Operation evaluation for specific examples
- Error cases (invalid operations, malformed input)
- Edge cases (empty input, single problem, single number per problem)

### Property-Based Testing

We will use the `proptest` crate for property-based testing. Each correctness property will be implemented as a property-based test:

- **Property 1**: Generate random worksheets with known problem counts and verify correct parsing
- **Property 2**: Generate random problems with varying alignments and verify complete extraction
- **Property 3**: Generate random addition problems and verify sum correctness
- **Property 4**: Generate random multiplication problems and verify product correctness
- **Property 5**: Generate problems with invalid operations and verify error handling
- **Property 6**: Generate random problem lists and verify grand total equals sum of individual solutions
- **Property 7**: Generate random problems, format them, parse them back, and verify equivalence

Each property test will run a minimum of 100 iterations to ensure robust validation across diverse inputs.

### Test Organization

Tests will be organized as:
- `src/parser.rs` - Parser implementation with inline unit tests
- `src/solver.rs` - Solver implementation with inline unit tests
- `src/lib.rs` - Integration tests and property-based tests
- `tests/` - End-to-end tests with sample worksheets

### Test Tagging

Each property-based test will include a comment tag in this format:
```rust
// Feature: math-worksheet-parser, Property N: [property description]
```

This ensures traceability between design properties and test implementations.
