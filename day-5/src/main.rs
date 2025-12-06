mod parser;
mod checker;

use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    // Determine mode: default is "available", can be "total" for total fresh range mode
    let mode = if args.len() > 1 {
        args[1].as_str()
    } else {
        "available"
    };

    // Read input.txt file
    let content = fs::read_to_string("input.txt")?;

    // Parse the input to get InventoryData
    let data = parser::parse_input(&content)?;

    // Execute based on mode
    match mode {
        "total" => {
            // Total fresh range mode: count all unique IDs across ranges
            let total_fresh = checker::count_total_fresh_in_ranges(&data.fresh_ranges);
            println!("Total fresh ingredient IDs in ranges: {}", total_fresh);
        }
        "available" => {
            // Available ingredient mode: count fresh ingredients from available list
            let fresh_count = checker::count_fresh_ingredients(&data);
            println!("Fresh ingredients: {}", fresh_count);
        }
        _ => {
            eprintln!("Unknown mode: '{}'. Use 'available' or 'total'.", mode);
            eprintln!("Usage: {} [available|total]", args[0]);
            std::process::exit(1);
        }
    }

    Ok(())
}
