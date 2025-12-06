use paper_roll_accessibility::grid::Grid;
use paper_roll_accessibility::analyzer::count_removable_rolls;

#[test]
fn test_count_removable_rolls_simple() {
    // Create a simple test case where we can manually verify the result
    let input = "\
@.@
...
@.@";

    let mut grid = Grid::new(input.to_string());
    let result = count_removable_rolls(&mut grid);

    // All 4 paper rolls should be accessible and removable in the first iteration
    assert_eq!(result, 4);
}

#[test]
fn test_count_removable_rolls_iterative() {
    // Create a case where removal happens in multiple iterations
    let input = "\
@@@
@@@
@@@";

    let mut grid = Grid::new(input.to_string());
    let result = count_removable_rolls(&mut grid);

    // The center roll has 8 neighbors, so it's not accessible initially
    // The 8 outer rolls are accessible (each has < 4 neighbors)
    // After removing the outer 8, the center becomes accessible
    // Total: 8 + 1 = 9
    assert_eq!(result, 9);
}

#[test]
fn test_count_removable_rolls_empty_grid() {
    let input = "";
    let mut grid = Grid::new(input.to_string());
    let result = count_removable_rolls(&mut grid);

    assert_eq!(result, 0);
}

#[test]
fn test_count_removable_rolls_no_paper_rolls() {
    let input = "\
...
...
...";

    let mut grid = Grid::new(input.to_string());
    let result = count_removable_rolls(&mut grid);

    assert_eq!(result, 0);
}
