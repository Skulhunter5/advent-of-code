use std::collections::BinaryHeap;

fn main() {
    let input = std::fs::read_to_string("../inputs/16-input").unwrap();

    let start = std::time::Instant::now();
    let (part1, part2) = solve(&input);
    let time = start.elapsed();

    println!("Time: {:?}", &time);
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Facing {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Facing {
    const ROTATION_CW: [Facing; 4] = [Facing::East, Facing::South, Facing::West, Facing::North];
    const ROTATION_CCW: [Facing; 4] = [Facing::West, Facing::North, Facing::East, Facing::South];
    const ROTATION_180: [Facing; 4] = [Facing::South, Facing::West, Facing::North, Facing::East];

    #[inline(always)]
    fn rotate_cw(&self) -> Self {
        Self::ROTATION_CW[*self as usize]
    }

    #[inline(always)]
    fn rotate_ccw(&self) -> Self {
        Self::ROTATION_CCW[*self as usize]
    }

    #[inline(always)]
    fn rotate_180(&self) -> Self {
        Self::ROTATION_180[*self as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Path,
    Wall,
    Visited {
        directions: [bool; 4],
        scores: [usize; 4],
    },
}

impl Tile {
    #[inline(always)]
    fn new_visited(facing: Facing, score: usize) -> Self {
        let mut directions = [false; 4];
        directions[facing as usize] = true;
        let mut scores = [0usize; 4];
        scores[facing as usize] = score;
        Self::Visited { directions, scores }
    }

    #[inline]
    fn try_visit(&mut self, facing: Facing, score: usize) -> bool {
        match self {
            Self::Path => {
                *self = Self::new_visited(facing, score);
                return true;
            }
            Self::Wall => return false,
            Self::Visited { ref mut directions, ref mut scores } => {
                if directions[facing as usize] && scores[facing as usize] <= score {
                    return false;
                } else {
                    directions[facing as usize] = true;
                    scores[facing as usize] = score;
                    return true;
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    #[inline(always)]
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    #[inline]
    fn offset(&self, facing: Facing) -> Self {
        match facing {
            Facing::North => Self::new(self.x, self.y - 1),
            Facing::East => Self::new(self.x + 1, self.y),
            Facing::South => Self::new(self.x, self.y + 1),
            Facing::West => Self::new(self.x - 1, self.y),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct SearchPoint {
    pos: Position,
    facing: Facing,
    score: usize,
}

impl SearchPoint {
    #[inline(always)]
    fn new(pos: Position, facing: Facing) -> Self {
        Self {
            pos,
            facing,
            score: 0,
        }
    }

    #[inline(always)]
    fn new_with_score(pos: Position, facing: Facing, score: usize) -> Self {
        Self { pos, facing, score }
    }
}

impl PartialOrd for SearchPoint {
    #[inline(always)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SearchPoint {
    #[inline(always)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // inverted ordering so the binary-heap becomes a min-heap instead of a max-heap
        other.score.cmp(&self.score)
    }
}

#[derive(Debug)]
struct Maze {
    map: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Maze {
    const SCORE_MOVE: usize = 1;
    const SCORE_ROTATE: usize = 1000;

    fn shortest_path(mut self, start: (Position, Facing), end: Position) -> Option<usize> {
        // cannot start on a wall
        if self.get(start.0) == Tile::Wall {
            return None;
        }
        // cannot end on a wall
        if self.get(end) == Tile::Wall {
            return None;
        }

        // Min-Heap because of inverted Ord implementation
        let mut queue = BinaryHeap::new();
        self.set(start.0, Tile::new_visited(start.1, 0));
        queue.push(SearchPoint::new(start.0, start.1));

        while let Some(SearchPoint { pos, facing, score }) = queue.pop() {
            if pos == end {
                return Some(score);
            }

            let rotations = [
                (facing, 0),
                (facing.rotate_cw(), Self::SCORE_ROTATE),
                (facing.rotate_ccw(), Self::SCORE_ROTATE),
                (facing.rotate_180(), Self::SCORE_ROTATE * 2),
            ];
            for (facing, rotation_score) in rotations {
                let next_pos = pos.offset(facing);
                let next_tile = self.get_mut(next_pos);
                let new_score = score + rotation_score + Self::SCORE_MOVE;
                if next_tile.try_visit(facing, new_score) {
                    queue.push(SearchPoint::new_with_score(
                        next_pos,
                        facing,
                        new_score,
                    ));
                }
            }
        }

        None
    }

    #[inline(always)]
    fn get(&self, pos: Position) -> Tile {
        self.map[self.width * pos.y + pos.x]
    }

    #[inline(always)]
    fn get_mut(&mut self, pos: Position) -> &mut Tile {
        &mut self.map[self.width * pos.y + pos.x]
    }

    #[inline(always)]
    fn set(&mut self, pos: Position, tile: Tile) {
        self.map[self.width * pos.y + pos.x] = tile;
    }
}

impl Maze {
    #[inline(always)]
    fn new(map: Vec<Tile>, width: usize, height: usize) -> Self {
        Self { map, width, height }
    }
}

fn solve(input: &String) -> (usize, usize) {
    let (maze, start, end) = parse_input(&input);
    let score = maze.shortest_path(start, end).expect("no path found");

    (score, 0)
}

fn parse_input(input: &String) -> (Maze, (Position, Facing), Position) {
    let mut start = None;
    let mut end = None;
    let mut map = Vec::new();
    let width = input.find("\n").unwrap_or(input.len());
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        if line.len() != width {
            panic!("invalid input: maze is not rectangular");
        }

        height += 1;
        for (x, c) in line.chars().enumerate() {
            map.push(match c {
                '#' => Tile::Wall,
                '.' => Tile::Path,
                'S' => {
                    if start.is_some() {
                        panic!("invalid input: multiple starting positions");
                    }
                    start = Some((Position::new(x, y), Facing::East));
                    Tile::Path
                }
                'E' => {
                    if end.is_some() {
                        panic!("invalid input: multiple ending positions");
                    }
                    end = Some(Position::new(x, y));
                    Tile::Path
                }
                _ => panic!("invalid input: invalid character: {c}"),
            });
        }
    }

    let start = start.expect("invalid input: no starting position");
    let end = end.expect("invalid input: no ending position");

    (Maze::new(map, width, height), start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
            .to_string();

        assert_eq!(solve(&input).0, 7036);
    }

    #[test]
    fn example2() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
            .to_string();

        assert_eq!(solve(&input).0, 11048);
    }
}
