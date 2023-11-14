use std::{fs, collections::HashSet};

fn main() {
    let input = fs::read_to_string("../inputs/03-input.txt").unwrap();

    let solution = solve(&input);
    println!("Part 1: {}", solution.0);
    println!("Part 2: {}", solution.1);
}

fn solve(input: &String) -> (usize, usize) {
    (part1(input), part2(input))
}

fn part1(input: &String) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    visited.insert((0, 0));
    for c in input.lines().next().unwrap().chars() {
        match c {
            '^' => y += 1,
            '>' => x += 1,
            'v' => y -= 1,
            '<' => x -= 1,
            _ => {
                panic!("Invalid character: '{c}'");
            }
        }
        visited.insert((x, y));
    }
    visited.len()
}

fn part2(input: &String) -> usize {
    let mut visited: HashSet<(u64, u64)> = HashSet::new();
    let mut turn = 0;
    let mut position = vec![(0, 0), (0, 0)];
    visited.insert((0, 0));
    for c in input.lines().next().unwrap().chars() {
        match c {
            '^' => position[turn].1 += 1,
            '>' => position[turn].0 += 1,
            'v' => position[turn].1 -= 1,
            '<' => position[turn].0 -= 1,
            _ => {
                panic!("Invalid character: '{c}'");
            }
        }
        visited.insert((position[turn].0, position[turn].1));
        turn = if turn == 0 { 1 } else { 0 };
    }
    visited.len()
}
