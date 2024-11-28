struct Sequence {
    data: Vec<Vec<i32>>,
}

impl Sequence {
    fn new(first: Vec<i32>) -> Self {
        Self { data: vec![first] }
    }

    fn predict(&mut self) -> i32 {
        loop {
            let changes = self
                .data
                .last()
                .expect("internal error: empty sequence")
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect::<Vec<_>>();
            let done = changes.iter().find(|x| **x != 0).is_none();
            if done {
                break;
            } else {
                self.data.push(changes);
            }
        }

        self.data.iter().rev().fold(0, |diff, x| {
            x.last().expect("internal error: empty sequence") + diff
        })
    }
}

fn main() {
    let input = std::fs::read_to_string("../inputs/09-input").unwrap();

    println!("{}", solve(&input));
}

fn solve(input: &String) -> i32 {
    input
        .lines()
        .map(|line| parse_line(line))
        .map(|mut sequence| sequence.predict())
        .sum()
}

fn parse_line(line: &str) -> Sequence {
    Sequence::new(
        line.split(' ')
            .map(|w| w.parse::<i32>().expect("invalid input: not a number"))
            .collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
            .to_string();

        assert_eq!(solve(&input), 114);
    }
}
