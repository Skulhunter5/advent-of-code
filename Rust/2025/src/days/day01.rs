use crate::{Day, Example, Solutions};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Action {
    direction: Direction,
    amount: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            x => Err(format!("invalid direction character: '{}'", x)),
        }
    }
}

pub struct Day01;

impl Day01 {
    fn parse_input<'a>(input: &'a str) -> std::iter::Map<std::str::Lines<'a>, fn(&str) -> Action> {
        input.trim_end().lines().map(|line| {
            let dir = line.chars().next().expect("invalid input: empty line");
            let amount = line[dir.len_utf8()..].parse::<u16>().expect("invalid input: invalid rotation amount");
            let direction = Direction::try_from(dir).expect("invalid input");
            Action { direction, amount }
        })
    }
}

impl Day for Day01 {
    fn solve_both(&self, input: String) -> Solutions {
        let actions = Self::parse_input(&input);

        let mut current: i16 = 50;
        let mut count1: usize = 0;
        let mut count2: usize = 0;
        for action in actions {
            match action.direction {
                Direction::Left => {
                    for _ in 0..action.amount {
                        if current == 1 {
                            count2 += 1;
                        }
                        if current == 0 {
                            current = 99;
                        } else {
                            current -= 1;
                        }
                    }
                }
                Direction::Right => {
                    for _ in 0..action.amount {
                        if current == 99 {
                            current = 0;
                            count2 += 1;
                        } else {
                            current += 1;
                        }
                    }
                }
            }
            if current == 0 {
                count1 += 1;
            }
        }

        return Solutions::both(count1.to_string(), count2.to_string());
    }

    fn get_year(&self) -> usize {
        2025
    }

    fn get_day(&self) -> usize {
        1
    }

    fn get_examples(&self) -> Option<Vec<Example>> {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        Some(vec![
            Example::new(input, Solutions::both("3", "6")),
        ])
    }
}
