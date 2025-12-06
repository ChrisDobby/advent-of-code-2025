# Requirements Document

## Introduction

This system parses and solves a math worksheet where problems can be arranged in two different formats:

1. **Horizontal Format (Original)**: Problems are arranged vertically in columns, with complete numbers stacked vertically and an operation symbol at the bottom. Problems are separated by full columns of spaces.

2. **Vertical Format (Extended)**: Each number is given in its own column, with the most significant digit at the top and the least significant digit at the bottom. Problems are read right-to-left, one column at a time. Problems are still separated by columns consisting only of spaces, and the operation symbol at the bottom indicates the operator to use.

The system must parse the worksheet in the specified format, solve each problem, and compute the grand total by summing all individual problem answers.

## Glossary

- **Worksheet**: A text file containing math problems arranged in columns
- **Problem**: A set of numbers with an operation symbol
- **Operation Symbol**: A mathematical operator (+, *) that appears at the bottom of each problem
- **Column Separator**: A full vertical column containing only whitespace that separates adjacent problems
- **Grand Total**: The sum of all individual problem answers
- **Parser**: The system component that reads and interprets the worksheet format
- **Horizontal Format**: Problems where complete numbers are stacked vertically in columns
- **Vertical Format**: Problems where each number occupies its own column with digits stacked vertically (most significant digit at top)
- **Parsing Mode**: The format interpretation mode (horizontal or vertical) used by the Parser

## Requirements

### Requirement 1

**User Story:** As a user, I want to parse a worksheet file, so that I can extract individual math problems from the columnar format.

#### Acceptance Criteria

1. WHEN the Parser reads a worksheet file THEN the system SHALL identify all problem columns by detecting column separators
2. WHEN the Parser encounters a column with only spaces THEN the system SHALL treat it as a problem separator
3. WHEN the Parser identifies a problem column THEN the system SHALL extract all numbers and the operation symbol from that column
4. WHEN the Parser reads numbers within a column THEN the system SHALL ignore left/right alignment differences
5. WHEN the Parser completes parsing THEN the system SHALL produce a list of problems where each problem contains its numbers and operation

### Requirement 2

**User Story:** As a user, I want to solve each parsed problem, so that I can compute individual answers.

#### Acceptance Criteria

1. WHEN the system processes a problem with addition operation THEN the system SHALL sum all numbers in that problem
2. WHEN the system processes a problem with multiplication operation THEN the system SHALL multiply all numbers in that problem
3. WHEN the system solves a problem THEN the system SHALL produce a numeric result for that problem
4. WHEN the system encounters an invalid operation symbol THEN the system SHALL report an error with the problem location

### Requirement 3

**User Story:** As a user, I want to compute the grand total, so that I can verify the worksheet solution.

#### Acceptance Criteria

1. WHEN all individual problems are solved THEN the system SHALL sum all individual answers to produce the grand total
2. WHEN the system computes the grand total THEN the system SHALL output the final result
3. WHEN the system completes processing THEN the system SHALL display both individual problem results and the grand total

### Requirement 4

**User Story:** As a user, I want to parse worksheets in vertical format, so that I can solve problems where digits are arranged vertically within columns.

#### Acceptance Criteria

1. WHEN the Parser operates in vertical mode THEN the system SHALL read each column as a single number with digits stacked vertically
2. WHEN the Parser reads a column in vertical mode THEN the system SHALL interpret the topmost digit as the most significant digit
3. WHEN the Parser identifies problems in vertical mode THEN the system SHALL group columns right-to-left into problems separated by whitespace columns
4. WHEN the Parser encounters the operation symbol in vertical mode THEN the system SHALL apply it to all numbers in that problem group
5. WHEN the Parser processes a problem in vertical mode THEN the system SHALL construct multi-digit numbers from vertical digit sequences

### Requirement 5

**User Story:** As a user, I want the system to handle edge cases gracefully, so that I can process various worksheet formats.

#### Acceptance Criteria

1. WHEN the worksheet contains empty lines THEN the system SHALL handle them without errors
2. WHEN the worksheet contains trailing whitespace THEN the system SHALL process it correctly
3. WHEN a problem column contains numbers of varying digit lengths THEN the system SHALL parse all numbers correctly
4. WHEN the input file is empty THEN the system SHALL report zero as the grand total
