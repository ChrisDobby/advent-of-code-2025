/// Represents the parsing mode for worksheets
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParsingMode {
    /// Original format: complete numbers stacked vertically in columns
    Horizontal,
    /// Extended format: digits stacked vertically, one number per column
    Vertical,
}

/// Represents a mathematical operation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operation {
    Add,
    Multiply,
}

/// Represents a single math problem
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Problem {
    pub numbers: Vec<i64>,
    pub operation: Operation,
}

/// Errors that can occur during parsing
#[derive(Debug)]
pub enum ParseError {
    InvalidOperation(char),
    EmptyProblem,
    InvalidNumber(String),
}

/// Transpose input text into columns
fn transpose_to_columns(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.is_empty() {
        return Vec::new();
    }

    // Find the maximum line length
    let max_len = lines.iter().map(|line| line.len()).max().unwrap_or(0);

    // Create columns
    let mut columns = vec![Vec::new(); max_len];

    for line in lines {
        for (col_idx, ch) in line.chars().enumerate() {
            columns[col_idx].push(ch);
        }
        // Pad shorter lines with spaces
        for col_idx in line.len()..max_len {
            columns[col_idx].push(' ');
        }
    }

    columns
}

/// Check if a column contains only whitespace
fn is_separator_column(column: &[char]) -> bool {
    column.iter().all(|&ch| ch.is_whitespace())
}

/// Split columns into groups separated by all-whitespace columns
fn split_into_problem_columns(columns: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    let mut problems = Vec::new();
    let mut current_problem = Vec::new();

    for column in columns {
        if is_separator_column(&column) {
            if !current_problem.is_empty() {
                problems.push(current_problem);
                current_problem = Vec::new();
            }
        } else {
            current_problem.push(column);
        }
    }

    // Don't forget the last problem
    if !current_problem.is_empty() {
        problems.push(current_problem);
    }

    problems
}

/// Extract the operation symbol from the bottom of a problem column group
fn extract_operation(problem_columns: &[Vec<char>]) -> Result<Operation, ParseError> {
    // The operation symbol should be at the bottom of the problem
    // Look through all columns in the problem to find the operation symbol
    for column in problem_columns {
        if let Some(&last_char) = column.last() {
            match last_char {
                '+' => return Ok(Operation::Add),
                '*' => return Ok(Operation::Multiply),
                _ if !last_char.is_whitespace() && !last_char.is_ascii_digit() => {
                    return Err(ParseError::InvalidOperation(last_char));
                }
                _ => continue,
            }
        }
    }

    // If we didn't find an operation, that's an error
    Err(ParseError::EmptyProblem)
}

/// Extract numbers from a problem column group
/// Numbers are read vertically, ignoring the operation symbol at the bottom
fn extract_numbers(problem_columns: &[Vec<char>]) -> Result<Vec<i64>, ParseError> {
    let mut numbers = Vec::new();

    if problem_columns.is_empty() {
        return Ok(numbers);
    }

    let num_rows = problem_columns[0].len();

    // Process each row (except the last one which contains the operation)
    for row_idx in 0..num_rows.saturating_sub(1) {
        // Collect characters from this row across all columns
        let mut row_chars = String::new();
        for column in problem_columns {
            if row_idx < column.len() {
                row_chars.push(column[row_idx]);
            }
        }

        // Trim and check if this row contains a number
        let trimmed = row_chars.trim();
        if !trimmed.is_empty() {
            // Try to parse as a number
            match trimmed.parse::<i64>() {
                Ok(num) => numbers.push(num),
                Err(_) => {
                    // Only error if it's not just whitespace or operation symbols
                    if trimmed.chars().any(|c| c.is_ascii_digit()) {
                        return Err(ParseError::InvalidNumber(trimmed.to_string()));
                    }
                }
            }
        }
    }

    Ok(numbers)
}

