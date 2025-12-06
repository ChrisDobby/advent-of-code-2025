# Implementation Plan

- [x] 1. Set up Rust project structure
  - Initialize a new Rust binary project with `cargo init`
  - Add `proptest` dependency to Cargo.toml for property-based testing
  - Create basic project structure with main.rs
  - _Requirements: All_

- [x] 2. Implement core data models and parsing
  - Define the `Range` struct with start and end fields (u64)
  - Define the `ParseError` enum for error handling
  - Implement `Display` trait for `ParseError` for clear error messages
  - _Requirements: 1.1, 1.2, 1.4_

- [x] 2.1 Implement range parsing function
  - Write `parse_ranges` function that takes a string and returns `Result<Vec<Range>, ParseError>`
  - Handle comma-separated ranges
  - Parse "start-end" format for each range
  - Trim whitespace from range specifications
  - _Requirements: 1.1, 1.2, 1.3_

- [x] 2.2 Write property test for parse round-trip
  - **Property 1: Parse round-trip consistency**
  - **Validates: Requirements 1.2**

- [ ]* 2.3 Write property test for whitespace invariance
  - **Property 2: Whitespace invariance**
  - **Validates: Requirements 1.3**

- [ ]* 2.4 Write property test for malformed input rejection
  - **Property 3: Malformed input rejection**
  - **Validates: Requirements 1.4**

- [x] 3. Implement ID validation logic
  - Write `is_invalid_id` function that takes a u64 and returns bool
  - Convert ID to string representation
  - Iterate through all possible sequence lengths that evenly divide the total length
  - For each sequence length, check if repeating that sequence recreates the full ID
  - Return true if any repeating pattern is found (at least 2 repetitions)
  - _Requirements: 2.1, 2.3, 2.4, 2.5, 2.7, 6.1, 6.2, 6.3, 6.4, 6.5_

- [ ]* 3.1 Write property test for repeated sequence detection
  - **Property 4: Repeated sequence detection**
  - **Validates: Requirements 2.1**

- [ ]* 3.2 Write property test for non-repeated sequence acceptance
  - **Property 5: Non-repeated sequence acceptance**
  - **Validates: Requirements 2.7**

- [ ]* 3.3 Write property test for multiple repetition detection
  - **Property 6: Multiple repetition detection**
  - **Validates: Requirements 2.4, 2.5**

- [ ]* 3.4 Write unit tests for known invalid IDs
  - Test that 11, 22, 33, 44, 55, 66, 77, 88, 99 are invalid
  - Test that 1010, 123123, 446446, 12341234, 1111111 are invalid
  - Test that 565656 (three repetitions), 824824824 (three repetitions), 2121212121 (five repetitions) are invalid
  - Test that 101, 123, 456 are valid
  - _Requirements: 2.2, 2.3, 2.4, 2.5_

- [x] 4. Implement range processing
  - Write `find_invalid_ids_in_range` function that takes a Range reference and returns Vec<u64>
  - Iterate from range.start to range.end inclusive
  - Apply `is_invalid_id` to each ID
  - Collect all invalid IDs into a vector
  - _Requirements: 3.1, 3.3, 3.4_

- [ ]* 4.1 Write property test for range completeness
  - **Property 7: Range completeness**
  - **Validates: Requirements 3.1**

- [ ]* 4.2 Write property test for invalid ID collection
  - **Property 8: Invalid ID collection**
  - **Validates: Requirements 3.3**

- [ ]* 4.3 Write unit test for empty range result
  - Test a range with no invalid IDs returns empty vector
  - _Requirements: 3.4_

- [x] 5. Implement sum calculation and main program flow
  - Write main function that reads "input.txt"
  - Parse ranges from file content
  - Process each range to find invalid IDs
  - Sum all invalid IDs using u128 to prevent overflow
  - Output the sum to standard output
  - Handle file I/O errors with clear error messages
  - _Requirements: 4.1, 4.3, 4.4, 5.1, 5.2, 5.3, 5.4_

- [ ]* 5.1 Write property test for sum correctness
  - **Property 9: Sum correctness**
  - **Validates: Requirements 4.1**

- [ ]* 5.2 Write unit test for zero sum case
  - Test that when no invalid IDs are found, output is 0
  - _Requirements: 4.4_

- [x] 6. Integration testing and validation
  - Test with the provided example input to verify correct output
  - Verify the example produces sum of 4174379265 (updated for new rules)
  - Test with the actual input.txt file
  - _Requirements: All_

- [x] 7. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
