use std::{collections::VecDeque, str::FromStr};

use bit_set::BitSet;

fn main() {
    let input = std::fs::read_to_string("../inputs/12-input").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}

#[derive(Debug)]
struct Region {
    grid: BitSet,
    width: usize,
    height: usize,
    area: usize,
    perimeter: usize,
}

impl Region {
    #[inline(always)]
    fn new(width: usize, height: usize) -> Self {
        let grid = BitSet::with_capacity(width * height);
        Self {
            grid,
            width,
            height,
            area: 0,
            perimeter: 0,
        }
    }

    #[inline]
    fn add_plot(&mut self, x: usize, y: usize, indices: &[(bool, usize, usize, usize); 4]) {
        let index = self.make_index(x, y);
        assert!(!self.grid.contains(index));

        self.grid.insert(index);
        self.area += 1;

        self.perimeter += 4;
        for (in_bounds, index, _, _) in indices {
            if !in_bounds {
                continue;
            }
            if self.grid.contains(*index) {
                self.perimeter -= 2;
            }
        }
    }

    fn count_sides(&self) -> usize {
        let mut count = 0;

        // check outer sides left & right
        let mut y = 0;
        // dynamic indices so we don't have to do the multiplication every iteration but instead
        // only an addition
        let mut index_left = y * self.width + 0; // x = 0
        let mut index_right = y * self.width + self.width - 1; // x = self.width - 1
        let mut is_side_left = false;
        let mut is_side_right = false;
        while y < self.height {
            if self.grid.contains(index_left) {
                if !is_side_left {
                    is_side_left = true;
                    count += 1;
                }
            } else {
                is_side_left = false;
            }

            if self.grid.contains(index_right) {
                if !is_side_right {
                    is_side_right = true;
                    count += 1;
                }
            } else {
                is_side_right = false;
            }

            y += 1;
            index_left += self.width;
            index_right += self.width;
        }

        // check inner sides left & right
        for x in 1..self.width {
            let mut y = 0;
            let mut index = y * self.width + x;
            let mut is_side_left = false;
            let mut is_side_right = false;
            while y < self.height {
                // check if (x, y) is a cell but (x-1, y) isn't
                // i.e. is there a side left of (x, y)
                if self.grid.contains(index) && !self.grid.contains(index - 1) {
                    if !is_side_left {
                        is_side_left = true;
                        count += 1;
                    }
                } else {
                    is_side_left = false;
                }

                // check if (x-1, y) is a cell but (x, y) isn't
                // i.e. is there a side right of (x-1, y)
                if self.grid.contains(index - 1) && !self.grid.contains(index) {
                    if !is_side_right {
                        is_side_right = true;
                        count += 1;
                    }
                } else {
                    is_side_right = false;
                }

                y += 1;
                index += self.width;
            }
        }

        // check outer sides top & bottom
        let mut x = 0;
        let mut index_top = 0 * self.width + x; // y = 0
        let mut index_bottom = (self.height - 1) * self.width + x; // y = self.height - 1
        let mut is_side_top = false;
        let mut is_side_bottom = false;
        while x < self.width {
            if self.grid.contains(index_top) {
                if !is_side_top {
                    is_side_top = true;
                    count += 1;
                }
            } else {
                is_side_top = false;
            }

            if self.grid.contains(index_bottom) {
                if !is_side_bottom {
                    is_side_bottom = true;
                    count += 1;
                }
            } else {
                is_side_bottom = false;
            }

            x += 1;
            index_top += 1;
            index_bottom += 1;
        }

        // check inner sides top & bottom
        for y in 1..self.height {
            let mut x = 0;
            let mut index = y * self.width + x;
            let mut is_side_top = false;
            let mut is_side_bottom = false;
            while x < self.width {
                // check if (x, y) is a cell but (x, y-1) isn't
                // i.e. is there a side above (x, y)
                if self.grid.contains(index) && !self.grid.contains(index - self.width) {
                    if !is_side_top {
                        is_side_top = true;
                        count += 1;
                    }
                } else {
                    is_side_top = false;
                }

                // check if (x, y-1) is a cell but (x, y) isn't
                // i.e. is there a side below (x, y-1)
                if self.grid.contains(index - self.width) && !self.grid.contains(index) {
                    if !is_side_bottom {
                        is_side_bottom = true;
                        count += 1;
                    }
                } else {
                    is_side_bottom = false;
                }

                x += 1;
                index += 1;
            }
        }

        count
    }

    #[inline(always)]
    fn make_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

#[derive(Debug)]
struct GardenMap {
    regions: Vec<Region>,
}

#[derive(Debug)]
enum ParseGardenMapError {
    InvalidShape,
}

impl FromStr for GardenMap {
    type Err = ParseGardenMapError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let width = input.find('\n').unwrap_or(input.len());
        let mut height = 0;
        let mut grid = Vec::new();

        for line in input.lines() {
            if line.len() != width {
                return Err(ParseGardenMapError::InvalidShape);
            }

            height += 1;
            for c in line.chars() {
                grid.push(Some(c));
            }
        }

        let mut regions = Vec::new();
        for x in 0..width {
            for y in 0..height {
                let index = y * width + x;
                if let Some(plant) = grid[index] {
                    let mut region = Region::new(width, height);
                    let mut queue = VecDeque::new();
                    grid[y * width + x] = None;
                    queue.push_back((x, y));

                    while let Some((x, y)) = queue.pop_front() {
                        let mut indices = [
                            (false, y * width + (x - 1), -1isize as usize, 0),
                            (false, y * width + (x + 1), 1, 0),
                            (false, (y - 1) * width + x, 0, -1isize as usize),
                            (false, (y + 1) * width + x, 0, 1),
                        ];
                        if x > 0 {
                            indices[0].0 = true;
                        }
                        if x < width - 1 {
                            indices[1].0 = true;
                        }
                        if y > 0 {
                            indices[2].0 = true;
                        }
                        if y < height - 1 {
                            indices[3].0 = true;
                        }
                        region.add_plot(x, y, &indices);

                        for (in_bounds, index, x_off, y_off) in indices {
                            if !in_bounds {
                                continue;
                            }
                            if grid[index] == Some(plant) {
                                grid[index] = None;
                                queue.push_back((x + x_off, y + y_off));
                            }
                        }
                    }
                    regions.push(region);
                }
            }
        }

        Ok(Self { regions })
    }
}

impl GardenMap {
    #[inline]
    fn fence_costs(&self) -> (usize, usize) {
        let part1 = self
            .regions
            .iter()
            .map(|region| region.area * region.perimeter)
            .sum::<usize>();
        let part2 = self
            .regions
            .iter()
            .map(|region| region.area * region.count_sides())
            .sum::<usize>();

        (part1, part2)
    }
}

fn solve(input: &String) -> (usize, usize) {
    let start = std::time::Instant::now();

    let map = GardenMap::from_str(input).unwrap();
    let fence_costs = map.fence_costs();

    let time = start.elapsed();
    println!("Time: {:?}", &time);

    fence_costs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "AAAA
BBCD
BBCC
EEEC"
            .to_string();

        assert_eq!(solve(&input), (140, 80));
    }

    #[test]
    fn example2() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE"
            .to_string();

        assert_eq!(solve(&input), (1930, 1206));
    }

    #[test]
    fn example3() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE"
            .to_string();

        // this example was only shown on part two, therefore there are no official values for part
        // one of this example
        assert_eq!(solve(&input).1, 236);
    }

    #[test]
    fn example4() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA"
            .to_string();

        // this example was only shown on part two, therefore there are no official values for part
        // one of this example
        assert_eq!(solve(&input).1, 368);
    }
}
