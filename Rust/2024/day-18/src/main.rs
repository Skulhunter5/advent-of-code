use std::collections::VecDeque;

fn main() {
    let input = std::fs::read_to_string("../inputs/18-input").unwrap();

    let (part1, part2) = solve(&input, 71, 1024);
    println!("Part 1: '{}'", &part1);
    println!("Part 2: '{}'", &part2);
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
}

#[derive(Debug)]
struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T>
where
    T: Clone + Default,
{
    #[inline]
    fn new(width: usize, height: usize) -> Self {
        Self {
            data: vec![T::default(); width * height],
            width,
            height,
        }
    }

    #[inline(always)]
    fn get(&self, pos: Position) -> Option<&T> {
        if pos.x >= self.width || pos.y >= self.height {
            return None;
        }
        self.data.get(pos.y * self.width + pos.x)
    }

    #[inline(always)]
    fn set(&mut self, pos: Position, value: T) {
        if pos.x >= self.width || pos.y >= self.height {
            panic!("position ({}, {}) out of bounds for grid of size {}x{}", pos.x, pos.y, self.width, self.height);
        }
        self.data[pos.y * self.width + pos.x] = value;
    }
}

#[derive(Debug)]
struct MemoryMap {
    corrupted: Grid<bool>,
    width: usize,
    height: usize,
}

impl MemoryMap {
    #[inline]
    fn new(width: usize, height: usize) -> Self {
        Self {
            corrupted: Grid::new(width, height),
            width,
            height,
        }
    }

    #[inline]
    fn corrupt(&mut self, pos: Position) {
        self.corrupted.set(pos, true);
    }

    #[inline]
    fn corrupt_all(&mut self, positions: &[Position]) {
        positions.iter().for_each(|pos| self.corrupt(*pos));
    }

    fn shortest_path(&self, start: Position, end: Position) -> Option<usize> {
        if start == end {
            return Some(0);
        }

        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut steps = 0;
        let mut counter = 1;

        let mut visited = Grid::<bool>::new(self.width, self.height);
        visited.set(start, true);

        while let Some(pos) = queue.pop_front() {
            if counter == 0 {
                steps += 1;
                counter = queue.len() + 1;
            }
            counter -= 1;
            //println!("Pos: {}x{} at {} steps (counter={})", pos.x, pos.y, steps, counter);
            //self.print_visited(&visited);

            let neighbors = [
                Position::new(pos.x - 1, pos.y),
                Position::new(pos.x + 1, pos.y),
                Position::new(pos.x, pos.y - 1),
                Position::new(pos.x, pos.y + 1),
            ];
            for neighbor in neighbors {
                if neighbor == end {
                    return Some(steps + 1);
                }
                if let Some(&false) = visited.get(neighbor) {
                    if let Some(&false) = self.corrupted.get(neighbor) {
                        queue.push_back(neighbor);
                        visited.set(neighbor, true);
                    }
                }
            }
        }

        return None;
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let c = match self.corrupted.get(Position::new(x, y)) {
                    None => unreachable!(),
                    Some(true) => '#',
                    Some(false) => '.',
                };
                print!("{c}");
            }
            println!();
        }
    }

    #[allow(dead_code)]
    fn print_visited(&self, visited: &Grid<bool>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position::new(x, y);
                let c = match self.corrupted.get(pos) {
                    None => unreachable!(),
                    Some(true) => '#',
                    Some(false) => match visited.get(pos) {
                        None => unreachable!(),
                        Some(true) => 'O',
                        Some(false) => '.',
                    }
                };
                print!("{c}");
            }
            println!();
        }
    }
}

fn solve(input: &String, size: usize, n_bytes: usize) -> (usize, usize) {
    let start = std::time::Instant::now();

    let bytes = input
        .lines()
        .map(|line| {
            let (left, right) = line
                .split_once(',')
                .expect("invalid input: no comma in line");
            let x = left.parse::<usize>().expect("invalid input: not a number");
            let y = right.parse::<usize>().expect("invalid input: not a number");
            Position::new(x, y)
        })
        .collect::<Vec<_>>();

    let mut mem = MemoryMap::new(size, size);
    //println!("mem before: ");
    //mem.print();
    mem.corrupt_all(&bytes[..n_bytes]);
    //println!();
    //println!("mem after: ");
    //mem.print();

    let min_steps = mem
        .shortest_path(Position::new(0, 0), Position::new(size - 1, size - 1))
        .expect("invalid input: no path to the end");

    let time = start.elapsed();
    println!("Time: {:?}", &time);

    (min_steps, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0"
        .to_string();

        assert_eq!(solve(&input, 7, 12), (22, 0));
    }
}
