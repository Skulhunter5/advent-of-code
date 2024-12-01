use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("../inputs/01-input").unwrap();

    println!("{}", solve(&input));
}

fn solve(input: &String) -> i32 {
    let mut counts = HashMap::new();

    let left = input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").expect("invalid input: line format");
            let left = left.parse::<i32>().expect("invalid input");
            let right = right.parse::<i32>().expect("invalid input");
            counts.entry(right).and_modify(|e| *e += 1).or_insert(1);

            left
        })
        .collect::<Vec<_>>();

    left.iter().map(|x| counts.get(x).unwrap_or(&0) * x).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3"
            .to_string();

        assert_eq!(solve(&input), 31);
    }
}
