fn main() {
    let input = std::fs::read_to_string("../inputs/03-input").unwrap();
    
    println!("{}", solve(&input));
}

fn solve(input: &String) -> u32 {
    let grid = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let width = grid[0].len();
    let height = grid.len();

    let mut x = 0usize;
    let mut y = 0;
    let mut numbers = Vec::new();
    let mut start = x;
    while y < height {
        if x >= width {
            if start != x {
                numbers.push((start, x - 1, y));
            }
            x = 0;
            start = 0;
            y += 1;
        } else {
            if !grid[y][x].is_numeric() {
                if start != x {
                    numbers.push((start, x - 1, y));
                }
                start = x + 1;
            }
            x += 1;
        }
    }

    let mut part_numbers = Vec::new();
    'outer:for (start, end, y) in numbers {
        let left = if start > 0 { start - 1 } else { start };
        let right = if end < (width - 1) { end + 1 } else { end };
        if y > 0 {
            for x in left..=right {
                let c = grid[y - 1][x];
                if !c.is_numeric() && c != '.' {
                    let num = grid[y].get(start..=end).unwrap().into_iter().collect::<String>().parse::<u32>().unwrap();
                    part_numbers.push(num);
                    continue 'outer;
                }
            }
        }
        if y < (height - 1) {
            for x in left..=right {
                let c = grid[y + 1][x];
                if !c.is_numeric() && c != '.' {
                    let num = grid[y].get(start..=end).unwrap().into_iter().collect::<String>().parse::<u32>().unwrap();
                    part_numbers.push(num);
                    continue 'outer;
                }
            }
        }
        if left < start {
            let c = grid[y][left];
            if !c.is_numeric() && c != '.' {
                let num = grid[y].get(start..=end).unwrap().into_iter().collect::<String>().parse::<u32>().unwrap();
                part_numbers.push(num);
                continue 'outer;
            }
        }
        if right > end {
            let c = grid[y][right];
            if !c.is_numeric() && c != '.' {
                let num = grid[y].get(start..=end).unwrap().into_iter().collect::<String>().parse::<u32>().unwrap();
                part_numbers.push(num);
                continue 'outer;
            }
        }
    }
    part_numbers.iter().sum()
}
