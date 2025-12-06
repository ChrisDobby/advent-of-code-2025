mod parser;
mod simulator;

use std::env;
use std::fs;
use std::io;
use std::process;

use parser::parse_rotations;
use simulator::{count_all_zero_passes, count_zero_crossings};

/// Counting method selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CountingMethod {
    /// Original method: count only when dial lands on 0 at end of rotation
    EndOfRotation,
    /// Method 0x434C49434B: count all passes through 0 during rotations
    AllPasses,
}

/// Read the contents of the input file
///
/// # Arguments
///
/// * `path` - Path to the input file
///
/// # Returns
///
/// * `Ok(String)` - The file contents as a string
/// * `Err(io::Error)` - An I/O error if the file cannot be read
fn read_input_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

/// Parse command-line arguments to determine counting method
///
/// # Returns
///
/// The selected counting method. Defaults to AllPasses (method 0x434C49434B).
fn parse_counting_method() -> CountingMethod {
    let args: Vec<String> = env::args().collect();

    // Check for --method flag
    for i in 0..args.len() {
        if args[i] == "--method" && i + 1 < args.len() {
            match args[i + 1].as_str() {
                "original" | "end" => return CountingMethod::EndOfRotation,
                "all" | "0x434C49434B" => return CountingMethod::AllPasses,
                _ => {
                    eprintln!("Warning: Unknown method '{}', using default (all passes)", args[i + 1]);
                    return CountingMethod::AllPasses;
                }
            }
        }
    }

    // Default to method 0x434C49434B
    CountingMethod::AllPasses
}

fn main() {
    // Parse command-line arguments to determine counting method
    let method = parse_counting_method();

    // Read input.txt file
    let input = match read_input_file("input.txt") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input.txt: {}", e);
            process::exit(1);
        }
    };

    // Parse rotations
    let rotations = match parse_rotations(&input) {
        Ok(rots) => rots,
        Err(e) => {
            eprintln!("Error parsing rotations: {}", e);
            process::exit(1);
        }
    };

    // Count zeros using the selected method
    let count = match method {
        CountingMethod::EndOfRotation => count_zero_crossings(&rotations),
        CountingMethod::AllPasses => count_all_zero_passes(&rotations),
    };

    // Print result to stdout with indication of method used
    match method {
        CountingMethod::EndOfRotation => {
            println!("{} (original method: end-of-rotation crossings)", count);
        }
        CountingMethod::AllPasses => {
            println!("{} (method 0x434C49434B: all passes through 0)", count);
        }
    }
}
