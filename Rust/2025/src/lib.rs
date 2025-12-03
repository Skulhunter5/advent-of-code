pub mod days;

pub struct Example {
    pub input: String,
    pub solutions: Solutions,
}

pub struct Solutions {
    pub part1: Option<String>,
    pub part2: Option<String>,
}

impl Solutions {
    pub fn first<S: AsRef<str>>(solution: S) -> Self {
        Self {
            part1: Some(solution.as_ref().to_owned()),
            part2: None,
        }
    }

    pub fn second<S: AsRef<str>>(solution: S) -> Self {
        Self {
            part1: None,
            part2: Some(solution.as_ref().to_owned()),
        }
    }

    pub fn both<S: AsRef<str>>(part1: S, part2: S) -> Self {
        Self {
            part1: Some(part1.as_ref().to_owned()),
            part2: Some(part2.as_ref().to_owned()),
        }
    }
}

impl Example {
    pub fn new<S: AsRef<str>>(input: S, solutions: Solutions) -> Self {
        Self {
            input: input.as_ref().to_owned(),
            solutions,
        }
    }
}

pub trait Day {
    fn solve_both(&self, input: String) -> Solutions;
    fn get_year(&self) -> usize;
    fn get_day(&self) -> usize;
    fn input_name(&self) -> String {
        format!("{}-{:02}", self.get_year(), self.get_day())
    }
    fn get_examples(&self) -> Option<Vec<Example>> {
        None
    }
}
