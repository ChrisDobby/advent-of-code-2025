// Analyzer module for counting accessible paper rolls

use crate::grid::Grid;

/// Counts the total number of accessible paper rolls in the grid
/// Iterates through all grid positions and counts positions that are both paper rolls and accessible
/// Requirements: 3.4
pub fn count_accessible_rolls(grid: &Grid) -> usize {
    let mut count = 0;

    // Iterate through all grid positions
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            // Count positions that are both paper rolls and accessible
            if grid.is_paper_roll(row, col) && grid.is_accessible(row, col) {
                count += 1;
            }
        }
    }

    count
}

/// Performs iterative removal simulation and returns total count of removed rolls
/// Repeatedly finds and removes all accessible rolls until none remain
/// Requirements: 5.1, 5.2, 5.4, 5.5, 5.6
pub fn count_removable_rolls(grid: &mut Grid) -> usize {
    let mut total_removed = 0;

    loop {
        // Find all currently accessible roll positions
        let accessible_rolls = grid.find_accessible_rolls();

        // If no accessible rolls remain, terminate the process
        if accessible_rolls.is_empty() {
            break;
        }

        // Remove all accessible rolls simultaneously
        for (row, col) in accessible_rolls.iter() {
            grid.remove_roll(*row, *col);
        }

        // Increment the total count by the number of rolls removed in this iteration
        total_removed += accessible_rolls.len();
    }

    total_removed
}
