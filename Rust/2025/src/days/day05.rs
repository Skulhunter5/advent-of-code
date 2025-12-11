use crate::{Day, Example, Solutions};

pub struct Day05;

impl Day for Day05 {
    fn solve_both(&self, input: String) -> Solutions {
        let input = input.trim_end();

        let (ranges, ingredients) = input.split_once("\n\n").expect("invalid input");

        let mut ranges = ranges
            .lines()
            .map(|line| {
                let (first, last) = line.split_once('-').expect("invalid input");
                let first = first
                    .parse::<usize>()
                    .expect("invalid input: invalid range start");
                let last = last
                    .parse::<usize>()
                    .expect("invalid input: invalid range end");
                first..=last
            })
            .collect::<Vec<_>>();

        let mut i = 0;
        'outer: while i < ranges.len() - 1 {
            let mut j = i + 1;
            'inner: while j < ranges.len() {
                let a = &ranges[i];
                let b = &ranges[j];

                if a.end() < b.start() || b.end() < a.start() {
                    j += 1;
                    continue 'inner;
                }
                if a.start() <= b.start() && a.end() >= b.end() {
                    ranges.remove(j);
                    continue 'inner;
                }
                if b.start() <= a.start() && b.end() >= a.end() {
                    ranges.remove(i);
                    continue 'outer;
                }
                if a.start() >= b.start() && *a.start() <= b.end() + 1 && a.end() >= b.end() {
                    ranges[i] = *b.start()..=*a.end();
                    ranges.remove(j);
                    continue 'outer;
                }
                if b.start() >= a.start() && *b.start() <= a.end() + 1 && b.end() >= a.end() {
                    ranges[i] = *a.start()..=*b.end();
                    ranges.remove(j);
                    continue 'outer;
                }
            }
            i += 1;
        }

        let total_fresh_ingredients: usize = ranges
            .iter()
            .map(|range| range.end() - range.start() + 1)
            .sum();

        let fresh_ingredient_count = ingredients
            .lines()
            .map(|line| {
                line.parse::<usize>()
                    .expect("invalid input: invalid ingredient id")
            })
            .filter(|ingredient| {
                for range in &ranges {
                    if range.contains(ingredient) {
                        return true;
                    }
                }
                return false;
            })
            .count();

        return Solutions::both(
            fresh_ingredient_count.to_string(),
            total_fresh_ingredients.to_string(),
        );
    }

    fn get_year(&self) -> usize {
        2025
    }

    fn get_day(&self) -> usize {
        5
    }

    fn get_examples(&self) -> Option<Vec<Example>> {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        Some(vec![Example::new(input, Solutions::both("3", "14"))])
    }
}
