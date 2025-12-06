use math_worksheet_parser::{parse_worksheet, compute_grand_total, ParsingMode};

#[test]
fn test_example_worksheet_horizontal() {
    let input = include_str!("../input.txt");

    // Parse the worksheet in horizontal mode
    let problems = parse_worksheet(input, ParsingMode::Horizontal).expect("Failed to parse worksheet");

    // Verify we got some problems
    assert!(!problems.is_empty(), "Should parse at least one problem");

    // Compute grand total
    let total = compute_grand_total(&problems);

    // Just verify it computes without panicking
    // The actual value would need to be verified manually
    println!("Parsed {} problems", problems.len());
    println!("Grand total: {}", total);
}

#[test]
fn test_example_worksheet_vertical() {
    let input = include_str!("../input.txt");

    // Parse the worksheet in vertical mode
    let problems = parse_worksheet(input, ParsingMode::Vertical).expect("Failed to parse worksheet");

    // Verify we got some problems
    assert!(!problems.is_empty(), "Should parse at least one problem");

    // Compute grand total
    let total = compute_grand_total(&problems);

    // Expected grand total for vertical mode: 3263827
    println!("Parsed {} problems", problems.len());
    println!("Grand total: {}", total);
    assert_eq!(total, 3263827);
}

#[test]
fn test_simple_worksheet() {
    let input = "10  20\n20  30\n+   *";

    let problems = parse_worksheet(input, ParsingMode::Horizontal).expect("Failed to parse");
    assert_eq!(problems.len(), 2);

    let total = compute_grand_total(&problems);
    // First problem: 10 + 20 = 30
    // Second problem: 20 * 30 = 600
    // Total: 30 + 600 = 630
    assert_eq!(total, 630);
}

#[test]
fn test_simple_vertical() {
    // Example from the problem description
    // Problem 1 (rightmost): 4 + 431 + 623 = 1058
    // Problem 2: 175 * 581 * 32 = 3253600
    // Problem 3: 8 + 248 + 369 = 625
    // Problem 4 (leftmost): 356 * 24 * 1 = 8544
    // Each number gets its own column, digits stacked vertically
    let input = "356 24 1  8 248 369  175 581 32  4 431 623\n  3 2    2 4   3    1   5   3   4   4   6\n  5 4    4 8   6    7   8   2     3   2\n  6      8     9    5   1         1   3\n  *      +          *              +";

    let problems = parse_worksheet(input, ParsingMode::Vertical).expect("Failed to parse");

    // Should have 4 problems (reading right-to-left)
    assert_eq!(problems.len(), 4);

    // Problem 1 (rightmost): 4 + 431 + 623 = 1058
    assert_eq!(problems[0].numbers, vec![4, 431, 623]);
    let result1 = problems[0].numbers.iter().sum::<i64>();
    assert_eq!(result1, 1058);

    // Problem 2: 175 * 581 * 32 = 3253600
    assert_eq!(problems[1].numbers, vec![175, 581, 32]);
    let result2 = problems[1].numbers.iter().product::<i64>();
    assert_eq!(result2, 3253600);

    // Problem 3: 8 + 248 + 369 = 625
    assert_eq!(problems[2].numbers, vec![8, 248, 369]);
    let result3 = problems[2].numbers.iter().sum::<i64>();
    assert_eq!(result3, 625);

    // Problem 4 (leftmost): 356 * 24 * 1 = 8544
    assert_eq!(problems[3].numbers, vec![356, 24, 1]);
    let result4 = problems[3].numbers.iter().product::<i64>();
    assert_eq!(result4, 8544);

    // Grand total: 1058 + 3253600 + 625 + 8544 = 3263827
    let total = compute_grand_total(&problems);
    assert_eq!(total, 3263827);
}
