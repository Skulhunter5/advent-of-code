use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/02-input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
}

fn part1(input: &String) -> u64 {
    let mut total: u64 = 0;
    for line in input.lines() {
        let sides = line.split("x").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
        let faces = vec![sides[0]*sides[1], sides[0]*sides[2], sides[1]*sides[2]];
        total += faces.iter().map(|x| x * 2).sum::<u64>() + faces.iter().min().unwrap();
    }
    total
}
