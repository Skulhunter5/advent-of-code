use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("../inputs/06-input.txt").unwrap();

    let solution = solve(&input);
    println!("Part 1: {}", solution.0);
    println!("Part 2: {}", solution.1);
}

fn solve(input: &String) -> (usize, usize) {
    // Needs to be stored in a Box because the overflows otherwise
    let mut lights = Box::new([[false; 1000]; 1000]);
    let mut brightnesses = Box::new([[0; 1000]; 1000]);

    for line in input.lines() {
        let re = Regex::new(r"(turn on|turn off|toggle) ([0-9]+,[0-9]+) through ([0-9]+,[0-9]+)").unwrap();
        for (_, [action, first, second]) in re.captures_iter(line).map(|c| c.extract()) {
            let a = first.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
            let b = second.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
            if action.starts_with("turn ") {
                let after = if &action[5..] == "on" { true } else { false };
                for i in a[0]..=b[0] {
                    for j in a[1]..=b[1] {
                        lights[i][j] = after;
                        if after {
                            brightnesses[i][j] += 1;
                        } else if brightnesses[i][j] > 0 {
                            brightnesses[i][j] -= 1;
                        }
                    }
                }
            } else {
                for i in a[0]..=b[0] {
                    for j in a[1]..=b[1] {
                        lights[i][j] = !lights[i][j];
                        brightnesses[i][j] += 2;
                    }
                }
            }
        }
    }
    let on_count = lights.iter().map(|line| line.iter().filter(|light| **light).count()).sum::<usize>();
    let total_brightness = brightnesses.iter().map(|line| line.iter().sum::<usize>()).sum::<usize>();
    (on_count, total_brightness)
}
