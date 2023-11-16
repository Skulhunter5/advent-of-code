use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/05-input.txt").unwrap();

    let solution = solve(&input);
    println!("Part 1: {}", solution.0);
    println!("Part 2: {}", solution.1);
}

fn solve(input: &String) -> (u64, u64) {
    (part1(input), part2(input))
}

fn part1(input: &String) -> u64 {
    let mut count = 0;
    for line in input.lines() {
        let enough_vowels = line.chars().filter(|c| "aeiou".contains(*c)).count() >= 3;
        if !enough_vowels {
            continue;
        }
        if line.contains("ab") || line.contains("cd") || line.contains("pq") || line.contains("xy") {
            continue;
        }
        for window in line.chars().collect::<Vec<_>>().windows(2) {
            if window[0] == window[1] {
                count += 1;
                break;
            }
        }
    }
    count
}

fn part2(input: &String) -> u64 {
    let mut count = 0;
    for line in input.lines() {
        let chars = line.chars().collect::<Vec<_>>();
        let predicate2 = chars.windows(3).find(|win| win[0] == win[2]).is_some();
        if !predicate2 {
            continue;
        }
        'outer:for i in 0..(chars.len() - 3) {
            let pair = format!("{}{}", chars[i], chars[i + 1]);
            for j in (i + 2)..(chars.len() - 1) {
                if pair == format!("{}{}", chars[j], chars[j + 1]) {
                    count += 1;
                    break 'outer;
                }
            }
        }
    }
    count
}
