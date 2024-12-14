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
    Visited { directions: Directions },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Directions {
    dirs: usize,
}

impl Directions {
    fn has_direction(&self, dir: Direction) -> bool {
        ((self.dirs >> dir as usize) & 1) != 0
    }

    fn set_direction(&mut self, dir: Direction) {
        self.dirs |= 1 << dir as usize;
    }
}

impl From<Direction> for Directions {
    fn from(dir: Direction) -> Self {
        Self {
            dirs: 1 << dir as usize,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    guard: Guard,
}

impl Map {
    fn trace_route(&mut self) -> Option<usize> {
        let mut visited = 1;
        loop {
            let (x, y) = self.guard.get_next();
            if x >= self.width || y >= self.height {
                break;
            }

            if self.get(x, y) == Tile::Obstruction {
                self.guard.rotate();
                let dir = self.guard.dir;
                if let Tile::Visited { directions } = self.get_mut(self.guard.x, self.guard.y) {
                    directions.set_direction(dir);
                } else {
                    unreachable!("guard can't be standing on a tile he hasn't visited");
                }
            } else {
                let dir = self.guard.dir;
                if self.get(x, y) == Tile::None {
                    self.set(
                        x,
                        y,
                        Tile::Visited {
                            directions: Directions::from(dir),
                        },
                    );
                    visited += 1;
                } else if let Tile::Visited { directions } = self.get_mut(x, y) {
                    if directions.has_direction(dir) {
                        return None;
                    }
                    directions.set_direction(dir);
                }
                self.guard.advance();
            }
        }

        Some(visited)
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> Tile {
        return self.grid[y][x];
    }

    #[inline]
    fn get_mut(&mut self, x: usize, y: usize) -> &mut Tile {
        return &mut self.grid[y][x];
    }

    #[inline]
    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.grid[y][x] = tile;
    }
}

fn count_possible_obstructions(map: &Map, tested_map: &Map) -> usize {
    let mut count = 0;

    for x in 0..map.width {
        for y in 0..map.height {
            if map.get(x, y) == Tile::Obstruction || (map.guard.x == x && map.guard.y == y) {
                continue;
            }
            // use the data from part one to skip tiles that will never be visited during a normal
            // route and can therefore not change the route
            if tested_map.get(x, y) == Tile::None {
                continue;
            }
            let mut new_map = map.clone();
            new_map.set(x, y, Tile::Obstruction);
            let res = new_map.trace_route();
            if res.is_none() {
                count += 1;
            }
        }
    }

    count
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
                        let dir = match c {
                            '^' => Direction::Up,
                            '>' => Direction::Right,
                            'v' => Direction::Down,
                            '<' => Direction::Left,
                            _ => unreachable!(),
                        };
                        guard = Some(Guard::new(x, y, dir));
                        Tile::Visited {
                            directions: Directions::from(dir),
                        }
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
    let start = std::time::Instant::now();

    let map = Map::try_from(input).unwrap();
    let mut test_map = map.clone();
    let visited_tiles = test_map
        .trace_route()
        .expect("input for part 1 should not contain a loop");
    let possible_obstructions = count_possible_obstructions(&map, &test_map);

    let time = start.elapsed();
    println!("Time: {:?}", time);

    (visited_tiles, possible_obstructions)
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

        assert_eq!(solve(&input), (41, 6));
    }
}
