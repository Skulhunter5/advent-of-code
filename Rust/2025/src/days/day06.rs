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
            let items = lines.iter_mut().map(|line| {
                line.next()
                    .expect("invalid input: lines have different numbers of elements")
            });
            let column_result = match operator {
                "+" => items
                    .map(|item| {
                        item.parse::<usize>().expect(
                            format!("invalid input: expected a number, found '{}'", item).as_str(),
                        )
                    })
                    .sum::<usize>(),
                "*" => items
                    .map(|item| {
                        item.parse::<usize>().expect(
                            format!("invalid input: expected a number, found '{}'", item).as_str(),
                        )
                    })
                    .product::<usize>(),
                x => panic!("invalid input: invalid operator: '{}'", x),
            };
            total += column_result;
        }

        return total;
    }

    fn solve_second(input: &str) -> usize {
        let mut lines = input.lines().collect::<Vec<_>>();
        let mut operators = lines.pop().expect("invalid input");
        while let Some(operator_pos) = operators.find(|c| c != ' ') {
            // let operator = operators.chars().nth(operator_pos);
        }

        todo!();
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

        Some(vec![Example::new(input, Solutions::first("4277556"))])
    }
}
