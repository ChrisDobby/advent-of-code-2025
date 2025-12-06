// Battery Joltage Calculator Library
// Core functionality for parsing and calculating battery joltage

use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};

/// Represents a single bank of batteries as a sequence of digits
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatteryBank {
    pub batteries: Vec<u8>,
}

impl BatteryBank {
    /// Parse a line into a battery bank
    ///
    /// Returns `None` if the line is empty or contains only whitespace.
    /// Returns `Err` if the line contains non-digit characters.
    ///
    /// # Arguments
    /// * `line` - A string slice containing the battery bank data
    ///
    /// # Returns
    /// * `Ok(Some(BatteryBank))` - Successfully parsed battery bank
    /// * `Ok(None)` - Empty or whitespace-only line (should be skipped)
    /// * `Err(ParseError)` - Line contains invalid characters
    pub fn from_line(line: &str) -> Result<Option<Self>, ParseError> {
        // Trim the line to check if it's empty or whitespace-only
        let trimmed = line.trim();

        // Handle empty and whitespace-only lines
        if trimmed.is_empty() {
            return Ok(None);
        }

        // Validate that all characters are digits and convert to Vec<u8>
        let mut batteries = Vec::with_capacity(trimmed.len());

        for ch in trimmed.chars() {
            if ch.is_ascii_digit() {
                // Convert char digit to u8 value (0-9)
                batteries.push(ch as u8 - b'0');
            } else {
                // Found a non-digit character - this is an error
                // Note: line number will be provided by the caller
                return Err(ParseError::InvalidCharacter {
                    line: 0, // Placeholder - will be set by caller
                    character: ch,
                });
            }
        }

        Ok(Some(BatteryBank { batteries }))
    }

    /// Find the maximum joltage that can be produced by selecting two batteries
    ///
    /// Examines all pairs of batteries (i, j) where i < j, calculates the joltage
    /// as a two-digit number (batteries[i] * 10 + batteries[j]), and returns the maximum.
    ///
    /// # Returns
    /// * `Ok(u32)` - The maximum joltage value
    /// * `Err(JoltageError)` - If the bank has fewer than 2 batteries
    ///
    /// # Examples
    /// ```
    /// use battery_joltage::BatteryBank;
    ///
    /// let bank = BatteryBank { batteries: vec![9, 8, 7] };
    /// assert_eq!(bank.find_max_joltage().unwrap(), 98);
    /// ```
    pub fn find_max_joltage(&self) -> Result<u32, JoltageError> {
        // Check if we have at least 2 batteries
        if self.batteries.len() < 2 {
            return Err(JoltageError::InsufficientBatteries {
                count: self.batteries.len(),
                required: 2,
            });
        }

        // Initialize max_joltage with the first possible pair
        let mut max_joltage = 0u32;

        // Iterate through all pairs (i, j) where i < j
        for i in 0..self.batteries.len() {
            for j in (i + 1)..self.batteries.len() {
                // Calculate joltage: first battery as tens digit, second as ones digit
                let joltage = (self.batteries[i] as u32) * 10 + (self.batteries[j] as u32);

                // Update maximum if this joltage is larger
                if joltage > max_joltage {
                    max_joltage = joltage;
                }
            }
        }

        Ok(max_joltage)
    }

    /// Find the maximum joltage that can be produced by selecting exactly n batteries
    ///
    /// Uses a greedy algorithm to select n batteries that form the largest possible number.
    /// The algorithm works left-to-right, selecting the largest digit available at each position
    /// while ensuring enough batteries remain to fill the remaining positions.
    ///
    /// # Arguments
    /// * `n` - The number of batteries to select
    ///
    /// # Returns
    /// * `Ok(u64)` - The maximum n-digit joltage value
    /// * `Err(JoltageError)` - If the bank has fewer than n batteries
    ///
    /// # Examples
    /// ```
    /// use battery_joltage::BatteryBank;
    ///
    /// let bank = BatteryBank { batteries: vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1] };
    /// assert_eq!(bank.find_max_joltage_n(12).unwrap(), 987654321111);
    /// ```
    pub fn find_max_joltage_n(&self, n: usize) -> Result<u64, JoltageError> {
        // Check if we have at least n batteries
        if self.batteries.len() < n {
            return Err(JoltageError::InsufficientBatteries {
                count: self.batteries.len(),
                required: n,
            });
        }

        // Greedy algorithm: for each position in the result, find the largest digit
        // we can place there while leaving enough batteries for the remaining positions
        let mut result = Vec::with_capacity(n);
        let mut start_index = 0;

        for position in 0..n {
            // How many more batteries do we need after this position?
            let remaining_needed = n - position - 1;

            // We can search up to this index (must leave enough batteries for remaining positions)
            let search_end = self.batteries.len() - remaining_needed;

            // Find the maximum digit in the valid range
            let mut max_digit = 0u8;
            let mut max_index = start_index;

            for i in start_index..search_end {
                if self.batteries[i] > max_digit {
                    max_digit = self.batteries[i];
                    max_index = i;
                }
            }

            // Add this digit to our result
            result.push(max_digit);

            // Next search starts after this selected battery
            start_index = max_index + 1;
        }

        // Convert the result vector to a u64 number
        let mut joltage = 0u64;
        for digit in result {
            joltage = joltage * 10 + (digit as u64);
        }

        Ok(joltage)
    }
}

