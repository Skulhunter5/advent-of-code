fn main() {
    let input = std::fs::read_to_string("../inputs/06-input").unwrap();

    println!("{}", solve(&input));
}

fn solve(input: &String) -> String {
    let lines = input.lines().take(2).collect::<Vec<_>>();

    let times = parse_line(lines[0]);
    let distances = parse_line(lines[1]);

    times
        .into_iter()
        .zip(distances)
        .map(|(time, record)| {
            (1..time)
                .filter(|start| ((time - start) * start) > record)
                .count() as u32
        })
        .product::<u32>()
        .to_string()
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split(' ')
        .skip(1)
        .filter(|token| !token.is_empty())
        .map(|token| token.parse::<u32>().unwrap())
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "Time:      7  15   30
Distance:  9  40  200"
            .to_string();

        assert_eq!(solve(&input), "288")
    }
}
