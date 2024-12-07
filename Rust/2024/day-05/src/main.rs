use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("../inputs/05-input").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}

fn handle_update(update: &Vec<u8>, comes_after: &[Vec<u8>; 100]) -> (usize, usize) {
    let correct = check_update(&update, &comes_after);
    if correct {
        return (update[update.len() / 2] as usize, 0);
    } else {
        let update = fix_update(&update, &comes_after);

        return (0, update[update.len() / 2] as usize);
    }
}

fn check_update(update: &Vec<u8>, comes_after: &[Vec<u8>; 100]) -> bool {
    for i in 0..update.len() {
        let page = update[i];
        for other in &update[i..] {
            if comes_after[page as usize].contains(other) {
                return false;
            }
        }
    }
    return true;
}

fn fix_update(update: &Vec<u8>, comes_after: &[Vec<u8>; 100]) -> Vec<u8> {
    let mut corrected = update.clone();

    corrected.sort_by(|a, b| {
        if comes_after[*a as usize].contains(b) {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    corrected
}

fn solve(input: &String) -> (usize, usize) {
    let (rules, updates) = parse_input(&input);
    let mut comes_after = [const { Vec::new() }; 100];
    for rule in rules {
        comes_after[rule.1 as usize].push(rule.0);
    }

    let mut total = (0, 0);
    for update in updates {
        let res = handle_update(&update, &comes_after);
        total = (total.0 + res.0, total.1 + res.1);
    }

    total
}

fn parse_input(input: &String) -> (Vec<(u8, u8)>, Vec<Vec<u8>>) {
    let (rules, updates) = input.split_once("\n\n").expect("invalid input: only one block of text");
    let rules = rules.lines().map(|line| {
        let (left, right) = line.split_once('|').expect("invalid input: invalid rule format");
        let left = left.parse::<u8>().expect("invalid input: format");
        let right = right.parse::<u8>().expect("invalid input: format");
        (left, right)
    }).collect::<Vec<_>>();
    let updates = updates.lines().map(|line| line.split(',').map(|page| page.parse::<u8>().expect("invalid input: not a number")).collect::<Vec<_>>()).collect::<Vec<_>>();

    (rules, updates)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47".to_string();

        assert_eq!(solve(&input), (143, 123));
    }
}
