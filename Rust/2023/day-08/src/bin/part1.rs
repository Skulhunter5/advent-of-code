use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("../inputs/08-input").unwrap();

    println!("{}", solve(&input));
}

fn solve(input: &String) -> usize {
    let (instructions, rest) = input.split_once("\n\n").expect("invalid input");
    let mut nodes = HashMap::new();
    rest.lines().for_each(|line| {
        let res = parse_line(line);
        nodes.insert(res.0, (res.1, res.2));
    });

    let mut cur = "AAA";
    let mut steps = 0;
    let mut i = 0;
    while cur != "ZZZ" {
        let node = nodes.get(cur).expect("node not found");
        cur = if instructions.chars().nth(i).expect("internal error: i out of range for instructions") == 'L' { node.0 } else { node.1 };
        i = (i + 1) % instructions.len();
        steps += 1;
    }

    steps
}

fn parse_line(line: &str) -> (&str, &str, &str) {
    let (node, rest) = line.split_once(" = ").expect("invalid input line");
    let (left, right) = rest[1..(rest.len() - 1)].split_once(", ").expect("invalid input line");
    (node, left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".to_string();

        assert_eq!(solve(&input), 2)
    }

    #[test]
    fn example2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)".to_string();

        assert_eq!(solve(&input), 6)
    }
}
