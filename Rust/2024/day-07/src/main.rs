type DailyPartRaw = u8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum DailyPart {
    One = 1,
    Two = 2,
}

impl DailyPart {
    #[inline]
    const fn raw(&self) -> DailyPartRaw {
        return *self as DailyPartRaw;
    }

    #[allow(dead_code)]
    const fn from_raw(raw: DailyPartRaw) -> DailyPart {
        match raw {
            1 => DailyPart::One,
            2 => DailyPart::Two,
            _ => panic!("invalid DailyPartRaw value"),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("../inputs/07-input").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    #[inline(always)]
    fn rotate<const PART: DailyPartRaw>(&self) -> (Self, bool) {
        match PART {
            1 => match self {
                Operator::Add => (Operator::Multiply, false),
                Operator::Multiply => (Operator::Add, true),
                Operator::Concatenate => unreachable!(),
            },
            2 => match self {
                Operator::Add => (Operator::Multiply, false),
                Operator::Multiply => (Operator::Concatenate, false),
                Operator::Concatenate => (Operator::Add, true),
            },
            _ => panic!("invalid DailyPartRaw value"),
        }
    }
}

#[derive(Debug)]
struct Operators<const PART: DailyPartRaw> {
    current: Option<Vec<Operator>>,
    count: usize,
    skipped: bool,
    done: bool,
}

impl<const PART: DailyPartRaw> Operators<PART> {
    fn new(count: usize) -> Self {
        Self {
            current: None,
            count,
            skipped: false,
            done: false,
        }
    }

    fn next(&mut self) -> Option<&Vec<Operator>> {
        if self.done {
            return None;
        }
        if let Some(ref mut current) = self.current {
            if self.skipped {
                self.skipped = false;
                return self.current.as_ref();
            }

            for i in 0..self.count {
                let i = self.count - 1 - i;
                let (new_operator, wrapped) = current[i].rotate::<PART>();
                current[i] = new_operator;
                if !wrapped {
                    return self.current.as_ref();
                }
            }
            self.done = true;
            return None;
        } else {
            self.current = Some(vec![Operator::Add; self.count]);
        }
        self.current.as_ref()
    }

    fn skip_at(&mut self, pos: usize) {
        if self.done {
            return;
        }
        if let Some(ref mut current) = self.current {
            self.skipped = true;

            for i in (self.count - 1 - pos)..self.count {
                let i = self.count - 1 - i;
                let (new_operator, wrapped) = current[i].rotate::<PART>();
                current[i] = new_operator;
                if !wrapped {
                    return;
                }
            }
            self.done = true;
        }
    }
}

#[derive(Debug)]
struct Equation {
    result: usize,
    factors: Vec<usize>,
}

impl Equation {
    fn new(result: usize, factors: Vec<usize>) -> Self {
        Self { result, factors }
    }

    fn is_possible(&self) -> Option<DailyPart> {
        if self.is_possible_part::<{ DailyPart::One.raw() }>() {
            return Some(DailyPart::One);
        } else if self.is_possible_part::<{ DailyPart::Two.raw() }>() {
            return Some(DailyPart::Two);
        } else {
            return None;
        }
    }

    fn is_possible_part<const PART: DailyPartRaw>(&self) -> bool {
        let mut cartesian_product = Operators::<PART>::new(self.factors.len() - 1);

        'outer: while let Some(operators) = cartesian_product.next() {
            let mut acc = self.factors[0] as usize;
            for (i, factor) in self.factors[1..].iter().enumerate() {
                let tmp = if PART == DailyPart::One.raw() {
                    &[acc + *factor, acc * *factor] as &[usize]
                } else {
                    let mut power = 10;
                    while *factor >= power {
                        power *= 10;
                    }
                    let concat = acc * power + *factor;
                    &[acc + *factor, acc * *factor, concat] as &[usize]
                };
                acc = tmp[operators[i] as usize];

                if acc > self.result {
                    cartesian_product.skip_at(i);
                    continue 'outer;
                }
            }
            if acc == self.result {
                return true;
            }
        }

        return false;
    }
}

fn solve(input: &String) -> (usize, usize) {
    let start = std::time::Instant::now();

    let equations = parse_input(&input);

    let results = equations
        .iter()
        .map(|equation| match equation.is_possible() {
            Some(part) => (
                if part == DailyPart::One {
                    equation.result
                } else {
                    0
                },
                equation.result,
            ),
            None => (0, 0),
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    let time = start.elapsed();
    println!("Time: {:?}", &time);

    results
}

fn parse_input(input: &String) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let (result, rest) = line
                .split_once(": ")
                .expect("invalid input: invalid line format");
            let result = result
                .parse::<usize>()
                .expect("invalid input: test value not a number");
            let factors = rest
                .split(' ')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            Equation::new(result, factors)
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
            .to_string();

        assert_eq!(solve(&input), (3749, 11387));
    }
}