/// Errors that can occur during parsing of input
#[derive(Debug)]
pub enum ParseError {
    FileNotFound(PathBuf),
    InvalidCharacter { line: usize, character: char },
    IoError(io::Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::FileNotFound(path) => {
                write!(f, "Input file not found: {}", path.display())
            }
            ParseError::InvalidCharacter { line, character } => {
                write!(
                    f,
                    "Invalid character '{}' found on line {}",
                    character, line
                )
            }
            ParseError::IoError(err) => {
                write!(f, "I/O error while reading input: {}", err)
            }
        }
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ParseError::IoError(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for ParseError {
    fn from(err: io::Error) -> Self {
        ParseError::IoError(err)
    }
}

/// Errors that can occur during joltage calculation
#[derive(Debug)]
pub enum JoltageError {
    InsufficientBatteries { count: usize, required: usize },
}

impl fmt::Display for JoltageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JoltageError::InsufficientBatteries { count, required } => {
                write!(
                    f,
                    "Insufficient batteries for joltage calculation: found {}, need at least {}",
                    count, required
                )
            }
        }
    }
}

impl std::error::Error for JoltageError {}

/// Errors that can occur during the overall processing pipeline
#[derive(Debug)]
pub enum ProcessingError {
    ParseError(ParseError),
    JoltageError { bank_index: usize, error: JoltageError },
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessingError::ParseError(err) => {
                write!(f, "Parse error: {}", err)
            }
            ProcessingError::JoltageError { bank_index, error } => {
                write!(f, "Error in bank {}: {}", bank_index, error)
            }
        }
    }
}

impl std::error::Error for ProcessingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProcessingError::ParseError(err) => Some(err),
            ProcessingError::JoltageError { error, .. } => Some(error),
        }
    }
}

impl From<ParseError> for ProcessingError {
    fn from(err: ParseError) -> Self {
        ProcessingError::ParseError(err)
    }
}

/// Result of processing a single battery bank
#[derive(Debug, Clone)]
pub struct BankResult {
    pub bank_index: usize,
    pub max_joltage: u64,
}

/// Result of processing all battery banks
#[derive(Debug)]
pub struct ProcessingResult {
    pub bank_results: Vec<BankResult>,
    pub total_joltage: u64,
    pub errors: Vec<ProcessingError>,
}

/// Calculate the total joltage across all battery banks
///
/// Processes each bank sequentially, calculating the maximum joltage for each.
/// Banks that produce errors (e.g., insufficient batteries) are skipped and
/// their errors are collected for reporting. The total joltage is the sum of
/// all successfully processed banks.
///
/// # Arguments
/// * `banks` - A slice of battery banks to process
///
/// # Returns
/// * `ProcessingResult` - Contains individual bank results, total joltage, and any errors
///
/// # Examples
/// ```
/// use battery_joltage::{BatteryBank, calculate_total_joltage};
///
/// let banks = vec![
///     BatteryBank { batteries: vec![9, 8, 7] },
///     BatteryBank { batteries: vec![5, 4, 3] },
/// ];
/// let result = calculate_total_joltage(&banks);
/// assert_eq!(result.total_joltage, 98 + 54);
/// assert_eq!(result.bank_results.len(), 2);
/// assert_eq!(result.errors.len(), 0);
/// ```
pub fn calculate_total_joltage(banks: &[BatteryBank]) -> ProcessingResult {
    calculate_total_joltage_n(banks, 2)
}

