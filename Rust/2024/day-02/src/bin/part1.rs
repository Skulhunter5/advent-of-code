fn main() {
    let input = std::fs::read_to_string("../inputs/02-input").unwrap();

    println!("{}", solve(&input));
}

fn solve(input: &String) -> usize {
    input
        .lines()
        .map(|line| {
            let mut values = line
                .split(" ")
                .map(|x| x.parse::<i8>().expect("invalid input: not a number"));
            let first = values.next().expect("invalid input: empty line");
            let second = values.next().expect("invalid input: line with one item");
            let dir = {
                let dist = second - first;
                if dist == 0 || dist.abs() > 3 {
                    0
                } else {
                    dist.signum()
                }
            };

            let safe = values
                .try_fold(second, |last, x| {
                    let diff = x - last;
                    if diff.abs() > 3 || diff.signum() != dir {
                        None
                    } else {
                        Some(x)
                    }
                })
                .is_some();

            safe as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
            .to_string();

        assert_eq!(solve(&input), 2);
    }
}
