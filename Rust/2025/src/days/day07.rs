use crate::{Day, Example, Solutions};

pub struct Day07;

impl Day for Day07 {
    fn solve_both(&self, input: String) -> Solutions {
        let mut input = input.into_bytes();
        let mut lines = input.split_mut(|byte| *byte == b'\n');
        let start = lines
            .next()
            .unwrap()
            .iter()
            .position(|byte| *byte == b'S')
            .expect("invalid input: no 'S' marker");
        let lines = lines
            .filter(|line| !line.is_empty())
            .enumerate()
            .filter(|(i, _)| i % 2 != 0)
            .map(|(_, line)| line);

        let beams = &mut vec![start];
        let new_beams = &mut Vec::new();

        let split_count: usize = lines
            .map(|line| {
                let mut split_count = 0;
                for beam in &*beams {
                    let beam = *beam;
                    if line[beam] == b'^' {
                        split_count += 1;
                        if new_beams
                            .last()
                            .map(|last_beam| *last_beam != beam - 1)
                            .unwrap_or(true)
                        {
                            new_beams.push(beam - 1);
                        }
                        new_beams.push(beam + 1);
                    } else {
                        if new_beams
                            .last()
                            .map(|last_beam| *last_beam != beam)
                            .unwrap_or(true)
                        {
                            new_beams.push(beam);
                        }
                    }
                }
                std::mem::swap(beams, new_beams);
                new_beams.clear();

                split_count
            })
            .sum();

        return Solutions::first(split_count.to_string());
    }

    fn get_year(&self) -> usize {
        2025
    }

    fn get_day(&self) -> usize {
        7
    }

    fn get_examples(&self) -> Option<Vec<Example>> {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

        Some(vec![Example::new(input, Solutions::both("21", "40"))])
    }
}
