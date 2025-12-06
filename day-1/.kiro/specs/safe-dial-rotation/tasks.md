# Implementation Plan

- [x] 1. Set up Rust project structure
  - Create new Rust binary project with cargo
  - Add proptest dependency to Cargo.toml for property-based testing
  - Create module structure: main.rs, parser.rs, simulator.rs
  - _Requirements: All_

- [x] 2. Implement core data types
  - Define Direction enum (Left, Right)
  - Define Rotation struct with direction and distance fields
  - Define ParseError enum with variants for different error types
  - Implement Display trait for error types
  - _Requirements: 1.1, 1.2, 1.3, 1.4, 4.4_

- [x] 3. Implement parser module
  - [x] 3.1 Implement parse_rotation_line function
    - Parse single line into Rotation struct
    - Handle L/R direction prefix
    - Extract and parse distance value
    - Return Result with appropriate ParseError variants
    - _Requirements: 1.1, 1.2, 1.3, 1.4_

  - [x] 3.2 Write property test for parsing round trip
    - **Property 1: Parsing round trip**
    - **Validates: Requirements 1.1, 1.2, 1.3, 1.4**

  - [ ]* 3.3 Write property test for invalid input error reporting
    - **Property 6: Invalid input error reporting**
    - **Validates: Requirements 4.4**

  - [x] 3.4 Implement parse_rotations function
    - Parse multi-line input string
    - Skip empty lines
    - Collect all rotations into Vec
    - Return Result with line number in errors
    - _Requirements: 1.5_

  - [ ]* 3.5 Write property test for multi-line parsing
    - **Property 2: Multi-line parsing completeness**
    - **Validates: Requirements 1.5**

- [x] 4. Implement simulator module
  - [x] 4.1 Implement Dial struct
    - Create new() constructor starting at position 50
    - Implement position() getter
    - _Requirements: 2.5_

  - [x] 4.2 Implement rotate method for Dial
    - Handle right rotation with (position + distance) mod 100
    - Handle left rotation with proper wraparound using rem_euclid
    - Return new position after rotation
    - _Requirements: 2.1, 2.2, 2.3, 2.4_

  - [ ]* 4.3 Write property test for right rotation arithmetic
    - **Property 3: Right rotation arithmetic**
    - **Validates: Requirements 2.1**

  - [ ]* 4.4 Write property test for left rotation arithmetic
    - **Property 4: Left rotation arithmetic**
    - **Validates: Requirements 2.2**

  - [x] 4.5 Implement count_zero_crossings function
    - Create Dial instance
    - Iterate through rotations
    - Apply each rotation and check if position equals 0
    - Count and return total zero crossings
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

  - [ ]* 4.6 Write property test for zero crossing detection
    - **Property 5: Zero crossing detection**
    - **Validates: Requirements 3.1, 3.2, 3.3**

  - [ ]* 4.7 Write unit test for initial position
    - Verify Dial starts at position 50
    - _Requirements: 2.5_

  - [ ]* 4.8 Write unit test for initial position not counted
    - Verify starting position is not counted as zero crossing
    - _Requirements: 3.4_

- [x] 5. Implement main module and file I/O
  - [x] 5.1 Implement read_input_file function
    - Use std::fs::read_to_string to read input.txt
    - Return Result with io::Error on failure
    - _Requirements: 4.1, 4.2, 4.3_

  - [x] 5.2 Implement main function
    - Read input.txt file
    - Parse rotations
    - Count zero crossings
    - Print result to stdout
    - Handle and display errors appropriately
    - _Requirements: 4.1, 4.2, 4.3, 5.1, 5.2, 5.3_

  - [ ]* 5.3 Write unit test for file not found error
    - Verify appropriate error when input.txt doesn't exist
    - _Requirements: 4.3_

  - [ ]* 5.4 Write integration test with example data
    - Test the example sequence from problem statement (should result in 3)
    - Verify end-to-end execution
    - _Requirements: All_

- [x] 6. Final checkpoint
  - Ensure all tests pass, ask the user if questions arise

- [x] 7. Implement method 0x434C49434B for counting all zero passes
  - [x] 7.1 Implement count_zeros_through_rotation helper function
    - Calculate how many times dial passes through 0 during a single rotation
    - Handle right rotations: floor((position + distance) / 100)
    - Handle left rotations: floor((100 - position + distance) / 100)
    - Account for edge cases where rotation ends exactly at 0
    - _Requirements: 6.1, 6.3, 6.4, 6.5_

  - [ ]* 7.2 Write property test for right rotation zero pass counting
    - **Property 7: Right rotation zero pass counting**
    - **Validates: Requirements 6.4**

  - [ ]* 7.3 Write property test for left rotation zero pass counting
    - **Property 8: Left rotation zero pass counting**
    - **Validates: Requirements 6.5**

  - [x] 7.4 Implement count_all_zero_passes function
    - Create Dial instance
    - Iterate through rotations
    - For each rotation, count all passes through 0 during the rotation
    - Sum total passes across all rotations
    - _Requirements: 6.1, 6.2, 6.3, 7.3_

  - [ ]* 7.5 Write property test for method 0x434C49434B
    - **Property 9: Method 0x434C49434B counts all passes**
    - **Validates: Requirements 7.3**

  - [ ]* 7.6 Write property test verifying original method unchanged
    - **Property 10: Original method unchanged**
    - **Validates: Requirements 7.2**

  - [ ]* 7.7 Write unit test with example from problem statement
    - Verify the example sequence results in 6 total passes (3 at end + 3 during)
    - Test individual rotations: L68 from 50 (1 pass), R60 from 95 (1 pass), L82 from 14 (1 pass)
    - _Requirements: 6.1, 6.2, 6.3_

- [x] 8. Update main to support both counting methods
  - [x] 8.1 Add command-line argument or configuration for method selection
    - Support running with original method or method 0x434C49434B
    - Default to method 0x434C49434B for the current puzzle
    - _Requirements: 7.1, 7.4_

  - [x] 8.2 Update main function to call appropriate counting function
    - Call count_zero_crossings for original method
    - Call count_all_zero_passes for method 0x434C49434B
    - Display result with clear indication of which method was used
    - _Requirements: 7.1, 7.2, 7.3_

- [x] 9. Final checkpoint for extended functionality
  - Ensure all tests pass, ask the user if questions arise
