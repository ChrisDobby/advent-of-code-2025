# Implementation Plan

- [x] 1. Set up Rust project structure
  - Initialize Cargo project with appropriate dependencies
  - Add `proptest` crate for property-based testing
  - Create module structure (parser, solver, lib)
  - _Requirements: All_

- [x] 2. Implement core data structures
  - Define `Operation` enum with Add and Multiply variants
  - Define `Problem` struct with numbers and operation fields
  - Define `ParseError` enum for error handling
  - Implement Debug, Clone, PartialEq, Eq traits as needed
  - _Requirements: 1.3, 1.5, 2.4_

- [x] 3. Implement parser module
  - [x] 3.1 Create column extraction logic
    - Write function to transpose input into columns
    - Implement column separator detection (all-whitespace columns)
    - _Requirements: 1.1, 1.2_

  - [x] 3.2 Create number and operation extraction
    - Write function to extract numbers from a column
    - Write function to extract operation symbol from column bottom
    - Handle varying alignments and digit lengths
    - _Requirements: 1.3, 1.4, 4.3_

  - [x] 3.3 Implement main parse_worksheet function
    - Combine column extraction and parsing logic
    - Return Result<Vec<Problem>, ParseError>
    - Handle edge cases (empty input, trailing whitespace, empty lines)
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 1.5, 4.1, 4.2, 4.3, 4.4_

  - [ ]* 3.4 Write property test for column separator detection
    - **Property 1: Column separator detection**
    - **Validates: Requirements 1.1, 1.2**

  - [ ]* 3.5 Write property test for number extraction
    - **Property 2: Complete number extraction**
    - **Validates: Requirements 1.3, 1.4, 4.3**

  - [ ]* 3.6 Write property test for invalid operation handling
    - **Property 5: Invalid operation error handling**
    - **Validates: Requirements 2.4**

- [x] 4. Implement solver module
  - [x] 4.1 Create solve_problem function
    - Implement addition logic for Add operation
    - Implement multiplication logic for Multiply operation
    - Return i64 result
    - _Requirements: 2.1, 2.2, 2.3_

  - [ ]* 4.2 Write property test for addition correctness
    - **Property 3: Addition correctness**
    - **Validates: Requirements 2.1**

  - [ ]* 4.3 Write property test for multiplication correctness
    - **Property 4: Multiplication correctness**
    - **Validates: Requirements 2.2**

- [x] 5. Implement aggregation logic
  - [x] 5.1 Create compute_grand_total function
    - Solve all problems in the list
    - Sum all individual results
    - Return grand total as i64
    - _Requirements: 3.1, 3.2_

  - [ ]* 5.2 Write property test for grand total aggregation
    - **Property 6: Grand total aggregation**
    - **Validates: Requirements 3.1**

- [x] 6. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 7. Implement formatter for round-trip testing
  - [x] 7.1 Create format_problem function
    - Convert Problem back to columnar text format
    - Ensure proper vertical alignment
    - _Requirements: 1.3, 1.5_

  - [ ]* 7.2 Write property test for parse-format round trip
    - **Property 7: Parse-format round trip**
    - **Validates: Requirements 1.3, 1.5**

- [x] 8. Create main application entry point
  - Read input.txt file
  - Parse worksheet
  - Solve all problems
  - Compute and display grand total
  - Handle errors gracefully with user-friendly messages
  - _Requirements: 3.2, 3.3_

- [ ]* 9. Write integration tests
  - Test with the provided example worksheet
  - Test with edge cases (empty file, single problem, large numbers)
  - Verify expected grand total matches actual output
  - _Requirements: All_

- [x] 10. Final Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 11. Implement vertical parsing mode
  - [x] 11.1 Create ParsingMode enum
    - Define Horizontal and Vertical variants
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

  - [x] 11.2 Implement parse_worksheet_vertical function
    - Read each column as a single number (digits stacked vertically)
    - Interpret topmost digit as most significant
    - Group columns right-to-left into problems
    - Handle column separators (whitespace columns)
    - Extract operation symbol from bottom of problem group
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

  - [x] 11.3 Update parse_worksheet to accept ParsingMode parameter
    - Dispatch to parse_worksheet_horizontal or parse_worksheet_vertical based on mode
    - Maintain backward compatibility
    - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

  - [ ]* 11.4 Write property test for vertical digit stacking
    - **Property 8: Vertical digit stacking**
    - **Validates: Requirements 4.1, 4.2**

  - [ ]* 11.5 Write property test for vertical problem grouping
    - **Property 9: Vertical problem grouping**
    - **Validates: Requirements 4.3**

  - [ ]* 11.6 Write property test for vertical mode correctness
    - **Property 10: Vertical mode correctness**
    - **Validates: Requirements 4.4, 4.5**

- [x] 12. Update main application for vertical mode
  - Add command-line argument or configuration to select parsing mode
  - Update main.rs to use vertical mode when specified
  - Test with the provided example
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [ ]* 13. Write integration tests for vertical mode
  - Test with the provided example (expected grand total: 3263827)
  - Test with edge cases (single column, varying digit counts)
  - Verify correct right-to-left problem grouping
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [ ] 14. Final Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
