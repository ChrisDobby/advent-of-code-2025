# Design Document

## Overview

The paper roll accessibility analyzer is a command-line Rust application that reads a grid-based warehouse layout from a file and determines which paper rolls can be accessed by forklifts. The core algorithm iterates through each paper roll position, counts adjacent paper rolls, and classifies accessibility based on a threshold of fewer than four adjacent rolls.

The application provides two analysis modes:
1. **Single-pass analysis**: Counts how many paper rolls are currently accessible
2. **Iterative removal analysis**: Simulates repeated removal of accessible rolls until none remain, counting the total number removed

The application follows a simple pipeline architecture: file I/O → parsing → grid analysis → output. The design emphasizes clarity and correctness, with strong type safety provided by Rust's type system.

## Architecture

The system consists of three main layers:

1. **Input Layer**: Handles file reading and error handling for missing or invalid files
2. **Core Logic Layer**: Contains the grid representation and accessibility analysis algorithm
3. **Output Layer**: Formats and displays results to the user

The application uses a single-pass algorithm that processes the grid once, examining each paper roll's neighborhood to determine accessibility.

## Components and Interfaces

### Grid Module

**Purpose**: Represents the warehouse layout and provides methods for querying and modifying positions.

**Key Types**:
- `Grid`: A struct containing a 2D vector of characters and dimensions
  - Fields: `cells: Vec<Vec<char>>`, `rows: usize`, `cols: usize`
  - Methods:
    - `new(input: String) -> Grid`: Constructs a Grid from file contents
    - `is_paper_roll(&self, row: usize, col: usize) -> bool`: Checks if a position contains '@'
    - `count_adjacent_paper_rolls(&self, row: usize, col: usize) -> usize`: Counts paper rolls in the 8 adjacent positions
    - `is_accessible(&self, row: usize, col: usize) -> bool`: Determines if a paper roll at the given position is accessible
    - `remove_roll(&mut self, row: usize, col: usize)`: Replaces a paper roll position with empty space
    - `find_accessible_rolls(&self) -> Vec<(usize, usize)>`: Returns positions of all currently accessible paper rolls

### Analyzer Module

**Purpose**: Orchestrates the analysis process and counts accessible paper rolls.

**Key Functions**:
- `count_accessible_rolls(grid: &Grid) -> usize`: Iterates through all grid positions and counts accessible paper rolls
- `count_removable_rolls(grid: &mut Grid) -> usize`: Performs iterative removal simulation and returns total count of removed rolls
  - Algorithm:
    1. Initialize total_removed counter to 0
    2. Loop until no accessible rolls remain:
       - Find all currently accessible roll positions
       - If none found, break loop
       - Remove all accessible rolls simultaneously
       - Add count to total_removed
    3. Return total_removed

### Main Module

**Purpose**: Entry point that coordinates file I/O and program execution.

**Key Functions**:
- `main()`: Reads input file, creates Grid, runs analysis, outputs result
- `read_input_file(path: &str) -> Result<String, std::io::Error>`: Reads file contents with error handling

## Data Models

### Grid Representation

The grid is stored as a `Vec<Vec<char>>` where:
- Each inner vector represents a row
- Characters are either '@' (paper roll) or '.' (empty space)
- Rows may have different lengths (handled gracefully)

### Position Coordinates

Positions are represented using zero-indexed `(row, col)` tuples of type `(usize, usize)`.

### Adjacent Position Offsets

The eight adjacent positions are defined by offset pairs:
```
(-1, -1)  (-1, 0)  (-1, 1)
( 0, -1)    [X]    ( 0, 1)
( 1, -1)  ( 1, 0)  ( 1, 1)
```

These offsets are applied with bounds checking to avoid out-of-range access.


## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

Based on the requirements analysis, the following properties must hold:

**Property 1: Line parsing preserves row count**
*For any* multi-line input string, the number of rows in the resulting Grid should equal the number of lines in the input.
**Validates: Requirements 1.4**

**Property 2: Character recognition correctness**
*For any* position in the Grid, if the character is '@' then `is_paper_roll` returns true, and if the character is '.' then `is_paper_roll` returns false.
**Validates: Requirements 1.5**

**Property 3: Interior positions examine eight neighbors**
*For any* paper roll position that is not on the edge of the Grid (row > 0, row < rows-1, col > 0, col < cols-1), the system should examine exactly 8 adjacent positions.
**Validates: Requirements 2.1**

**Property 4: Boundary positions stay in bounds**
*For any* paper roll position in the Grid, when counting adjacent paper rolls, the system should never attempt to access positions outside the Grid boundaries.
**Validates: Requirements 2.2**

