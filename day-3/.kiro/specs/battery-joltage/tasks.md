# Implementation Plan

- [x] 1. Set up Rust project structure
  - Create new Rust binary project with `cargo init`
  - Add `proptest` dependency to `Cargo.toml` for property-based testing
  - Set up basic project structure with `lib.rs` and `main.rs`
  - _Requirements: All_

- [x] 2. Implement core data structures and error types
  - Define `BatteryBank` struct with `Vec<u8>` for storing batteries
  - Define error enums: `ParseError`, `JoltageError`, `ProcessingError`
  - Implement `Display` and `Error` traits for error types
  - _Requirements: 1.4, 1.5, 2.3_

- [x] 3. Implement battery bank parsing
  - Write `BatteryBank::from_line()` to parse a line into a battery bank
  - Validate that lines contain only digit characters
  - Handle empty and whitespace-only lines appropriately
  - _Requirements: 1.1, 1.2, 1.3, 1.5_

- [ ]* 3.1 Write property test for line parsing
  - **Property 1: Line parsing correctness**
  - **Validates: Requirements 1.1, 1.2, 1.3**

- [ ]* 3.2 Write property test for invalid input rejection
  - **Property 4: Invalid input rejection**
  - **Validates: Requirements 1.5**

- [ ]* 3.3 Write unit tests for parsing edge cases
  - Test empty lines, whitespace-only lines
  - Test lines with non-digit characters
  - Test valid digit-only lines
  - _Requirements: 1.1, 1.2, 1.3, 1.5_

- [x] 4. Implement maximum joltage calculation
  - Write `BatteryBank::find_max_joltage()` method
  - Iterate through all pairs (i, j) where i < j
  - Calculate joltage as `batteries[i] * 10 + batteries[j]`
  - Return maximum joltage found
  - Handle error case for banks with fewer than 2 batteries
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

- [ ]* 4.1 Write property test for maximum joltage correctness
  - **Property 2: Maximum joltage correctness**
  - **Validates: Requirements 2.1, 2.2, 2.5**

- [ ]* 4.2 Write unit tests for joltage calculation
  - Test the four examples from problem statement (987654321111111 → 98, etc.)
  - Test banks with all same digits
  - Test banks with ascending/descending digits
  - Test error case with single battery
  - Test error case with empty bank
  - _Requirements: 2.1, 2.2, 2.3, 2.5_

- [x] 5. Implement file parsing
  - Write `parse_input_file()` function to read file line by line
  - Use buffered I/O for efficiency
  - Skip empty/whitespace lines
  - Collect all battery banks or return first error
  - Handle file-not-found errors
  - _Requirements: 1.1, 1.3, 1.4_

- [ ]* 5.1 Write unit tests for file parsing
  - Test with non-existent file path
  - Test with empty file
  - Test with file containing multiple valid banks
  - Test with file containing mix of valid and empty lines
  - _Requirements: 1.1, 1.3, 1.4_

- [x] 6. Implement total joltage calculation
  - Write `calculate_total_joltage()` function
  - Process each bank and sum maximum joltages
  - Handle banks that produce errors (skip and continue)
  - Collect results and errors for reporting
  - _Requirements: 3.1, 3.3_

- [ ]* 6.1 Write property test for total joltage summation
  - **Property 3: Total joltage is sum of maximums**
  - **Validates: Requirements 3.1**

- [ ]* 6.2 Write property test for error handling with mixed banks
  - **Property 5: Error handling for valid banks with invalid banks**
  - **Validates: Requirements 3.3**

- [ ]* 6.3 Write unit tests for aggregation
  - Test with all valid banks
  - Test with mix of valid and invalid banks
  - Test with empty collection of banks
  - _Requirements: 3.1, 3.3_

- [x] 7. Implement main application logic
  - Write `main()` function to orchestrate the pipeline
  - Read input file path from command-line arguments or use default "input.txt"
  - Call parsing and calculation functions
  - Display individual bank results and total joltage
  - Handle and display errors appropriately
  - Set exit codes: 0 for success, non-zero for errors
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [ ]* 7.1 Write integration tests
  - Create test input files with known results
  - Test end-to-end execution with valid input
  - Test end-to-end execution with invalid input
  - Verify exit codes
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [x] 8. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 9. Test with actual input.txt file
  - Run the application against the provided input.txt
  - Verify output is correct
  - Document the final answer
  - _Requirements: All_

- [x] 10. Implement n-battery joltage calculation
  - Add `find_max_joltage_n(&self, n: usize) -> Result<u64, JoltageError>` method to `BatteryBank`
  - Implement greedy algorithm: for each result position, find the largest digit we can place while leaving enough batteries for remaining positions
  - Return error if bank has fewer than n batteries
  - Use u64 for return type to handle 12-digit numbers
  - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [ ]* 10.1 Write property test for n-battery maximum correctness
  - **Property 6: N-battery maximum joltage correctness**
  - **Validates: Requirements 6.1, 6.2**

- [ ]* 10.2 Write property test for n-battery insufficient batteries
  - **Property 7: N-battery insufficient batteries error**
  - **Validates: Requirements 6.3**

- [ ]* 10.3 Write property test for n-battery result structure
  - **Property 8: N-battery result structure**
  - **Validates: Requirements 6.4**

- [ ]* 10.4 Write unit tests for n-battery calculation
  - Test the four examples from problem statement with n=12
  - Test with n=2 to verify it matches original algorithm
  - Test edge cases: n equals bank length, n=1
  - Test error case with insufficient batteries
  - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [x] 11. Add command-line option for n-battery mode
  - Add optional command-line argument to specify number of batteries to select
  - Default to 2 for backward compatibility
  - Parse and validate the argument
  - Update main() to use the specified n value
  - _Requirements: 6.1, 6.2, 6.3, 6.4_

- [ ]* 11.1 Write unit tests for command-line parsing
  - Test with no arguments (should default to 2)
  - Test with valid n argument
  - Test with invalid n argument (non-numeric, zero, negative)
  - _Requirements: 6.1_

- [x] 12. Update output formatting for large numbers
  - Modify display logic to handle u64 values
  - Ensure proper formatting of 12-digit numbers
  - Update result structures to support both u32 and u64
  - _Requirements: 6.4_

- [x] 13. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 14. Test with actual input.txt file using 12-battery mode
  - Run the application with n=12 against the provided input.txt
  - Verify output matches expected result (3121910778619)
  - Document the final answer
  - _Requirements: 6.1, 6.2, 6.4_
