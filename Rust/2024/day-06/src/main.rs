fn main() {
    let input = std::fs::read_to_string("../inputs/06-input").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    None,
    Obstruction,
    Visited,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

struct Guard {
    x: usize,
    y: usize,
    dir: Direction,
}

impl Guard {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Self { x, y, dir }
    }

    fn rotate(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }

    fn get_next(&self) -> (usize, usize) {
        match self.dir {
            Direction::Up => (self.x, self.y - 1),
            Direction::Right => (self.x + 1, self.y),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
        }
    }

    fn advance(&mut self) {
        (self.x, self.y) = self.get_next();
    }
}

struct Map {
    grid: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    guard: Guard,
}

impl Map {
    fn trace_route(&mut self) -> usize {
        let mut visited = 1;
        // only requires
        loop {
            let (x, y) = self.guard.get_next();
            if x >= self.width || y >= self.height {
                break;
            }

            if self.get(x, y) == Tile::Obstruction {
                self.guard.rotate();
            } else {
                if self.get(x, y) == Tile::None {
                    self.set(x, y, Tile::Visited);
                    visited += 1;
                }
                self.guard.advance();
            }
        }

        visited
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> Tile {
        return self.grid[y][x];
    }

    #[inline]
    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.grid[y][x] = tile;
    }
}

impl TryFrom<&String> for Map {
    type Error = String;

    fn try_from(input: &String) -> Result<Self, Self::Error> {
        let mut guard = None;
        let mut grid = Vec::new();
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                row.push(match c {
                    '#' => Tile::Obstruction,
                    '.' => Tile::None,
                    '^' | '>' | 'v' | '<' => {
                        guard = Some(Guard::new(
                            x,
                            y,
                            match c {
                                '^' => Direction::Up,
                                '>' => Direction::Right,
                                'v' => Direction::Down,
                                '<' => Direction::Left,
                                _ => unreachable!(),
                            },
                        ));
                        Tile::Visited
                    }
                    x => return Err(format!("invalid input: invalid character '{}", x)),
                });
            }
            grid.push(row);
        }

        let height = grid.len();
        let width = grid[0].len();
        if grid.iter().position(|row| row.len() != width).is_some() {
            return Err("invalid input: map not a square".to_string());
        }

        Ok(Self {
            grid,
            width,
            height,
            guard: guard.expect("invalid input: no guard"),
        })
    }
}

fn solve(input: &String) -> (usize, usize) {
    let mut map = Map::try_from(input).unwrap();

    (map.trace_route(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
            .to_string();

        assert_eq!(solve(&input), (41, 0));
    }
}