/// Calculate the total joltage across all battery banks using n batteries per bank
///
/// Processes each bank sequentially, calculating the maximum joltage for each
/// by selecting exactly n batteries. Banks that produce errors (e.g., insufficient
/// batteries) are skipped and their errors are collected for reporting. The total
/// joltage is the sum of all successfully processed banks.
///
/// # Arguments
/// * `banks` - A slice of battery banks to process
/// * `n` - The number of batteries to select from each bank
///
/// # Returns
/// * `ProcessingResult` - Contains individual bank results, total joltage, and any errors
///
/// # Examples
/// ```
/// use battery_joltage::{BatteryBank, calculate_total_joltage_n};
///
/// let banks = vec![
///     BatteryBank { batteries: vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1] },
/// ];
/// let result = calculate_total_joltage_n(&banks, 12);
/// assert_eq!(result.total_joltage, 987654321111);
/// assert_eq!(result.bank_results.len(), 1);
/// assert_eq!(result.errors.len(), 0);
/// ```
pub fn calculate_total_joltage_n(banks: &[BatteryBank], n: usize) -> ProcessingResult {
    let mut bank_results = Vec::new();
    let mut total_joltage = 0u64;
    let mut errors = Vec::new();

    // Process each bank sequentially
    for (index, bank) in banks.iter().enumerate() {
        let result = if n == 2 {
            // Use the optimized 2-battery algorithm
            bank.find_max_joltage().map(|v| v as u64)
        } else {
            // Use the n-battery algorithm
            bank.find_max_joltage_n(n)
        };

        match result {
            Ok(max_joltage) => {
                // Successfully calculated joltage - add to results
                bank_results.push(BankResult {
                    bank_index: index,
                    max_joltage,
                });
                total_joltage += max_joltage;
            }
            Err(error) => {
                // Bank produced an error - collect it and continue
                errors.push(ProcessingError::JoltageError {
                    bank_index: index,
                    error,
                });
            }
        }
    }

    ProcessingResult {
        bank_results,
        total_joltage,
        errors,
    }
}

