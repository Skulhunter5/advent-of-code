use crate::{Day, Example, Solutions};

const BYTE_EMPTY: u8 = b'.';
const BYTE_PAPER_ROLL: u8 = b'@';

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<u8>,
}

impl Grid {
    pub fn new(cells: Vec<u8>) -> Self {
        let width = cells
            .iter()
            .position(|byte| *byte == b'\n')
            .expect("invalid input");
        let height = (cells.len() + 1) / (width + 1);
        Self {
            width,
            height,
            cells,
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            panic!("index {x}x{y} out of bounds");
        }

        self.cells[self.index(x, y)] == BYTE_PAPER_ROLL
    }

    fn remove(&mut self, x: usize, y: usize) {
        if x >= self.width || y >= self.height {
            panic!("index {x}x{y} out of bounds");
        }

        let index = self.index(x, y);
        self.cells[index] = BYTE_EMPTY
    }

    fn index(&self, x: usize, y: usize) -> usize {
        x + y * (self.width + 1)
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut neighbors = 0;

        let min_x = if x > 0 { x - 1 } else { x };
        let max_x = if x < self.width - 1 { x + 1 } else { x };
        let min_y = if y > 0 { y - 1 } else { y };
        let max_y = if y < self.height - 1 { y + 1 } else { y };

        let x0 = x;
        let y0 = y;
        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if x == x0 && y == y0 {
                    continue;
                }
                if self.get(x, y) {
                    neighbors += 1;
                }
            }
        }

        return neighbors;
    }
}

pub struct Day04;

impl Day for Day04 {
    fn solve_both(&self, input: String) -> Solutions {
        let input = input.into_bytes();
        let mut grid = Grid::new(input);

        let mut total_rolls_removed = 0;
        let mut initially_accessible_rolls_count = None;
        let mut accessible = Vec::new();
        loop {
            for x in 0..grid.width {
                for y in 0..grid.height {
                    if grid.get(x, y) {
                        let neighbors = grid.count_neighbors(x, y);
                        if neighbors < 4 {
                            accessible.push((x, y));
                        }
                    }
                }
            }

            if initially_accessible_rolls_count.is_none() {
                initially_accessible_rolls_count = Some(accessible.len());
            }
            total_rolls_removed += accessible.len();
            if accessible.len() == 0 {
                break;
            }

            for (x, y) in &accessible {
                grid.remove(*x, *y);
            }
            accessible.clear();
        }

        return Solutions::both(
            initially_accessible_rolls_count.unwrap().to_string(),
            total_rolls_removed.to_string(),
        );
    }

    fn get_year(&self) -> usize {
        2025
    }

    fn get_day(&self) -> usize {
        4
    }

    fn get_examples(&self) -> Option<Vec<Example>> {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        Some(vec![Example::new(input, Solutions::both("13", "43"))])
    }
}
