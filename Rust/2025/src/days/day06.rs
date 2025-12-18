use crate::{Day, Example, Solutions};

pub struct Day06;

impl Day06 {
    fn solve_first(input: &str) -> usize {
        let mut lines = input
            .lines()
            .map(|line| line.split(' ').filter(|token| !token.is_empty()))
            .collect::<Vec<_>>();
        let operators = lines.pop().expect("invalid input");

        let mut total = 0;
        for operator in operators {
            let items = lines
                .iter_mut()
                .map(|line| {
                    line.next()
                        .expect("invalid input: lines have different numbers of elements")
                })
                .map(|item| {
                    item.parse::<usize>().expect(
                        format!("invalid input: expected a number, found '{}'", item).as_str(),
                    )
                });
            let column_result = match operator {
                "+" => items.sum::<usize>(),
                "*" => items.product::<usize>(),
                x => panic!("invalid input: invalid operator: '{}'", x),
            };
            total += column_result;
        }

        return total;
    }

    fn solve_second(input: &str) -> usize {
        let mut lines = input.lines().collect::<Vec<_>>();
        let line_length = lines[0].len();
        let operators = lines.pop().expect("invalid input");
        let mut lines = lines.iter().map(|line| line.chars()).collect::<Vec<_>>();
        let mut offset = 0;
        let mut total = 0;
        while offset < operators.len() {
            let operator = operators.chars().nth(offset).expect("invalid input");
            let problem_width = operators[(offset + 1)..]
                .find(|c| c != ' ')
                .unwrap_or(line_length - offset);
            let items = (0..problem_width)
                .map(|_| {
                    let res = lines
                        .iter_mut()
                        .map(|line| {
                            line.next()
                                .expect("invalid input: lines have different lengths")
                        })
                        .filter(|c| *c != ' ')
                        .collect::<String>();
                    return res;
                })
                .map(|s| s.parse::<usize>().expect("invalid input"));
            let column_result: usize = match operator {
                '+' => items.sum(),
                '*' => items.product(),
                x => panic!("invalid input: invalid operator: '{}'", x),
            };
            total += column_result;

            offset += problem_width + 1;
            if offset < operators.len() {
                lines.iter_mut().for_each(|line| {
                    line.next().unwrap();
                });
            }
        }

        return total;
    }
}

impl Day for Day06 {
    fn solve_both(&self, input: String) -> Solutions {
        let input = input.trim_end();
        let first = Self::solve_first(&input);
        let second = Self::solve_second(&input);

        return Solutions::both(first.to_string(), second.to_string());
    }

    fn get_year(&self) -> usize {
        2025
    }

    fn get_day(&self) -> usize {
        6
    }

    fn get_examples(&self) -> Option<Vec<Example>> {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        Some(vec![Example::new(
            input,
            Solutions::both("4277556", "3263827"),
        )])
    }
}
