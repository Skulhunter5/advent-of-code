#![feature(linked_list_cursors)]

use std::collections::LinkedList;

fn main() {
    let input = std::fs::read_to_string("../inputs/11-input").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}

#[derive(Debug)]
struct Stones {
    stones: LinkedList<u64>,
}

impl Stones {
    #[inline(always)]
    fn len(&self) -> usize {
        return self.stones.len();
    }

    #[inline(always)]
    fn blink(&mut self, count: usize) {
        for i in 0..count {
            self.blink_once();
            println!("{}: {}", i, self.stones.len());
        }
    }

    fn blink_once(&mut self) {
        let mut stones = self.stones.cursor_front_mut();
        while let Some(stone) = stones.current() {
            if *stone == 0 {
                *stone = 1;
            } else {
                let mut power = 10;
                let mut j = 1;
                while power <= *stone {
                    power *= 10;
                    j += 1;
                }
                if j & 1 == 0 {
                    j /= 2;
                    while j > 0 {
                        power /= 10;
                        j -= 1;
                    }
                    let left = *stone / power;
                    let right = *stone - left * power;
                    *stone = right;
                    stones.insert_before(left);
                } else {
                    *stone = stone
                        .checked_mul(2024)
                        .expect("internal error: stone number type too small");
                    // unchecked multiplication is technically slightly faster although it doesn't
                    // really make a difference in this case because the bottleneck comes from
                    // somewhere else
                    //*stone *= 2024;
                }
            }
            stones.move_next();
        }
    }
}

fn solve(input: &String) -> (usize, usize) {
    let start = std::time::Instant::now();

    let mut line = parse_line(&input);
    line.blink(25);
    let len25 = line.len();

    let time = start.elapsed();
    println!("Time: {:?}", &time);

    (len25, 0)
}

fn parse_line(input: &String) -> Stones {
    let stones = input
        .trim()
        .split(' ')
        .map(|word| word.parse::<u64>().expect("invalid input: not a number"))
        //.collect::<Vec<_>>();
        .collect::<LinkedList<_>>();

    Stones { stones }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "125 17".to_string();

        assert_eq!(solve(&input), (55312, 0));
    }
}
