use crate::{Day, Example, Solutions};

pub struct Day03;

impl Day for Day03 {
    fn solve_both(&self, input: String) -> Solutions {
        let total_output_joltage = input
            .trim_end()
            .lines()
            .map(|line| {
                let line = line.as_bytes();
                let first = line[..(line.len() - 1)].iter().max().unwrap();
                let first_pos = line.iter().position(|byte| byte == first).unwrap();
                let first = *first - b'0';
                let second = *line[(first_pos + 1)..].iter().max().unwrap() as u8 - b'0';
                (first * 10 + second) as usize
            })
            .sum::<usize>();

        return Solutions::first(total_output_joltage.to_string());
    }

    fn get_year(&self) -> usize {
        2025
    }

    fn get_day(&self) -> usize {
        3
    }

    fn get_examples(&self) -> Option<Vec<Example>> {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        Some(vec![Example::new(input, Solutions::first("357"))])
    }
}
