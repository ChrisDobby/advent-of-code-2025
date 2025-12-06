use math_worksheet_parser::{parse_worksheet, compute_grand_total, solve_problem, ParsingMode};
use std::env;
use std::fs;
use std::process;

fn main() {
    // Check for command-line argument to specify parsing mode
    let args: Vec<String> = env::args().collect();
    let mode = if args.len() > 1 && args[1] == "--vertical" {
        ParsingMode::Vertical
    } else {
        ParsingMode::Horizontal
    };

    // Read input.txt file
    let input = match fs::read_to_string("input.txt") {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading input.txt: {}", err);
            process::exit(1);
        }
    };

    // Parse worksheet
    let problems = match parse_worksheet(&input, mode) {
        Ok(problems) => problems,
        Err(err) => {
            eprintln!("Error parsing worksheet: {:?}", err);
            process::exit(1);
        }
    };

    // Solve all problems and display individual results
    println!("Math Worksheet Parser");
    println!("Mode: {:?}", mode);
    println!("=====================\n");

    for (i, problem) in problems.iter().enumerate() {
        let result = solve_problem(problem);
        println!("Problem {}: {} = {}", i + 1, format_problem_inline(problem), result);
    }

    // Compute and display grand total
    let grand_total = compute_grand_total(&problems);
    println!("\n=====================");
    println!("Grand Total: {}", grand_total);
}

/// Helper function to format a problem inline for display
fn format_problem_inline(problem: &math_worksheet_parser::Problem) -> String {
    let op_symbol = match problem.operation {
        math_worksheet_parser::Operation::Add => "+",
        math_worksheet_parser::Operation::Multiply => "*",
    };

    problem.numbers
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(&format!(" {} ", op_symbol))
}
