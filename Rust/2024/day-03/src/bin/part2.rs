use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("../inputs/03-input").unwrap();

    println!("{}", solve(&input));
}

fn solve(input: &String) -> usize {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();

    let mut enabled = true;
    input
        .lines()
        .map(|line| {
            let mut res = 0;

            let mut start = if enabled {
                0
            } else {
                if let Some(index) = line.find("do()") {
                    index + "do()".len()
                } else {
                    return 0;
                }
            };
            loop {
                let end = if let Some(offset) = line[start..].find("don't()") {
                    Some(start + offset)
                } else {
                    None
                };
                res += re
                    .captures_iter(&line[start..end.unwrap_or(line.len())])
                    .map(|c| c.extract())
                    .map(|(_, [a, b])| {
                        let a = a.parse::<usize>().expect("incorrect regex match");
                        let b = b.parse::<usize>().expect("incorrect regex match");
                        a * b
                    })
                    .sum::<usize>();
                let end = if let Some(index) = end {
                    index + "don't()".len()
                } else {
                    enabled = true;
                    break;
                };

                start = if let Some(index) = line[end..].find("do()") {
                    end + index + "do()".len()
                } else {
                    enabled = false;
                    break;
                };
            }

            res
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();

        assert_eq!(solve(&input), 48);
    }
}
