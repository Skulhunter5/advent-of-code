use std::{env, path::Path, process::ExitCode};

use aoc_2025::days;

enum Mode {
    Run,
    Test,
}

fn main() -> ExitCode {
    let mut args = env::args();
    let _binary_path = args.next().unwrap();
    let mode = if let Some(mode) = args.next() {
        match mode.as_str() {
            "test" => Mode::Test,
            _ => panic!("invalid usage"),
        }
    } else {
        Mode::Run
    };

    match mode {
        Mode::Run => run(),
        Mode::Test => test_examples(),
    }

    return ExitCode::SUCCESS;
}

fn run() {
    for day in days::get_all() {
        let input = std::fs::read_to_string(Path::new("./inputs/").join(day.input_name()))
            .expect(format!("missing input: {}", day.input_name()).as_str());
        let solutions = day.solve_both(input);
        println!("Day {:02}", day.get_day());
        if let Some(part1) = solutions.part1 {
            println!("- Part 1: {}", part1);
        }
        if let Some(part2) = solutions.part2 {
            println!("- Part 2: {}", part2);
        }
    }
}

fn test_examples() {
    for day in days::get_all() {
        if let Some(examples) = day.get_examples() {
            for (i, example) in examples.into_iter().enumerate() {
                let input = example.input;
                let expected = example.solutions;
                let solutions = day.solve_both(input);
                println!("Day {:02}", day.get_day());
                println!("- Example {}", i + 1);
                if let Some(expected1) = expected.part1 {
                    if let Some(part1) = solutions.part1 {
                        if expected1 == part1 {
                            println!("  - Part 1: success");
                        } else {
                            println!(
                                "  - Part 1: failed: {{ expected: '{}', got: '{}' }}",
                                expected1, part1
                            );
                        };
                    } else {
                        println!("  - Part 1: missing");
                    }
                }
                if let Some(expected2) = expected.part2 {
                    if let Some(part2) = solutions.part2 {
                        if expected2 == part2 {
                            println!("  - Part 2: success");
                        } else {
                            println!(
                                "  - Part 2: failed: {{ expected: '{}', got: '{}' }}",
                                expected2, part2
                            );
                        };
                    } else {
                        println!("  - Part 2: missing");
                    }
                }
            }
        }
    }
}
