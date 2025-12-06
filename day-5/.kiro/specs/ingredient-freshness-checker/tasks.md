# Implementation Plan

- [x] 1. Set up Rust project structure
  - Create new Rust project with Cargo
  - Add proptest dependency for property-based testing
  - Create module structure (parser, checker, main)
  - _Requirements: All_

- [x] 2. Implement data models and error types
  - Define FreshRange struct with start and end fields
  - Define InventoryData struct with fresh_ranges and available_ingredients
  - Define ParseError enum with variants for different error cases
  - Implement Display trait for ParseError for human-readable error messages
  - _Requirements: 1.4_

- [x] 3. Implement range parsing logic
  - Write parse_range function to extract start and end from "start-end" format
  - Validate that start <= end during parsing
  - Handle single-value ranges (e.g., "208521390563908-208521390563908")
  - Return ParseError for invalid range formats
  - _Requirements: 1.2, 1.5_

- [ ]* 3.1 Write property test for range parsing
  - **Property 1: Range parsing completeness**
  - **Validates: Requirements 1.1, 1.2**

- [x] 4. Implement full input file parsing
  - Write parse_input function to split input on blank line
  - Parse first section to extract all fresh ranges
  - Parse second section to extract all available ingredient IDs
  - Return ParseError if sections are missing or malformed
  - _Requirements: 1.1, 1.3, 1.4_

- [ ]* 4.1 Write property test for ingredient ID parsing
  - **Property 2: Ingredient ID parsing completeness**
  - **Validates: Requirements 1.3**

- [ ]* 4.2 Write property test for parse error handling
  - **Property 3: Parse error reporting**
  - **Validates: Requirements 1.4**

- [ ]* 4.3 Write unit tests for parsing edge cases
  - Test empty input sections
  - Test single-value ranges
  - Test malformed range formats
  - Test missing blank line separator
  - _Requirements: 1.4, 1.5_

- [x] 5. Implement freshness checking logic
  - Write is_fresh function to check if ingredient ID falls within any range
  - Use inclusive boundary checking (start <= id <= end)
  - Handle overlapping ranges correctly
  - _Requirements: 2.1, 2.2, 2.4_

- [ ]* 5.1 Write property test for freshness classification
  - **Property 4: Freshness classification correctness**
  - **Validates: Requirements 2.1, 2.2**

- [ ]* 5.2 Write unit tests for boundary conditions
  - Test ingredient IDs at range start boundaries
  - Test ingredient IDs at range end boundaries
  - Test ingredient IDs in overlapping range sections
  - _Requirements: 2.2, 2.4_

- [x] 6. Implement fresh ingredient counting
  - Write count_fresh_ingredients function
  - Iterate through available ingredients and count fresh ones
  - Return total count
  - _Requirements: 2.5_

- [ ]* 6.1 Write property test for counting accuracy
  - **Property 5: Fresh count accuracy**
  - **Validates: Requirements 2.5**

- [x] 7. Implement main application flow
  - Read input.txt file
  - Call parse_input to get InventoryData
  - Call count_fresh_ingredients to get result
  - Print result in human-readable format
  - Handle and display errors appropriately
  - _Requirements: 3.1, 3.2_

- [ ]* 7.1 Write integration test with example data
  - Test with the example from problem description (expected: 3 fresh ingredients)
  - Verify correct output format
  - _Requirements: 3.1_

- [ ]* 7.2 Write unit test for zero fresh ingredients case
  - Test with no overlapping ranges and ingredients
  - Verify output shows count of zero
  - _Requirements: 3.3_

- [x] 8. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 9. Implement total fresh range counting functionality
  - Add count_total_fresh_in_ranges function to checker module
  - Use HashSet to collect all unique ingredient IDs across ranges
  - Iterate through each range and add all IDs from start to end inclusive
  - Return the size of the HashSet as the count
  - _Requirements: 4.1, 4.2, 4.3, 4.5_

- [ ]* 9.1 Write property test for total fresh range uniqueness
  - **Property 6: Total fresh range count uniqueness**
  - **Validates: Requirements 4.1, 4.2**

- [ ]* 9.2 Write property test for range expansion completeness
  - **Property 7: Range expansion completeness**
  - **Validates: Requirements 4.3, 4.5**

- [ ]* 9.3 Write unit tests for total fresh range counting
  - Test with non-overlapping ranges
  - Test with overlapping ranges (verify deduplication)
  - Test with adjacent ranges
  - Test with single-value ranges
  - Test with example from problem description (expected: 14 fresh IDs for ranges 3-5, 10-14, 16-20, 12-18)
  - _Requirements: 4.1, 4.2, 4.3_

- [x] 10. Update main application to support total fresh range mode
  - Add command-line argument or mode selection for total fresh range mode
  - Call count_total_fresh_in_ranges when in total fresh range mode
  - Output result with appropriate label
  - _Requirements: 4.4_

- [x] 11. Final Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
