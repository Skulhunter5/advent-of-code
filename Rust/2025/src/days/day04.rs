use crate::{Day, Example, Solutions};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Grid<'a> {
    width: usize,
    height: usize,
    cells: &'a [u8],
}

impl<'a> From<&'a str> for Grid<'a> {
    fn from(value: &'a str) -> Self {
        let cells = value.as_bytes();
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
}

impl Grid<'_> {
    fn get(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            panic!("index {x}x{y} out of bounds");
        }

        self.cells[self.index(x, y)] == b'@'
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
        let input = input.trim_end();
        let grid = Grid::from(input);

        let mut accessible_rolls_count = 0;

        for x in 0..grid.width {
            for y in 0..grid.height {
                if grid.get(x, y) {
                    let neighbors = grid.count_neighbors(x, y);
                    if neighbors < 4 {
                        accessible_rolls_count += 1;
                    }
                }
            }
        }

        return Solutions::first(accessible_rolls_count.to_string());
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

        Some(vec![Example::new(input, Solutions::first("13"))])
    }
}
