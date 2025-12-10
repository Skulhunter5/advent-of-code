use crate::{Day, Example, Solutions};

pub struct Day03;

impl Day for Day03 {
    fn solve_both(&self, input: String) -> Solutions {
        let input = input.trim_end();

        let total_output_joltage_1 = input
            .lines()
            .map(|line| {
                let line = line.as_bytes();
                let first = line[..(line.len() - 1)].iter().max().unwrap();
                let first_pos = line.iter().position(|byte| byte == first).unwrap();
                let first = *first - b'0';
                let second = *line[(first_pos + 1)..].iter().max().unwrap() - b'0';
                (first * 10 + second) as usize
            })
            .sum::<usize>();

        let total_output_joltage_2 = input
            .lines()
            .map(|line| {
                let mut line = line.as_bytes();

                let mut joltage: usize = 0;
                for i in (0..12).rev() {
                    let max = line[..(line.len() - i)].iter().max().unwrap();
                    let pos = line.iter().position(|byte| byte == max).unwrap();
                    line = &line[(pos + 1)..];
                    let digit = max - b'0';
                    joltage = joltage * 10 + digit as usize;
                }

                joltage
            })
            .sum::<usize>();

        return Solutions::both(
            total_output_joltage_1.to_string(),
            total_output_joltage_2.to_string(),
        );
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

        Some(vec![Example::new(
            input,
            Solutions::both("357", "3121910778619"),
        )])
    }
}
