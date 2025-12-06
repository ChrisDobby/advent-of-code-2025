# Implementation Plan

- [x] 1. Set up Rust project structure
  - Initialize a new Rust binary project with `cargo init`
  - Add quickcheck dependency to Cargo.toml for property-based testing
  - Create module structure: main.rs, grid.rs, analyzer.rs
  - _Requirements: All_

- [x] 2. Implement Grid data structure and basic operations
  - [x] 2.1 Create Grid struct with fields for cells, rows, and cols
    - Define the Grid struct in grid.rs
    - Implement constructor that parses input string into 2D vector
    - Handle empty input and varying line lengths
    - _Requirements: 1.3, 1.4, 1.5_

  - [x] 2.2 Implement character recognition methods
    - Write `is_paper_roll` method to check if position contains '@'
    - Add bounds checking for position validity
    - _Requirements: 1.5_

  - [ ]* 2.3 Write property test for line parsing
    - **Property 1: Line parsing preserves row count**
    - **Validates: Requirements 1.4**

  - [ ]* 2.4 Write property test for character recognition
    - **Property 2: Character recognition correctness**
    - **Validates: Requirements 1.5**

- [ ] 3. Implement adjacent position counting logic
  - [x] 3.1 Create method to count adjacent paper rolls
    - Define the 8 direction offsets for adjacent positions
    - Implement `count_adjacent_paper_rolls` with bounds checking
    - Handle edge and corner cases correctly
    - _Requirements: 2.1, 2.2, 2.4, 3.1_

  - [ ]* 3.2 Write property test for interior positions
    - **Property 3: Interior positions examine eight neighbors**
    - **Validates: Requirements 2.1**

  - [ ]* 3.3 Write property test for boundary safety
    - **Property 4: Boundary positions stay in bounds**
    - **Validates: Requirements 2.2**

  - [ ]* 3.4 Write property test for neighbor counting
    - **Property 5: Only paper rolls are counted as neighbors**
    - **Validates: Requirements 2.4**

  - [ ]* 3.5 Write property test for count accuracy
    - **Property 6: Adjacent count matches manual calculation**
    - **Validates: Requirements 3.1**

- [ ] 4. Implement accessibility classification
  - [x] 4.1 Create is_accessible method
    - Implement logic to check if adjacent count < 4
    - Ensure method only applies to paper roll positions
    - _Requirements: 3.2, 3.3_

  - [ ]* 4.2 Write property test for accessibility threshold
    - **Property 7: Accessibility threshold correctness**
    - **Validates: Requirements 3.2, 3.3**

- [ ] 5. Implement analyzer module
  - [x] 5.1 Create count_accessible_rolls function
    - Iterate through all grid positions
    - Count positions that are both paper rolls and accessible
    - Return total count
    - _Requirements: 3.4_

  - [ ]* 5.2 Write property test for total count
    - **Property 8: Total count equals sum of accessible rolls**
    - **Validates: Requirements 3.4**

  - [ ]* 5.3 Write unit test with example from problem statement
    - Use the 10x10 example grid provided
    - Verify output is 13 accessible rolls
    - _Requirements: 3.4_

- [x] 6. Implement file I/O and main function
  - [x] 6.1 Create file reading function
    - Implement `read_input_file` with error handling
    - Handle missing file case with clear error message
    - Handle empty file case
    - _Requirements: 1.1, 1.2, 1.3_

  - [x] 6.2 Implement main function
    - Read from "input.txt"
    - Create Grid from file contents
    - Call analyzer to count accessible rolls
    - Output result as single integer
    - _Requirements: 1.1, 4.1, 4.2_

  - [ ]* 6.3 Write property test for output format
    - **Property 9: Output is a valid non-negative integer**
    - **Validates: Requirements 4.1, 4.2**

  - [ ]* 6.4 Write unit tests for error cases
    - Test missing file error handling
    - Test empty file produces output of 0
    - _Requirements: 1.2, 1.3, 4.3_

- [x] 7. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [x] 8. Implement grid modification methods for iterative removal
  - [x] 8.1 Implement remove_roll method
    - Add `remove_roll(&mut self, row: usize, col: usize)` method to Grid
    - Replace paper roll at position with empty space character
    - _Requirements: 5.3_

  - [ ]* 8.2 Write property test for removal operation
    - **Property 11: Removal replaces with empty space**
    - **Validates: Requirements 5.3**

  - [x] 8.3 Implement find_accessible_rolls method
    - Add `find_accessible_rolls(&self) -> Vec<(usize, usize)>` method to Grid
    - Iterate through all positions and collect those that are accessible paper rolls
    - Return vector of position tuples
    - _Requirements: 5.1_

  - [ ]* 8.4 Write property test for accessible roll identification
    - **Property 10: Accessible roll identification correctness**
    - **Validates: Requirements 5.1**

- [-] 9. Implement iterative removal algorithm
  - [x] 9.1 Create count_removable_rolls function
    - Add `count_removable_rolls(grid: &mut Grid) -> usize` to analyzer module
    - Initialize total_removed counter
    - Loop: find accessible rolls, break if none, remove all, increment counter
    - Return total count of removed rolls
    - _Requirements: 5.1, 5.2, 5.4, 5.5, 5.6_

  - [ ]* 9.2 Write property test for termination condition
    - **Property 12: Termination leaves no accessible rolls**
    - **Validates: Requirements 5.5**

  - [ ]* 9.3 Write property test for termination guarantee
    - **Property 14: Iterative removal eventually terminates**
    - **Validates: Requirements 5.5**

  - [ ]* 9.4 Write property test for removal count monotonicity
    - **Property 13: Total removed count is non-decreasing**
    - **Validates: Requirements 5.6**

  - [ ]* 9.5 Write unit test with iterative removal example
    - Use the 10x10 example grid from the problem statement
    - Verify output is 43 total removable rolls
    - _Requirements: 5.6_

- [x] 10. Update main function to support both analysis modes
  - [x] 10.1 Add command-line argument parsing or dual output
    - Decide on interface: either output both counts or add a flag for mode selection
    - Update main function to call both count_accessible_rolls and count_removable_rolls
    - Output results clearly labeled
    - _Requirements: 4.1, 5.6_

- [x] 11. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
