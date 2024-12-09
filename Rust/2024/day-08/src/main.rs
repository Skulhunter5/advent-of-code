use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("../inputs/08-input").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    #[inline(always)]
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    #[inline(always)]
    fn within_bounds(&self, width: usize, height: usize) -> bool {
        return self.x < width && self.y < height;
    }
}

#[derive(Debug)]
struct Map {
    width: usize,
    height: usize,
    antennas: Vec<Vec<Position>>,
}

impl Map {
    fn new(width: usize, height: usize, antennas: Vec<Vec<Position>>) -> Self {
        Self {
            width,
            height,
            antennas,
        }
    }

    fn find_antinodes(&self) -> (usize, usize) {
        let mut grid1 = vec![false; self.width * self.height];
        let mut grid2 = vec![false; self.width * self.height];
        let mut count1 = 0;
        let mut count2 = 0;

        for antennas in &self.antennas {
            for i in 0..(antennas.len() - 1) {
                for j in (i + 1)..antennas.len() {
                    let a = &antennas[i];
                    let b = &antennas[j];
                    let dx = b.x - a.x;
                    let dy = b.y - a.y;
                    let antinodes = [
                        Position::new(a.x - dx, a.y - dy),
                        Position::new(b.x + dx, b.y + dy),
                    ];
                    for antinode in antinodes {
                        self.check_antinode(&antinode, &mut grid1, &mut count1);
                        self.check_antinode(&antinode, &mut grid2, &mut count2);
                    }

                    let mut pos = a.clone();
                    loop {
                        self.check_antinode(&pos, &mut grid2, &mut count2);
                        pos.x -= dx;
                        pos.y -= dy;

                        if !pos.within_bounds(self.width, self.height) {
                            break;
                        }
                    }

                    let mut pos = b.clone();
                    loop {
                        self.check_antinode(&pos, &mut grid2, &mut count2);
                        pos.x += dx;
                        pos.y += dy;

                        if !pos.within_bounds(self.width, self.height) {
                            break;
                        }
                    }
                }
            }
        }

        (count1, count2)
    }

    #[inline(always)]
    fn check_antinode(&self, antinode: &Position, grid: &mut Vec<bool>, count: &mut usize) {
        if antinode.within_bounds(self.width, self.height) {
            if !grid[antinode.y * self.height + antinode.x] {
                grid[antinode.y * self.height + antinode.x] = true;
                *count += 1;
            }
        }
    }
}

fn solve(input: &String) -> (usize, usize) {
    let start = std::time::Instant::now();

    let map = parse_input(&input);

    let antinodes = map.find_antinodes();

    let time = start.elapsed();
    println!("Time: {:?}", &time);

    antinodes
}

fn parse_input(input: &String) -> Map {
    let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();

    let width = input.find("\n").expect("invalid input");
    let mut height = 0;
    for (y, line) in input.lines().enumerate() {
        height += 1;
        assert!(line.len() == width);

        for (x, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                let positions = antennas.entry(c).or_insert(Vec::new());
                positions.push(Position::new(x, y));
            }
        }
    }

    Map::new(width, height, antennas.into_values().collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"
            .to_string();

        assert_eq!(solve(&input), (14, 34));
    }
}
