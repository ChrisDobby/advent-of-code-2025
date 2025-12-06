// Grid module for representing the warehouse layout

pub struct Grid {
    cells: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    /// Constructs a Grid from input string
    /// Handles empty input and varying line lengths
    pub fn new(input: String) -> Grid {
        if input.is_empty() {
            return Grid {
                cells: Vec::new(),
                rows: 0,
                cols: 0,
            };
        }

        let lines: Vec<&str> = input.lines().collect();
        let rows = lines.len();

        // Find the maximum column width
        let cols = lines.iter().map(|line| line.len()).max().unwrap_or(0);

        // Parse each line into a row of characters
        let cells: Vec<Vec<char>> = lines
            .iter()
            .map(|line| line.chars().collect())
            .collect();

        Grid { cells, rows, cols }
    }

    /// Checks if a position contains a paper roll ('@')
    /// Returns false if position is out of bounds
    pub fn is_paper_roll(&self, row: usize, col: usize) -> bool {
        // Bounds checking
        if row >= self.rows {
            return false;
        }

        // Check if the row has enough columns (handles varying line lengths)
        if col >= self.cells[row].len() {
            return false;
        }

        self.cells[row][col] == '@'
    }

    /// Counts the number of paper rolls in the 8 adjacent positions
    /// Handles edge and corner cases with bounds checking
    pub fn count_adjacent_paper_rolls(&self, row: usize, col: usize) -> usize {
        // Define the 8 direction offsets for adjacent positions
        // (row_offset, col_offset)
        let offsets: [(i32, i32); 8] = [
            (-1, -1), // top-left
            (-1, 0),  // top
            (-1, 1),  // top-right
            (0, -1),  // left
            (0, 1),   // right
            (1, -1),  // bottom-left
            (1, 0),   // bottom
            (1, 1),   // bottom-right
        ];

        let mut count = 0;

        for (row_offset, col_offset) in offsets.iter() {
            // Calculate the adjacent position with bounds checking
            // Convert usize to i32 for arithmetic, then back to usize if valid
            let adj_row = row as i32 + row_offset;
            let adj_col = col as i32 + col_offset;

            // Check if the adjacent position is within bounds
            if adj_row >= 0 && adj_col >= 0 {
                let adj_row_usize = adj_row as usize;
                let adj_col_usize = adj_col as usize;

                // Use is_paper_roll which already handles bounds checking
                if self.is_paper_roll(adj_row_usize, adj_col_usize) {
                    count += 1;
                }
            }
        }

        count
    }

    /// Determines if a paper roll at the given position is accessible
    /// A paper roll is accessible if it has fewer than 4 adjacent paper rolls
    /// This method should only be called for positions that contain paper rolls
    pub fn is_accessible(&self, row: usize, col: usize) -> bool {
        // Ensure this position contains a paper roll
        if !self.is_paper_roll(row, col) {
            return false;
        }

        // Count adjacent paper rolls and check if less than 4
        let adjacent_count = self.count_adjacent_paper_rolls(row, col);
        adjacent_count < 4
    }

    /// Returns the number of rows in the grid
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the grid
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Removes a paper roll at the specified position by replacing it with empty space
    /// This method modifies the grid in place
    pub fn remove_roll(&mut self, row: usize, col: usize) {
        // Bounds checking
        if row >= self.rows {
            return;
        }

        // Check if the row has enough columns (handles varying line lengths)
        if col >= self.cells[row].len() {
            return;
        }

        // Replace the paper roll with empty space
        self.cells[row][col] = '.';
    }

    /// Finds all currently accessible paper rolls in the grid
    /// Returns a vector of position tuples (row, col)
    pub fn find_accessible_rolls(&self) -> Vec<(usize, usize)> {
        let mut accessible_positions = Vec::new();

        // Iterate through all positions in the grid
        for row in 0..self.rows {
            for col in 0..self.cells[row].len() {
                // Check if this position is an accessible paper roll
                if self.is_accessible(row, col) {
                    accessible_positions.push((row, col));
                }
            }
        }

        accessible_positions
    }
}