/// Parse a worksheet from text format in horizontal mode (original behavior)
pub fn parse_worksheet_horizontal(input: &str) -> Result<Vec<Problem>, ParseError> {
    // Handle empty input
    if input.trim().is_empty() {
        return Ok(Vec::new());
    }

    // Step 1: Transpose input into columns
    let columns = transpose_to_columns(input);

    if columns.is_empty() {
        return Ok(Vec::new());
    }

    // Step 2: Split columns into problem groups
    let problem_column_groups = split_into_problem_columns(columns);

    // Step 3: Parse each problem group
    let mut problems = Vec::new();

    for problem_columns in problem_column_groups {
        if problem_columns.is_empty() {
            continue;
        }

        // Extract operation and numbers
        let operation = extract_operation(&problem_columns)?;
        let numbers = extract_numbers(&problem_columns)?;

        // Validate that we have at least some numbers
        if numbers.is_empty() {
            return Err(ParseError::EmptyProblem);
        }

        problems.push(Problem { numbers, operation });
    }

    Ok(problems)
}

/// Parse a worksheet from text format in vertical mode
/// In vertical mode, each column represents a single number with digits stacked vertically
/// (most significant digit at top). Problems are grouped right-to-left.
pub fn parse_worksheet_vertical(input: &str) -> Result<Vec<Problem>, ParseError> {
    // Handle empty input
    if input.trim().is_empty() {
        return Ok(Vec::new());
    }

    // Step 1: Transpose input into columns
    let columns = transpose_to_columns(input);

    if columns.is_empty() {
        return Ok(Vec::new());
    }

    // Step 2: Split columns into problem groups (separated by whitespace columns)
    let problem_column_groups = split_into_problem_columns(columns);

    // Step 3: Parse each problem group
    // In vertical mode, we need to reverse the order since problems are read right-to-left
    let mut problems = Vec::new();

    for problem_columns in problem_column_groups.into_iter().rev() {
        if problem_columns.is_empty() {
            continue;
        }

        // Extract operation symbol (should be at the bottom of one of the columns)
        let operation = extract_operation_vertical(&problem_columns)?;

        // Extract numbers - each column is one number
        let numbers = extract_numbers_vertical(&problem_columns)?;

        // Validate that we have at least some numbers
        if numbers.is_empty() {
            return Err(ParseError::EmptyProblem);
        }

        problems.push(Problem { numbers, operation });
    }

    Ok(problems)
}

/// Extract the operation symbol in vertical mode
/// The operation symbol should be at the bottom of the rightmost column in the problem group
fn extract_operation_vertical(problem_columns: &[Vec<char>]) -> Result<Operation, ParseError> {
    // Look for the operation symbol at the bottom of any column
    for column in problem_columns.iter().rev() {
        if let Some(&last_char) = column.last() {
            match last_char {
                '+' => return Ok(Operation::Add),
                '*' => return Ok(Operation::Multiply),
                _ if !last_char.is_whitespace() && !last_char.is_ascii_digit() => {
                    return Err(ParseError::InvalidOperation(last_char));
                }
                _ => continue,
            }
        }
    }

    Err(ParseError::EmptyProblem)
}

/// Extract numbers in vertical mode
/// Each column represents a single number with digits stacked vertically
/// The topmost digit is the most significant digit
/// Numbers are extracted right-to-left
fn extract_numbers_vertical(problem_columns: &[Vec<char>]) -> Result<Vec<i64>, ParseError> {
    let mut numbers = Vec::new();

    // Process columns right-to-left
    for column in problem_columns.iter().rev() {
        // Build a number from the digits in this column (top to bottom)
        let mut digit_chars = String::new();

        for &ch in column {
            if ch.is_ascii_digit() {
                digit_chars.push(ch);
            } else if ch == '+' || ch == '*' {
                // Stop when we hit the operation symbol
                break;
            }
            // Skip whitespace
        }

        // If we collected any digits, parse them as a number
        if !digit_chars.is_empty() {
            match digit_chars.parse::<i64>() {
                Ok(num) => numbers.push(num),
                Err(_) => return Err(ParseError::InvalidNumber(digit_chars)),
            }
        }
    }

    Ok(numbers)
}

/// Parse a worksheet from text format with specified parsing mode
pub fn parse_worksheet(input: &str, mode: ParsingMode) -> Result<Vec<Problem>, ParseError> {
    match mode {
        ParsingMode::Horizontal => parse_worksheet_horizontal(input),
        ParsingMode::Vertical => parse_worksheet_vertical(input),
    }
}

