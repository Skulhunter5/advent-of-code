use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string("../inputs/10-input").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}

type SizeType = usize;
type MapHeight = u8;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    x: SizeType,
    y: SizeType,
}

impl Position {
    #[inline(always)]
    fn new(x: SizeType, y: SizeType) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    fn is_within_bounds(&self, width: SizeType, height: SizeType) -> bool {
        return self.x < width && self.y < height;
    }

    #[inline(always)]
    fn get_neighbors(&self) -> Vec<Position> {
        vec![
            self + Direction::Up,
            self + Direction::Right,
            self + Direction::Down,
            self + Direction::Left,
        ]
    }
}

impl std::ops::Add<Direction> for &Position {
    type Output = Position;

    #[inline(always)]
    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Right => Position::new(self.x + 1, self.y),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug)]
struct TopographicMap {
    grid: Vec<MapHeight>,
    width: SizeType,
    height: SizeType,
    zeros: Vec<Position>,
}

#[derive(Debug)]
#[allow(dead_code)]
enum ParseTopographicMapError {
    InvalidCharacter(char),
    InvalidShape,
}

impl FromStr for TopographicMap {
    type Err = ParseTopographicMapError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let width = input.find('\n').unwrap_or(input.len());
        let mut height = 0;

        let mut grid = Vec::new();
        let mut zeros = Vec::new();

        for (y, line) in input.lines().enumerate() {
            if line.len() != width {
                return Err(ParseTopographicMapError::InvalidShape);
            }

            height += 1;
            for (x, c) in line.chars().enumerate() {
                let digit = c.to_digit(10);
                if let Some(digit) = digit {
                    grid.push(digit as MapHeight);
                    if digit == 0 {
                        zeros.push(Position::new(x as SizeType, y as SizeType));
                    }
                } else {
                    return Err(ParseTopographicMapError::InvalidCharacter(c));
                }
            }
        }

        Ok(Self {
            grid,
            width: width as SizeType,
            height,
            zeros,
        })
    }
}

impl TopographicMap {
    fn count_trails(&self) -> (usize, usize) {
        let mut count = 0;
        for start in &self.zeros {
            count += self.find_trail_ends(false, &start, 0).len();
        }

        let mut count_distinct = 0;
        for start in &self.zeros {
            count_distinct += self.find_trail_ends(true, &start, 0).len();
        }

        (count, count_distinct)
    }

    fn find_trail_ends(&self, distinct: bool, pos: &Position, height: MapHeight) -> Vec<Position> {
        if height == 9 {
            return vec![pos.clone()];
        }

        let next_height = height + 1;
        let mut ends = Vec::new();
        for npos in &pos.get_neighbors() {
            if npos.is_within_bounds(self.width, self.height) {
                if self.get_at(npos.x, npos.y) == next_height {
                    let tmp = self.find_trail_ends(distinct, &npos, next_height);
                    if distinct {
                        ends.extend_from_slice(&tmp);
                    } else {
                        for end in tmp {
                            if !ends.contains(&end) {
                                ends.push(end);
                            }
                        }
                    }
                }
            }
        }
        ends
    }

    fn get_at(&self, x: SizeType, y: SizeType) -> MapHeight {
        return self.grid[y as usize * self.width as usize + x as usize];
    }
}

fn solve(input: &String) -> (usize, usize) {
    let start = std::time::Instant::now();

    let map = TopographicMap::from_str(&input).unwrap();

    let trail_counts = map.count_trails();

    let time = start.elapsed();
    println!("Time: {:?}", &time);

    trail_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732"
            .to_string();

        assert_eq!(solve(&input), (36, 81));
    }
}
