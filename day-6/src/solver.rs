use crate::parser::{Problem, Operation};

/// Solve a single problem by applying its operation
pub fn solve_problem(problem: &Problem) -> i64 {
    match problem.operation {
        Operation::Add => problem.numbers.iter().sum(),
        Operation::Multiply => problem.numbers.iter().product(),
    }
}

/// Compute the grand total by summing all problem results
pub fn compute_grand_total(problems: &[Problem]) -> i64 {
    problems.iter()
        .map(|problem| solve_problem(problem))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_addition() {
        let problem = Problem {
            numbers: vec![10, 20, 30],
            operation: Operation::Add,
        };
        assert_eq!(solve_problem(&problem), 60);
    }

    #[test]
    fn test_solve_multiplication() {
        let problem = Problem {
            numbers: vec![2, 3, 4],
            operation: Operation::Multiply,
        };
        assert_eq!(solve_problem(&problem), 24);
    }

    #[test]
    fn test_solve_single_number_addition() {
        let problem = Problem {
            numbers: vec![42],
            operation: Operation::Add,
        };
        assert_eq!(solve_problem(&problem), 42);
    }

    #[test]
    fn test_solve_single_number_multiplication() {
        let problem = Problem {
            numbers: vec![42],
            operation: Operation::Multiply,
        };
        assert_eq!(solve_problem(&problem), 42);
    }

    #[test]
    fn test_compute_grand_total() {
        let problems = vec![
            Problem {
                numbers: vec![10, 20],
                operation: Operation::Add,
            },
            Problem {
                numbers: vec![2, 3],
                operation: Operation::Multiply,
            },
            Problem {
                numbers: vec![100, 50],
                operation: Operation::Add,
            },
        ];
        // 10+20=30, 2*3=6, 100+50=150, total=30+6+150=186
        assert_eq!(compute_grand_total(&problems), 186);
    }

    #[test]
    fn test_compute_grand_total_empty() {
        let problems: Vec<Problem> = vec![];
        assert_eq!(compute_grand_total(&problems), 0);
    }

    #[test]
    fn test_compute_grand_total_single_problem() {
        let problems = vec![
            Problem {
                numbers: vec![5, 10, 15],
                operation: Operation::Add,
            },
        ];
        assert_eq!(compute_grand_total(&problems), 30);
    }
}
