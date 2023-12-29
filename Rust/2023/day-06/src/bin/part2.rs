fn main() {
    let input = std::fs::read_to_string("../inputs/06-input").unwrap();

    println!("{}", solve(&input));
}

fn solve(input: &String) -> String {
    let lines = input
        .lines()
        .take(2)
        .collect::<Vec<_>>();

    let time = parse_line(lines[0]);
    let record = parse_line(lines[1]);

    (1..time)
        .filter(|start| ((time - start) * start) > record)
        .count()
        .to_string()
}

fn parse_line(line: &str) -> u64 {
    line
        .replace(" ", "")
        .split_once(':')
        .unwrap()
        .1
        .parse::<u64>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "Time:      7  15   30
Distance:  9  40  200"
            .to_string();

        assert_eq!(solve(&input), "71503")
    }
}
