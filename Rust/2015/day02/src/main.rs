use std::{fs, cmp::min};

fn main() {
    let input = fs::read_to_string("../inputs/02-input.txt").unwrap();

    let solution = solve(&input);
    println!("Part 1: {}", solution.0);
    println!("Part 2: {}", solution.1);
}

fn solve(input: &String) -> (u64, u64) {
    let mut total_paper: u64 = 0;
    let mut total_ribbon: u64 = 0;
    for line in input.lines() {
        let mut sides = line
            .split("x")
            .map(|x| x.parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        sides.sort_unstable();

        let l = sides[0];
        let w = sides[1];
        let h = sides[2];

        let paper = 2 * l * w + 2 * l * h + 2 * w * h;
        let slack = min(l * w, min(l * h, w * h));

        let bow = l * w * h;
        let ribbon = 2 * l + 2 * w;

        total_paper += paper + slack;
        total_ribbon += bow + ribbon;
    }
    (total_paper, total_ribbon)
}