**Property 5: Only paper rolls are counted as neighbors**
*For any* position in the Grid, when counting adjacent paper rolls, only positions containing '@' should contribute to the count.
**Validates: Requirements 2.4**

**Property 6: Adjacent count matches manual calculation**
*For any* paper roll position in the Grid, the count returned by `count_adjacent_paper_rolls` should equal the number of '@' characters in the valid adjacent positions.
**Validates: Requirements 3.1**

**Property 7: Accessibility threshold correctness**
*For any* paper roll position in the Grid, it should be classified as accessible if and only if it has fewer than 4 adjacent paper rolls.
**Validates: Requirements 3.2, 3.3**

**Property 8: Total count equals sum of accessible rolls**
*For any* Grid, the total count of accessible paper rolls should equal the number of positions where `is_paper_roll` is true AND `is_accessible` is true.
**Validates: Requirements 3.4**

**Property 9: Output is a valid non-negative integer**
*For any* Grid, the output should be parseable as a non-negative integer.
**Validates: Requirements 4.1, 4.2**

**Property 10: Accessible roll identification correctness**
*For any* Grid during iterative removal, all identified accessible rolls should have fewer than 4 adjacent paper rolls at the time of identification.
**Validates: Requirements 5.1**

**Property 11: Removal replaces with empty space**
*For any* position that contains a paper roll, after calling remove_roll on that position, the position should contain an empty space character.
**Validates: Requirements 5.3**

**Property 12: Termination leaves no accessible rolls**
*For any* Grid, when the iterative removal process terminates, the remaining Grid should contain zero accessible paper rolls.
**Validates: Requirements 5.5**

**Property 13: Total removed count is non-decreasing**
*For any* Grid, the total count of removed rolls should be greater than or equal to the count of initially accessible rolls (since removing rolls may expose more accessible rolls).
**Validates: Requirements 5.6**

**Property 14: Iterative removal eventually terminates**
*For any* Grid, the iterative removal process should terminate in a finite number of iterations (bounded by the total number of paper rolls in the initial Grid).
**Validates: Requirements 5.5**

## Error Handling

The system handles the following error conditions:

1. **Missing Input File**: When "input.txt" does not exist, the program prints a clear error message to stderr and exits with a non-zero status code.

2. **File Read Errors**: If the file exists but cannot be read (permissions, I/O errors), the program reports the specific error and exits gracefully.

3. **Empty File**: An empty input file is treated as a valid grid with zero rows and zero paper rolls, resulting in an output of 0.

4. **Malformed Input**: The system is tolerant of varying line lengths and treats any character that is not '@' as empty space.

## Testing Strategy

The testing approach combines unit tests for specific scenarios and property-based tests for general correctness.

### Unit Testing

Unit tests will cover:
- Specific example grids with known expected outputs (including the example from the problem statement)
- The iterative removal example from the problem statement (10x10 grid with expected output of 43 total removable rolls)
- Edge cases: empty grids, single-cell grids, grids with no paper rolls, grids with all paper rolls
- Grids where no rolls are accessible (all should remain)
- Grids where all rolls are accessible in the first iteration
- Corner and edge positions to verify boundary handling
- File I/O error conditions (missing file, empty file)

### Property-Based Testing

Property-based tests will use the **quickcheck** crate for Rust, which is the standard property-based testing library for Rust. Each property-based test will be configured to run a minimum of 100 iterations to ensure thorough coverage of the input space.

Each property-based test will:
- Be tagged with a comment explicitly referencing the correctness property from this design document
- Use the format: `// Feature: paper-roll-accessibility, Property {number}: {property_text}`
- Generate random grids with varying dimensions and paper roll distributions
- Verify the specified property holds across all generated inputs

**Test Generators**:
- `arbitrary_grid()`: Generates random grids with dimensions between 1x1 and 50x50, with random '@' and '.' characters
- `arbitrary_grid_with_paper_rolls()`: Generates grids guaranteed to contain at least one paper roll
- `arbitrary_position(grid)`: Generates valid positions within a given grid's bounds
- `arbitrary_small_grid()`: Generates smaller grids (up to 10x10) for iterative removal tests to ensure reasonable test execution time

The property-based tests will validate that the core logic works correctly across a wide range of inputs, while unit tests will ensure specific edge cases and examples are handled correctly.

For iterative removal testing, property-based tests will verify invariants like termination, correctness of final state, and monotonicity of the removal count, while unit tests will validate the specific example provided in the requirements.
