// Battery Joltage Calculator
// Main entry point for the application

use battery_joltage::{parse_input_file, calculate_total_joltage_n};
use std::env;
use std::path::Path;
use std::process;

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    let mut input_path = "input.txt";
    let mut n_batteries = 2; // Default to 2 for backward compatibility

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-n" | "--batteries" => {
                // Next argument should be the number of batteries
                if i + 1 >= args.len() {
                    eprintln!("Error: {} requires a value", args[i]);
                    eprintln!("Usage: {} [input_file] [-n|--batteries <number>]", args[0]);
                    process::exit(1);
                }

                match args[i + 1].parse::<usize>() {
                    Ok(n) if n > 0 => {
                        n_batteries = n;
                        i += 2;
                    }
                    Ok(_) => {
                        eprintln!("Error: number of batteries must be greater than 0");
                        process::exit(1);
                    }
                    Err(_) => {
                        eprintln!("Error: invalid number '{}' for {}", args[i + 1], args[i]);
                        eprintln!("Usage: {} [input_file] [-n|--batteries <number>]", args[0]);
                        process::exit(1);
                    }
                }
            }
            arg if !arg.starts_with('-') => {
                // This is the input file path
                input_path = &args[i];
                i += 1;
            }
            _ => {
                eprintln!("Error: unknown option '{}'", args[i]);
                eprintln!("Usage: {} [input_file] [-n|--batteries <number>]", args[0]);
                process::exit(1);
            }
        }
    }

    // Parse the input file
    let banks = match parse_input_file(Path::new(input_path)) {
        Ok(banks) => banks,
        Err(err) => {
            eprintln!("Error parsing input file: {}", err);
            process::exit(1);
        }
    };

    // Calculate total joltage across all banks using n batteries
    let result = calculate_total_joltage_n(&banks, n_batteries);

    // Display individual bank results
    println!("Battery Joltage Calculator");
    println!("==========================");
    println!("Mode: {}-battery selection\n", n_batteries);

    if !result.bank_results.is_empty() {
        println!("Individual Bank Results:");
        for bank_result in &result.bank_results {
            println!("  Bank {}: Maximum Joltage = {}",
                     bank_result.bank_index + 1,
                     bank_result.max_joltage);
        }
        println!();
    }

    // Display errors if any occurred
    if !result.errors.is_empty() {
        println!("Errors encountered:");
        for error in &result.errors {
            eprintln!("  {}", error);
        }
        println!();
    }

    // Display total output joltage prominently
    println!("==========================");
    println!("TOTAL OUTPUT JOLTAGE: {}", result.total_joltage);
    println!("==========================");

    // Set exit code: 0 for success, non-zero if there were errors
    if !result.errors.is_empty() {
        process::exit(2);
    }
}
