# Requirements Document

## Introduction

This system analyzes a sequence of dial rotations to determine how many times a safe dial points at position 0 during the rotation sequence. The dial is circular with positions 0-99, and rotations can be left (toward lower numbers) or right (toward higher numbers). The system reads rotation instructions from an input file and counts zero crossings.

## Glossary

- **Dial**: A circular mechanism with 100 positions numbered 0 through 99
- **Rotation**: An instruction consisting of a direction (L or R) and a distance value
- **Position**: The current number the dial is pointing at (0-99)
- **Zero Crossing**: An event where the dial points at position 0 after completing a rotation
- **Parser**: The component that reads and interprets rotation instructions from text format
- **Simulator**: The component that applies rotations to the dial and tracks position changes

## Requirements

### Requirement 1

**User Story:** As a user, I want to parse rotation instructions from a file, so that the system can process the dial movements.

#### Acceptance Criteria

1. WHEN the Parser reads a line containing a rotation instruction THEN the system SHALL extract the direction (L or R) and distance value
2. WHEN the Parser encounters an L prefix THEN the system SHALL interpret it as a left rotation toward lower numbers
3. WHEN the Parser encounters an R prefix THEN the system SHALL interpret it as a right rotation toward higher numbers
4. WHEN the Parser reads a distance value THEN the system SHALL interpret it as the number of clicks to rotate
5. WHEN the Parser processes all lines in the input file THEN the system SHALL produce a sequence of rotation instructions

### Requirement 2

**User Story:** As a user, I want the dial to rotate correctly in both directions, so that positions are calculated accurately.

#### Acceptance Criteria

1. WHEN the Simulator applies a right rotation THEN the system SHALL add the distance to the current position modulo 100
2. WHEN the Simulator applies a left rotation THEN the system SHALL subtract the distance from the current position modulo 100
3. WHEN a rotation causes the position to exceed 99 THEN the system SHALL wrap around to 0
4. WHEN a rotation causes the position to go below 0 THEN the system SHALL wrap around to 99
5. THE Simulator SHALL start with the dial pointing at position 50

### Requirement 3

**User Story:** As a user, I want the system to count zero crossings, so that I can determine the safe password.

#### Acceptance Criteria

1. WHEN the Simulator completes a rotation THEN the system SHALL check if the resulting position equals 0
2. WHEN the dial points at position 0 after a rotation THEN the system SHALL increment the zero crossing counter
3. WHEN all rotations are processed THEN the system SHALL report the total count of zero crossings
4. THE system SHALL not count the initial position as a zero crossing

### Requirement 6

**User Story:** As a user, I want to count all times the dial points at 0 during rotations using method 0x434C49434B, so that I can calculate an alternative password.

#### Acceptance Criteria

1. WHEN the Simulator applies a rotation THEN the system SHALL count every time the dial position passes through 0 during the rotation
2. WHEN a rotation ends at position 0 THEN the system SHALL count that final position as one occurrence
3. WHEN a rotation of distance d causes the dial to wrap around THEN the system SHALL count each complete pass through position 0
4. WHEN calculating passes through 0 for a right rotation from position p with distance d THEN the system SHALL count floor((p + d) / 100) occurrences
5. WHEN calculating passes through 0 for a left rotation from position p with distance d THEN the system SHALL count floor((p + d) / 100) occurrences where the effective distance accounts for wraparound

### Requirement 4

**User Story:** As a user, I want to read rotation instructions from input.txt, so that I can analyze the specific puzzle input.

#### Acceptance Criteria

1. WHEN the system starts THEN the system SHALL attempt to read from a file named input.txt
2. WHEN the input file exists THEN the system SHALL read all rotation instructions from it
3. WHEN the input file does not exist THEN the system SHALL report an error message
4. WHEN the input file contains invalid format THEN the system SHALL report which line caused the error

### Requirement 5

**User Story:** As a user, I want to see the final password value, so that I know the answer to the puzzle.

#### Acceptance Criteria

1. WHEN all rotations are processed THEN the system SHALL display the total count of zero crossings
2. THE system SHALL output the result in a clear, readable format
3. THE system SHALL complete execution after displaying the result

### Requirement 7

**User Story:** As a user, I want to choose between counting methods, so that I can solve both parts of the puzzle.

#### Acceptance Criteria

1. WHEN the system starts THEN the system SHALL support both the original counting method and method 0x434C49434B
2. WHEN using the original method THEN the system SHALL count only zero crossings at the end of rotations
3. WHEN using method 0x434C49434B THEN the system SHALL count all times the dial points at 0 including during rotations
4. THE system SHALL provide a way to select which counting method to use