/// Parse an input file containing battery banks
///
/// Reads the file line by line using buffered I/O. Each non-empty line
/// should contain a battery bank (sequence of digits). Empty lines and
/// whitespace-only lines are skipped.
///
/// # Arguments
/// * `path` - Path to the input file
///
/// # Returns
/// * `Ok(Vec<BatteryBank>)` - Successfully parsed battery banks
/// * `Err(ParseError)` - File not found, I/O error, or invalid line content
///
/// # Examples
/// ```no_run
/// use battery_joltage::parse_input_file;
/// use std::path::Path;
///
/// let banks = parse_input_file(Path::new("input.txt")).unwrap();
/// println!("Parsed {} battery banks", banks.len());
/// ```
pub fn parse_input_file(path: &Path) -> Result<Vec<BatteryBank>, ParseError> {
    // Check if file exists and open it
    let file = File::open(path).map_err(|err| {
        if err.kind() == io::ErrorKind::NotFound {
            ParseError::FileNotFound(path.to_path_buf())
        } else {
            ParseError::IoError(err)
        }
    })?;

    // Use buffered reader for efficiency
    let reader = BufReader::new(file);
    let mut banks = Vec::new();
    let mut line_number = 0;

    // Process each line
    for line_result in reader.lines() {
        line_number += 1;
        let line = line_result?;

        // Parse the line into a battery bank
        match BatteryBank::from_line(&line) {
            Ok(Some(bank)) => {
                // Valid bank - add it to our collection
                banks.push(bank);
            }
            Ok(None) => {
                // Empty or whitespace-only line - skip it
                continue;
            }
            Err(ParseError::InvalidCharacter { character, .. }) => {
                // Invalid character found - return error with correct line number
                return Err(ParseError::InvalidCharacter {
                    line: line_number,
                    character,
                });
            }
            Err(err) => {
                // Other parse error - propagate it
                return Err(err);
            }
        }
    }

    Ok(banks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn battery_bank_creation() {
        let bank = BatteryBank {
            batteries: vec![1, 2, 3],
        };
        assert_eq!(bank.batteries.len(), 3);
    }

    #[test]
    fn parse_error_display() {
        let err = ParseError::InvalidCharacter {
            line: 5,
            character: 'x',
        };
        assert_eq!(err.to_string(), "Invalid character 'x' found on line 5");
    }

    #[test]
    fn joltage_error_display() {
        let err = JoltageError::InsufficientBatteries { count: 1, required: 2 };
        assert_eq!(
            err.to_string(),
            "Insufficient batteries for joltage calculation: found 1, need at least 2"
        );
    }

    #[test]
    fn processing_error_display() {
        let joltage_err = JoltageError::InsufficientBatteries { count: 0, required: 2 };
        let proc_err = ProcessingError::JoltageError {
            bank_index: 3,
            error: joltage_err,
        };
        assert!(proc_err.to_string().contains("Error in bank 3"));
    }

    // Tests for BatteryBank::from_line()

    #[test]
    fn from_line_valid_digits() {
        let result = BatteryBank::from_line("123456789");
        assert!(result.is_ok());
        let bank = result.unwrap();
        assert!(bank.is_some());
        let bank = bank.unwrap();
        assert_eq!(bank.batteries, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn from_line_empty_string() {
        let result = BatteryBank::from_line("");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn from_line_whitespace_only() {
        let result = BatteryBank::from_line("   \t  \n  ");
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn from_line_with_leading_trailing_whitespace() {
        let result = BatteryBank::from_line("  987654321  ");
        assert!(result.is_ok());
        let bank = result.unwrap();
        assert!(bank.is_some());
        let bank = bank.unwrap();
        assert_eq!(bank.batteries, vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn from_line_invalid_character() {
        let result = BatteryBank::from_line("123x456");
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::InvalidCharacter { character, .. } => {
                assert_eq!(character, 'x');
            }
            _ => panic!("Expected InvalidCharacter error"),
        }
    }

    #[test]
    fn from_line_with_space_in_middle() {
        let result = BatteryBank::from_line("123 456");
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::InvalidCharacter { character, .. } => {
                assert_eq!(character, ' ');
            }
            _ => panic!("Expected InvalidCharacter error"),
        }
    }

    #[test]
    fn from_line_single_digit() {
        let result = BatteryBank::from_line("5");
        assert!(result.is_ok());
        let bank = result.unwrap();
        assert!(bank.is_some());
        let bank = bank.unwrap();
        assert_eq!(bank.batteries, vec![5]);
    }

    #[test]
    fn from_line_all_zeros() {
        let result = BatteryBank::from_line("0000");
        assert!(result.is_ok());
        let bank = result.unwrap();
        assert!(bank.is_some());
        let bank = bank.unwrap();
        assert_eq!(bank.batteries, vec![0, 0, 0, 0]);
    }

    // Tests for BatteryBank::find_max_joltage()

    #[test]
    fn find_max_joltage_simple_case() {
        let bank = BatteryBank {
            batteries: vec![9, 8, 7],
        };
        assert_eq!(bank.find_max_joltage().unwrap(), 98);
    }

    #[test]
    fn find_max_joltage_two_batteries() {
        let bank = BatteryBank {
            batteries: vec![5, 3],
        };
        assert_eq!(bank.find_max_joltage().unwrap(), 53);
    }

    #[test]
    fn find_max_joltage_all_same() {
        let bank = BatteryBank {
            batteries: vec![5, 5, 5, 5],
        };
        assert_eq!(bank.find_max_joltage().unwrap(), 55);
    }

    #[test]
    fn find_max_joltage_ascending() {
        let bank = BatteryBank {
            batteries: vec![1, 2, 3, 4, 5],
        };
        assert_eq!(bank.find_max_joltage().unwrap(), 45);
    }

    #[test]
    fn find_max_joltage_descending() {
        let bank = BatteryBank {
            batteries: vec![9, 8, 7, 6, 5],
        };
        assert_eq!(bank.find_max_joltage().unwrap(), 98);
    }

    #[test]
    fn find_max_joltage_insufficient_batteries_empty() {
        let bank = BatteryBank {
            batteries: vec![],
        };
        let result = bank.find_max_joltage();
        assert!(result.is_err());
        match result.unwrap_err() {
            JoltageError::InsufficientBatteries { count, required } => {
                assert_eq!(count, 0);
                assert_eq!(required, 2);
            }
        }
    }

    #[test]
    fn find_max_joltage_insufficient_batteries_single() {
        let bank = BatteryBank {
            batteries: vec![7],
        };
        let result = bank.find_max_joltage();
        assert!(result.is_err());
        match result.unwrap_err() {
            JoltageError::InsufficientBatteries { count, required } => {
                assert_eq!(count, 1);
                assert_eq!(required, 2);
            }
        }
    }

    #[test]
    fn find_max_joltage_with_zeros() {
        let bank = BatteryBank {
            batteries: vec![0, 9, 0],
        };
        assert_eq!(bank.find_max_joltage().unwrap(), 90);
    }

    #[test]
    fn find_max_joltage_all_zeros() {
        let bank = BatteryBank {
            batteries: vec![0, 0, 0],
        };
        assert_eq!(bank.find_max_joltage().unwrap(), 0);
    }

    // Tests for parse_input_file()

    use std::fs;

    #[test]
    fn parse_input_file_not_found() {
        let result = parse_input_file(Path::new("nonexistent_file.txt"));
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::FileNotFound(path) => {
                assert_eq!(path, PathBuf::from("nonexistent_file.txt"));
            }
            _ => panic!("Expected FileNotFound error"),
        }
    }

    #[test]
    fn parse_input_file_empty() {
        // Create a temporary empty file
        let temp_path = "test_empty.txt";
        fs::write(temp_path, "").unwrap();

        let result = parse_input_file(Path::new(temp_path));
        assert!(result.is_ok());
        let banks = result.unwrap();
        assert_eq!(banks.len(), 0);

        // Clean up
        fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn parse_input_file_multiple_valid_banks() {
        // Create a temporary file with multiple valid banks
        let temp_path = "test_multiple.txt";
        let content = "123456789\n987654321\n111111111\n";
        fs::write(temp_path, content).unwrap();

        let result = parse_input_file(Path::new(temp_path));
        assert!(result.is_ok());
        let banks = result.unwrap();
        assert_eq!(banks.len(), 3);
        assert_eq!(banks[0].batteries, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        assert_eq!(banks[1].batteries, vec![9, 8, 7, 6, 5, 4, 3, 2, 1]);
        assert_eq!(banks[2].batteries, vec![1, 1, 1, 1, 1, 1, 1, 1, 1]);

        // Clean up
        fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn parse_input_file_with_empty_lines() {
        // Create a temporary file with empty lines mixed in
        let temp_path = "test_empty_lines.txt";
        let content = "123\n\n456\n   \n789\n";
        fs::write(temp_path, content).unwrap();

        let result = parse_input_file(Path::new(temp_path));
        assert!(result.is_ok());
        let banks = result.unwrap();
        assert_eq!(banks.len(), 3);
        assert_eq!(banks[0].batteries, vec![1, 2, 3]);
        assert_eq!(banks[1].batteries, vec![4, 5, 6]);
        assert_eq!(banks[2].batteries, vec![7, 8, 9]);

        // Clean up
        fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn parse_input_file_invalid_character() {
        // Create a temporary file with an invalid character
        let temp_path = "test_invalid.txt";
        let content = "123\n456x789\n999\n";
        fs::write(temp_path, content).unwrap();

        let result = parse_input_file(Path::new(temp_path));
        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::InvalidCharacter { line, character } => {
                assert_eq!(line, 2);
                assert_eq!(character, 'x');
            }
            _ => panic!("Expected InvalidCharacter error"),
        }

        // Clean up
        fs::remove_file(temp_path).unwrap();
    }

    // Tests for calculate_total_joltage()

    #[test]
    fn calculate_total_joltage_all_valid() {
        let banks = vec![
            BatteryBank {
                batteries: vec![9, 8, 7],
            },
            BatteryBank {
                batteries: vec![5, 4, 3],
            },
            BatteryBank {
                batteries: vec![2, 1],
            },
        ];

        let result = calculate_total_joltage(&banks);

        assert_eq!(result.bank_results.len(), 3);
        assert_eq!(result.errors.len(), 0);
        assert_eq!(result.bank_results[0].max_joltage, 98);
        assert_eq!(result.bank_results[1].max_joltage, 54);
        assert_eq!(result.bank_results[2].max_joltage, 21);
        assert_eq!(result.total_joltage, 98 + 54 + 21);
    }

    #[test]
    fn calculate_total_joltage_with_errors() {
        let banks = vec![
            BatteryBank {
                batteries: vec![9, 8, 7],
            },
            BatteryBank {
                batteries: vec![5], // Only 1 battery - will error
            },
            BatteryBank {
                batteries: vec![4, 3, 2],
            },
            BatteryBank {
                batteries: vec![], // Empty - will error
            },
        ];

        let result = calculate_total_joltage(&banks);

        // Should have 2 successful results and 2 errors
        assert_eq!(result.bank_results.len(), 2);
        assert_eq!(result.errors.len(), 2);

        // Check successful banks
        assert_eq!(result.bank_results[0].bank_index, 0);
        assert_eq!(result.bank_results[0].max_joltage, 98);
        assert_eq!(result.bank_results[1].bank_index, 2);
        assert_eq!(result.bank_results[1].max_joltage, 43);

        // Total should only include successful banks
        assert_eq!(result.total_joltage, 98 + 43);

        // Check errors
        match &result.errors[0] {
            ProcessingError::JoltageError { bank_index, .. } => {
                assert_eq!(*bank_index, 1);
            }
            _ => panic!("Expected JoltageError"),
        }
        match &result.errors[1] {
            ProcessingError::JoltageError { bank_index, .. } => {
                assert_eq!(*bank_index, 3);
            }
            _ => panic!("Expected JoltageError"),
        }
    }

    #[test]
    fn calculate_total_joltage_empty_collection() {
        let banks: Vec<BatteryBank> = vec![];
        let result = calculate_total_joltage(&banks);

        assert_eq!(result.bank_results.len(), 0);
        assert_eq!(result.errors.len(), 0);
        assert_eq!(result.total_joltage, 0);
    }

    #[test]
    fn calculate_total_joltage_all_errors() {
        let banks = vec![
            BatteryBank {
                batteries: vec![5],
            },
            BatteryBank {
                batteries: vec![],
            },
            BatteryBank {
                batteries: vec![1],
            },
        ];

        let result = calculate_total_joltage(&banks);

        assert_eq!(result.bank_results.len(), 0);
        assert_eq!(result.errors.len(), 3);
        assert_eq!(result.total_joltage, 0);
    }

    // Tests for BatteryBank::find_max_joltage_n()

    #[test]
    fn find_max_joltage_n_example_1() {
        // 987654321111111 -> 987654321111 (12 digits)
        let bank = BatteryBank {
            batteries: vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1],
        };
        assert_eq!(bank.find_max_joltage_n(12).unwrap(), 987654321111);
    }

    #[test]
    fn find_max_joltage_n_example_2() {
        // 811111111111119 -> 811111111119 (12 digits)
        let bank = BatteryBank {
            batteries: vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9],
        };
        assert_eq!(bank.find_max_joltage_n(12).unwrap(), 811111111119);
    }

    #[test]
    fn find_max_joltage_n_example_3() {
        // 234234234234278 -> 434234234278 (12 digits)
        let bank = BatteryBank {
            batteries: vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8],
        };
        assert_eq!(bank.find_max_joltage_n(12).unwrap(), 434234234278);
    }

    #[test]
    fn find_max_joltage_n_example_4() {
        // 818181911112111 -> 888911112111 (12 digits)
        let bank = BatteryBank {
            batteries: vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1],
        };
        assert_eq!(bank.find_max_joltage_n(12).unwrap(), 888911112111);
    }

    #[test]
    fn find_max_joltage_n_matches_original_for_n2() {
        // Verify that n=2 gives same result as original algorithm
        let bank = BatteryBank {
            batteries: vec![9, 8, 7, 6, 5],
        };
        assert_eq!(bank.find_max_joltage_n(2).unwrap(), 98);
        assert_eq!(bank.find_max_joltage().unwrap(), 98);
    }

    #[test]
    fn find_max_joltage_n_exact_length() {
        // When n equals bank length, should return all digits in order
        let bank = BatteryBank {
            batteries: vec![9, 8, 7],
        };
        assert_eq!(bank.find_max_joltage_n(3).unwrap(), 987);
    }

    #[test]
    fn find_max_joltage_n_single_battery() {
        let bank = BatteryBank {
            batteries: vec![5, 3, 7, 2],
        };
        assert_eq!(bank.find_max_joltage_n(1).unwrap(), 7);
    }

    #[test]
    fn find_max_joltage_n_insufficient_batteries() {
        let bank = BatteryBank {
            batteries: vec![1, 2, 3],
        };
        let result = bank.find_max_joltage_n(5);
        assert!(result.is_err());
        match result.unwrap_err() {
            JoltageError::InsufficientBatteries { count, required } => {
                assert_eq!(count, 3);
                assert_eq!(required, 5);
            }
        }
    }

    #[test]
    fn find_max_joltage_n_empty_bank() {
        let bank = BatteryBank {
            batteries: vec![],
        };
        let result = bank.find_max_joltage_n(1);
        assert!(result.is_err());
    }
}
