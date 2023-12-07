fn main() {
    let input = std::fs::read_to_string("../inputs/02-input").unwrap();

    println!("{}", solve(&input));
}

struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

impl std::ops::Add for Draw {
    type Output = Draw;

    fn add(self, rhs: Self) -> Self::Output {
        Draw {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Draw {
    fn max(&self, other: &Self) -> Self {
        Draw {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }

    fn get_power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn solve(input: &String) -> String {
    input.lines()
        .map(|line| {
            // "Game {id}: {count} {color}, {count} {color}; {count} {color}, ..."
            // -> remaining="{id}: {count} {color}, {count} {color}; {count} {color}, ..."
            let (_, remaining) = line.split_once(' ').unwrap();
            // "{id}: {count} {color}, {count} {color}; {count} {color}, ..."
            // -> remaining="{count} {color}, {count} {color}; {count} {color}, ..."
            let (_, remaining) = remaining.split_once(": ").unwrap();
            
            let draws = remaining.split("; ").map(|draw_str| {
                // draw_str="{count} {color}, {count} {color}"
                draw_str.split(", ").map(|batch| {
                    let (count, color) = batch.split_once(' ').unwrap();
                    let count = count.parse::<u32>().unwrap();
                    match color {
                        "red" => Draw {
                            red: count,
                            green: 0,
                            blue: 0,
                        },
                        "green" => Draw {
                            red: 0,
                            green: count,
                            blue: 0,
                        },
                        "blue" => Draw {
                            red: 0,
                            green: 0,
                            blue: count,
                        },
                        c => {
                            panic!("invalid color: {c}");
                        }
                    }
                })
                .fold(Draw { red: 0, green: 0, blue: 0 }, |a, b| a + b)
            }).collect::<Vec<_>>();

            draws.iter()
                .fold(Draw { red: 0, green: 0, blue: 0 }, |a, b| a.max(b))
                .get_power()
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        assert_eq!(solve(&input), "2286");
    }
}
