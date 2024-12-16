use std::{fmt::Display, str::FromStr};

fn main() {
    let input = std::fs::read_to_string("../inputs/15-input").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    None,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Vec2 {
    #[inline(always)]
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add<Direction> for Vec2 {
    type Output = Vec2;

    fn add(self, dir: Direction) -> Self::Output {
        match dir {
            Direction::Up => Vec2::new(self.x, self.y - 1),
            Direction::Right => Vec2::new(self.x + 1, self.y),
            Direction::Down => Vec2::new(self.x, self.y + 1),
            Direction::Left => Vec2::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug)]
struct Warehouse {
    grid: Vec<Tile>,
    width: usize,
    height: usize,
    robot: Vec2,
}

#[derive(Debug)]
#[allow(dead_code)]
enum ParseWarehouseError {
    InvalidShape,
    UnexpectedCharacter(char),
    MultipleRobots,
}

impl FromStr for Warehouse {
    type Err = ParseWarehouseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let width = input.find('\n').expect("invalid input");
        let mut height = 0;
        let mut grid = Vec::new();
        let mut robot = None;

        for (y, line) in input.lines().enumerate() {
            if line.len() != width {
                return Err(ParseWarehouseError::InvalidShape);
            }

            height += 1;
            for (x, c) in line.chars().enumerate() {
                grid.push(match c {
                    '#' => Tile::Wall,
                    'O' => Tile::Box,
                    '@' => {
                        if robot.is_some() {
                            return Err(ParseWarehouseError::MultipleRobots);
                        }
                        robot = Some(Vec2::new(x as isize, y as isize));
                        Tile::None
                    }
                    '.' => Tile::None,
                    x => return Err(ParseWarehouseError::UnexpectedCharacter(x)),
                });
            }
        }

        let robot = robot.expect("invalid input: no robot");

        Ok(Self {
            grid,
            width,
            height,
            robot,
        })
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = if Vec2::new(x as isize, y as isize) == self.robot {
                    '@'
                } else {
                    match self.get(x, y) {
                        Tile::None => '.',
                        Tile::Box => 'O',
                        Tile::Wall => '#',
                        Tile::BoxLeft => '[',
                        Tile::BoxRight => ']',
                    }
                };
                write!(f, "{c}")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Warehouse {
    #[inline]
    fn robot_move(&mut self, dir: Direction) {
        let check_move = self.check_move(self.robot, dir);
        if check_move {
            self.do_move(self.robot, dir);
        }
        return;
    }

    fn check_move(&self, pos: Vec2, dir: Direction) -> bool {
        let next_pos = pos + dir;
        if pos == self.robot {
            return self.check_move(next_pos, dir);
        }

        let tile = self.get(pos.x as usize, pos.y as usize);
        let other_pos = match tile {
            Tile::None => return true,
            Tile::Wall => return false,
            Tile::Box => return self.check_move(next_pos, dir),
            Tile::BoxLeft => Vec2::new(pos.x + 1, pos.y),
            Tile::BoxRight => Vec2::new(pos.x - 1, pos.y),
        };
        assert!({
            let other_tile = self.get(other_pos.x as usize, other_pos.y as usize);
            other_tile == Tile::BoxLeft || other_tile == Tile::BoxRight
        });
        let other_next_pos = other_pos + dir;
        if dir == Direction::Right || dir == Direction::Left {
            return self.check_move(other_next_pos, dir);
        } else if dir == Direction::Up || dir == Direction::Down {
            return self.check_move(next_pos, dir) && self.check_move(other_next_pos, dir);
        } else {
            unreachable!();
        }
    }

    fn do_move(&mut self, pos: Vec2, dir: Direction) {
        let next_pos = pos + dir;
        if pos == self.robot {
            self.do_move(next_pos, dir);
            self.robot = next_pos;
            return;
        }

        let tile = self.get(pos.x as usize, pos.y as usize);
        let other_pos = match tile {
            Tile::None => return,
            Tile::Wall => return,
            Tile::Box => {
                self.do_move(next_pos, dir);
                self.set(pos.x as usize, pos.y as usize, Tile::None);
                self.set(next_pos.x as usize, next_pos.y as usize, Tile::Box);
                return;
            },
            Tile::BoxLeft => Vec2::new(pos.x + 1, pos.y),
            Tile::BoxRight => Vec2::new(pos.x - 1, pos.y),
        };
        let other_tile = self.get(other_pos.x as usize, other_pos.y as usize);
        assert!(other_tile == Tile::BoxLeft || other_tile == Tile::BoxRight);

        let other_next_pos = other_pos + dir;
        if dir != Direction::Right && dir != Direction::Left {
            self.do_move(next_pos, dir);
        }
        self.do_move(other_next_pos, dir);
        self.set(pos.x as usize, pos.y as usize, Tile::None);
        self.set(other_pos.x as usize, other_pos.y as usize, Tile::None);
        self.set(next_pos.x as usize, next_pos.y as usize, tile);
        self.set(other_next_pos.x as usize, other_next_pos.y as usize, other_tile);
    }

    #[inline]
    fn sum_box_gps(&self) -> usize {
        let mut total = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get(x, y);
                if tile == Tile::Box || tile == Tile::BoxLeft {
                    total += 100 * y + x;
                }
            }
        }

        total
    }

    #[inline]
    fn make_wide(&self) -> Warehouse {
        let width = self.width * 2;
        let height = self.height;
        let mut grid = Vec::with_capacity(width * height);

        for y in 0..self.height {
            for x in 0..self.width {
                let double_tile = match self.get(x, y) {
                    Tile::BoxLeft => panic!("Tried make a wide warehouse wider"),
                    Tile::BoxRight => panic!("Tried make a wide warehouse wider"),
                    Tile::Box => {
                        grid.push(Tile::BoxLeft);
                        grid.push(Tile::BoxRight);
                        continue;
                    }
                    tile => tile,
                };
                grid.push(double_tile);
                grid.push(double_tile);
            }
        }

        let robot = Vec2::new(self.robot.x * 2, self.robot.y);

        Self { grid, width, height, robot }
    }

    #[inline(always)]
    fn get(&self, x: usize, y: usize) -> Tile {
        return self.grid[y * self.width + x];
    }

    #[inline(always)]
    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.grid[y * self.width + x] = tile;
    }
}

fn solve(input: &String) -> (usize, usize) {
    let start = std::time::Instant::now();

    let (mut warehouse, moves) = parse_input(&input);
    let mut wide_warehouse = warehouse.make_wide();

    //println!("Warehouse:\n{}", &warehouse);
    for m in &moves {
        warehouse.robot_move(*m);
        //println!("Move: {m}");
        //println!("{}", &warehouse);
    }

    //println!("Wide warehouse:\n{}", &wide_warehouse);
    for m in &moves {
        wide_warehouse.robot_move(*m);
        //println!("Move: {m}");
        //println!("{}", &wide_warehouse);
    }

    let gps_sum1 = warehouse.sum_box_gps();
    let gps_sum2 = wide_warehouse.sum_box_gps();

    let time = start.elapsed();
    println!("Time: {:?}", &time);

    (gps_sum1, gps_sum2)
}

fn parse_input(input: &String) -> (Warehouse, Vec<Direction>) {
    let (warehouse, moves) = input.split_once("\n\n").expect("invalid input");
    let warehouse = Warehouse::from_str(&warehouse).unwrap();
    let moves = moves
        .split('\n')
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            x => panic!("invalid input: invalid character in moves: '{x}'"),
        })
        .collect::<Vec<_>>();

    (warehouse, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<"
            .to_string();

        assert_eq!(solve(&input).0, 2028);
    }

    #[test]
    fn example2() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"
            .to_string();

        assert_eq!(solve(&input), (10092, 9021));
    }
}
