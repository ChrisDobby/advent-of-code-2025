pub mod grid;
pub mod analyzer;

use std::fs;
use std::io;

/// Reads the contents of a file with error handling
/// Returns the file contents as a String or an IO error
fn read_input_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn main() {
    // Read from "input.txt"
    let input_contents = match read_input_file("input.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            std::process::exit(1);
        }
    };

    // Create Grid from file contents for first analysis (immutable reference)
    let grid = grid::Grid::new(input_contents.clone());

    // Call analyzer to count accessible rolls (single-pass analysis)
    let accessible_count = analyzer::count_accessible_rolls(&grid);

    // Create a mutable Grid for iterative removal analysis
    let mut grid_for_removal = grid::Grid::new(input_contents);

    // Call analyzer to count removable rolls (iterative removal analysis)
    let removable_count = analyzer::count_removable_rolls(&mut grid_for_removal);

    // Output results clearly labeled
    println!("Accessible rolls: {}", accessible_count);
    println!("Total removable rolls: {}", removable_count);
}
