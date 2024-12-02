fn main() {
    let input = std::fs::read_to_string("../inputs/02-input").unwrap();

    println!("{}", solve(&input));
}

fn check(values: &[i8]) -> bool {
    (0..values.len())
        .position(|skip| {
            let first = if skip == 0 { 1 } else { 0 };
            let second = if first + 1 == skip {
                first + 2
            } else {
                first + 1
            };
            let dir = {
                let diff = values[second] - values[first];
                if diff == 0 || diff.abs() > 3 {
                    return false;
                } else {
                    diff.signum()
                }
            };

            let mut i = second + 1;
            let mut last = values[second];
            while i < values.len() {
                if i == skip {
                    i += 1;
                    continue;
                }

                let diff = values[i] - last;
                if diff.abs() > 3 || diff.signum() != dir {
                    return false;
                } else {
                    last = values[i];
                }

                i += 1;
            }
            return true;
        })
        .is_some()
}

fn solve(input: &String) -> usize {
    input
        .lines()
        .map(|line| {
            let values = line
                .split(" ")
                .map(|x| x.parse::<i8>().expect("invalid input: not a number"))
                .collect::<Vec<_>>();

            check(&values[..]) as usize
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

        assert_eq!(solve(&input), 4);
    }
}
