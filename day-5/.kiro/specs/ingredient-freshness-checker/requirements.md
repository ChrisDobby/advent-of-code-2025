# Requirements Document

## Introduction

This system processes an inventory management database to determine ingredient freshness based on defined freshness ranges. The system supports two modes of operation:

1. **Available Ingredient Mode**: Determines which ingredient IDs from a list of available ingredients are considered fresh based on defined freshness ranges
2. **Total Fresh Range Mode**: Calculates the total count of all unique ingredient IDs that the fresh ranges consider to be fresh, regardless of availability

The system reads input data containing inclusive ID ranges that define fresh ingredients and optionally a list of available ingredient IDs.

## Glossary

- **Ingredient ID**: A unique numeric identifier for an ingredient in the inventory system
- **Fresh Range**: An inclusive range of ingredient IDs (format: start-end) where all IDs within the range are considered fresh
- **Available Ingredient**: An ingredient ID from the inventory that needs to be checked for freshness
- **System**: The ingredient freshness checker application

## Requirements

### Requirement 1

**User Story:** As an inventory manager, I want to parse the input file format, so that I can extract fresh ranges and available ingredient IDs for processing.

#### Acceptance Criteria

1. WHEN the System reads the input file THEN the System SHALL parse all fresh range definitions from the first section before the blank line
2. WHEN the System encounters a fresh range definition THEN the System SHALL extract both the start and end values as inclusive bounds
3. WHEN the System reads the input file THEN the System SHALL parse all available ingredient IDs from the section after the blank line
4. WHEN the input file contains malformed data THEN the System SHALL report an error with details about the parsing failure
5. WHEN a fresh range has identical start and end values THEN the System SHALL treat it as a single-value range

### Requirement 2

**User Story:** As an inventory manager, I want the system to determine ingredient freshness based on ranges, so that I can identify which available ingredients are fresh.

#### Acceptance Criteria

1. WHEN an available ingredient ID falls within any fresh range THEN the System SHALL classify that ingredient as fresh
2. WHEN fresh ranges overlap THEN the System SHALL treat any ingredient ID in any overlapping range as fresh
3. WHEN an available ingredient ID does not fall within any fresh range THEN the System SHALL classify that ingredient as spoiled
4. WHEN checking ingredient freshness THEN the System SHALL use inclusive range boundaries where both start and end values are considered fresh
5. WHEN processing all available ingredients THEN the System SHALL count the total number of fresh ingredients

### Requirement 3

**User Story:** As an inventory manager, I want the system to output the count of fresh ingredients, so that I can quickly assess inventory quality.

#### Acceptance Criteria

1. WHEN the System completes processing THEN the System SHALL output the total count of fresh ingredients
2. WHEN the System outputs results THEN the System SHALL display the count in a clear, human-readable format
3. WHEN no available ingredients are fresh THEN the System SHALL output a count of zero

### Requirement 4

**User Story:** As an inventory manager, I want to calculate the total number of unique ingredient IDs considered fresh across all ranges, so that I can understand the complete scope of fresh ingredients defined by the ranges.

#### Acceptance Criteria

1. WHEN the System operates in total fresh range mode THEN the System SHALL identify all unique ingredient IDs that fall within any fresh range
2. WHEN fresh ranges overlap THEN the System SHALL count each unique ingredient ID only once
3. WHEN calculating total fresh IDs THEN the System SHALL include all IDs from range start to range end inclusive
4. WHEN the System completes total fresh range calculation THEN the System SHALL output the count of unique fresh ingredient IDs
5. WHEN a fresh range spans multiple IDs THEN the System SHALL count every ID within that range

### Requirement 5

**User Story:** As a developer, I want the system to handle large datasets efficiently, so that inventory processing completes in reasonable time.

#### Acceptance Criteria

1. WHEN the System processes ingredient IDs THEN the System SHALL use efficient data structures for range lookups
2. WHEN the System checks if an ingredient is fresh THEN the System SHALL minimize redundant range comparisons
3. WHEN the input contains thousands of ranges and ingredient IDs THEN the System SHALL complete processing without performance degradation
