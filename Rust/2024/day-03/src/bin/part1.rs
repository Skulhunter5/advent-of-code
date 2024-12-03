use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("../inputs/03-input").unwrap();

    println!("{}", solve(&input));
}

fn solve(input: &String) -> usize {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|c| c.extract())
                .map(|(_, [a, b])| {
                    let a = a.parse::<usize>().expect("incorrect regex match");
                    let b = b.parse::<usize>().expect("incorrect regex match");
                    a * b
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();

        assert_eq!(solve(&input), 161);
    }
}