/// Format a problem back to columnar text format
/// Numbers are right-aligned, with the operation symbol at the bottom
pub fn format_problem(problem: &Problem) -> String {
    if problem.numbers.is_empty() {
        return String::new();
    }

    // Determine the operation symbol
    let op_symbol = match problem.operation {
        Operation::Add => '+',
        Operation::Multiply => '*',
    };

    // Find the maximum width needed (considering all numbers and the operation symbol)
    let max_width = problem.numbers.iter()
        .map(|n| n.to_string().len())
        .max()
        .unwrap_or(1)
        .max(1); // At least 1 for the operation symbol

    // Build the output with right-aligned numbers
    let mut lines = Vec::new();

    for number in &problem.numbers {
        let num_str = number.to_string();
        let padding = max_width - num_str.len();
        lines.push(format!("{}{}", " ".repeat(padding), num_str));
    }

    // Add the operation symbol (right-aligned)
    let op_padding = max_width - 1;
    lines.push(format!("{}{}", " ".repeat(op_padding), op_symbol));

    lines.join("\n")
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_problem_addition() {
        let problem = Problem {
            numbers: vec![10, 20, 30],
            operation: Operation::Add,
        };
        let formatted = format_problem(&problem);
        let expected = "10\n20\n30\n +";
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_format_problem_multiplication() {
        let problem = Problem {
            numbers: vec![2, 3, 4],
            operation: Operation::Multiply,
        };
        let formatted = format_problem(&problem);
        let expected = "2\n3\n4\n*";
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_format_problem_varying_widths() {
        let problem = Problem {
            numbers: vec![1, 100, 5],
            operation: Operation::Add,
        };
        let formatted = format_problem(&problem);
        let expected = "  1\n100\n  5\n  +";
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_format_problem_single_number() {
        let problem = Problem {
            numbers: vec![42],
            operation: Operation::Add,
        };
        let formatted = format_problem(&problem);
        let expected = "42\n +";
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_format_problem_large_numbers() {
        let problem = Problem {
            numbers: vec![12345, 67890],
            operation: Operation::Multiply,
        };
        let formatted = format_problem(&problem);
        let expected = "12345\n67890\n    *";
        assert_eq!(formatted, expected);
    }

    #[test]
    fn test_format_problem_empty() {
        let problem = Problem {
            numbers: vec![],
            operation: Operation::Add,
        };
        let formatted = format_problem(&problem);
        assert_eq!(formatted, "");
    }
}


    #[test]
    fn test_format_parse_round_trip() {
        let original = Problem {
            numbers: vec![10, 20, 30],
            operation: Operation::Add,
        };

        let formatted = format_problem(&original);
        let parsed = parse_worksheet(&formatted, ParsingMode::Horizontal).unwrap();

        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0], original);
    }

    #[test]
    fn test_format_parse_round_trip_multiplication() {
        let original = Problem {
            numbers: vec![5, 10, 2],
            operation: Operation::Multiply,
        };

        let formatted = format_problem(&original);
        let parsed = parse_worksheet(&formatted, ParsingMode::Horizontal).unwrap();

        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0], original);
    }

    #[test]
    fn test_format_parse_round_trip_varying_widths() {
        let original = Problem {
            numbers: vec![1, 100, 5, 1234],
            operation: Operation::Add,
        };

        let formatted = format_problem(&original);
        let parsed = parse_worksheet(&formatted, ParsingMode::Horizontal).unwrap();

        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0], original);
    }

    #[test]
    fn test_parse_vertical_simple() {
        let input = "1 2\n2 3\n3 4\n* +";
        let problems = parse_worksheet_vertical(input).unwrap();

        assert_eq!(problems.len(), 2);
        // Right-to-left: first problem is "234 +"
        assert_eq!(problems[0].numbers, vec![234]);
        assert_eq!(problems[0].operation, Operation::Add);
        // Second problem is "123 *"
        assert_eq!(problems[1].numbers, vec![123]);
        assert_eq!(problems[1].operation, Operation::Multiply);
    }

    #[test]
    fn test_parse_vertical_two_numbers_one_problem() {
        // Two columns with no space between them = one problem with two numbers
        let input = "12\n23\n34\n++";
        let problems = parse_worksheet_vertical(input).unwrap();

        // Should be one problem with two numbers
        assert_eq!(problems.len(), 1);
        // Columns are: [1,2,3,+] and [2,3,4,+]
        // Reading right-to-left: 234, 123
        assert_eq!(problems[0].numbers, vec![234, 123]);
        assert_eq!(problems[0].operation, Operation::Add);
    }
