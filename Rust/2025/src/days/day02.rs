use std::{iter::Map, ops::RangeInclusive, str::Split};

use crate::{Day, Example, Solutions};

pub struct Day02;

impl Day02 {
    fn parse_input(input: &str) -> Map<Split<'_, char>, fn(&str) -> RangeInclusive<usize>> {
        input.trim_end().split(',').map(|part| {
            let (first_id, last_id) = part
                .split_once('-')
                .expect("invalid input: invalid range format");
            let first_id = first_id
                .parse::<usize>()
                .expect("invalid input: invalid first ID");
            let last_id = last_id
                .parse::<usize>()
                .expect("invalid input: invalid last ID");
            first_id..=last_id
        })
    }

    fn is_valid_id_1(id: &usize) -> bool {
        let id = id.to_string();

        if id.len() % 2 == 0 {
            let middle = id.len() / 2;
            return id[..middle] != id[middle..];
        }

        return true;
    }

    fn is_valid_id_2(id: &usize) -> bool {
        let id = id.to_string();

        for x in 1..=(id.len() / 2) {
            if id.len() % x == 0 {
                let pattern = &id[..x];
                let mut invalid = true;
                for i in 0..(id.len() / x) {
                    let part = &id[(i * x)..((i + 1) * x)];
                    if part != pattern {
                        invalid = false;
                        break;
                    }
                }
                if invalid {
                    return false;
                }
            }
        }

        return true;
    }
}

impl Day for Day02 {
    fn solve_both(&self, input: String) -> Solutions {
        let ranges = Self::parse_input(&input);
        let mut invalid_sum_1 = 0;
        let mut invalid_sum_2 = 0;

        for range in ranges {
            for id in range {
                if !Day02::is_valid_id_1(&id) {
                    invalid_sum_1 += id;
                }
                if !Day02::is_valid_id_2(&id) {
                    invalid_sum_2 += id;
                }
            }
        }

        return Solutions::both(invalid_sum_1.to_string(), invalid_sum_2.to_string());
    }

    fn get_year(&self) -> usize {
        2025
    }

    fn get_day(&self) -> usize {
        2
    }

    fn get_examples(&self) -> Option<Vec<Example>> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        Some(vec![Example::new(
            input,
            Solutions::both("1227775554", "4174379265"),
        )])
    }
}
