use std::fs;

fn main() {
    let input = fs::read_to_string("../inputs/01-input.txt").unwrap();

    println!("Part 1: {}", part1(&input).unwrap());
    println!("Part 2: {}", part2(&input).unwrap());
}

fn part1(input: &String) -> anyhow::Result<i32> {
    let mut floor: i32 = 0;
    input.chars().try_for_each(|c| {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            '\n' => {},
            _ => return Err(anyhow::anyhow!("Invalid character: '{c}' ({:#X})", c as u8)),
        }
        Ok(())
    })?;

    Ok(floor)
}

fn part2(input: &String) -> anyhow::Result<i32> {
    let mut floor: i32 = 0;
    let mut result = 1;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            '\n' => {},
            _ => return Err(anyhow::anyhow!("Invalid character: '{c}' ({:#X})", c as u8)),
        }
        if floor == -1 {
            break;
        }
        result += 1;
    }

    Ok(result)
}
