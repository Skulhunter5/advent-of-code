use std::{collections::HashSet, env, path::Path, process::ExitCode};

use aoc_2025::{Day, days};

enum Mode {
    Run,
    Test,
}

fn main() -> ExitCode {
    let mut args = env::args();
    let _binary_path = args.next().unwrap();
    let mut mode = None;
    let mut days_to_run = HashSet::new();
    while let Some(arg) = args.next() {
        if let Some(new_mode) = match arg.as_str() {
            "test" => Some(Mode::Test),
            "run" => Some(Mode::Run),
            _ => None,
        } {
            if mode.is_some() {
                panic!("error: multiple modes given");
            }
            mode = Some(new_mode);
        } else if ["--day"].contains(&arg.as_str()) {
            let day = args
                .next()
                .expect(format!("error: missing parameter for {}", arg).as_str());
            let day = day
                .parse::<usize>()
                .expect(format!("error: invalid argument: {}", day).as_str());
            days_to_run.insert(day);
        } else if ["--days", "-d"].contains(&arg.as_str()) {
            let days_str = args
                .next()
                .expect(format!("error: missing parameter for {}", arg).as_str());
            days_to_run.extend(days_str.split(',').map(|day| {
                day.parse::<usize>()
                    .expect(format!("error: invalid parameter: {}", days_str).as_str())
            }));
        } else {
            panic!("error: unexpected argument: {}", arg);
        }
    }

    let days_to_run =
        days::iter().filter(|day| days_to_run.is_empty() || days_to_run.contains(&day.get_day()));
    match mode.unwrap_or(Mode::Run) {
        Mode::Run => run(days_to_run),
        Mode::Test => test_examples(days_to_run),
    }

    return ExitCode::SUCCESS;
}

fn run<D: Iterator<Item = Box<dyn Day>>>(days_to_run: D) {
    for day in days_to_run {
        let Ok(input) = std::fs::read_to_string(Path::new("./inputs/").join(day.input_name())) else {
            continue;
        };
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

fn test_examples<D: Iterator<Item = Box<dyn Day>>>(days_to_run: D) {
    for day in days_to_run {
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
