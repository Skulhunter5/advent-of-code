fn main() {
    let input = std::fs::read_to_string("../inputs/11-input").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}

const CACHE_MAX_VALUE: u64 = 1000;

#[derive(Debug)]
struct Cache {
    max_value: u64,
    total_blinks: usize,
    data: Box<[Option<u64>]>,
}

impl Cache {
    #[inline(always)]
    fn new(max_value: u64, total_blinks: usize) -> Self {
        Self {
            max_value,
            total_blinks,
            data: vec![None; <u64 as TryInto<usize>>::try_into(max_value).unwrap() * total_blinks]
                .into_boxed_slice(),
        }
    }

    #[inline(always)]
    fn get(&self, value: u64, remaining_blinks: usize) -> Option<u64> {
        if value >= self.max_value || remaining_blinks > self.total_blinks || remaining_blinks == 0
        {
            return None;
        }

        return self.data[self.index(value, remaining_blinks)];
    }

    #[inline(always)]
    fn set(&mut self, value: u64, remaining_blinks: usize, result: u64) {
        if value >= self.max_value || remaining_blinks > self.total_blinks || remaining_blinks == 0
        {
            return;
        }

        self.data[self.index(value, remaining_blinks)] = Some(result);
    }

    #[inline(always)]
    fn index(&self, value: u64, remaining_blinks: usize) -> usize {
        value as usize + (remaining_blinks - 1) * self.max_value as usize
    }
}

fn blink(stones: &Vec<u64>, total_blinks: usize) -> u64 {
    let mut total = 0;
    let mut cache = Cache::new(CACHE_MAX_VALUE, total_blinks);
    for stone in stones {
        total += blink_recursive(*stone, total_blinks, &mut cache);
    }
    total
}

fn blink_recursive(stone: u64, remaining_blinks: usize, cache: &mut Cache) -> u64 {
    if remaining_blinks == 0 {
        return 1;
    }

    if let Some(res) = cache.get(stone, remaining_blinks) {
        return res;
    }

    let (a, b) = process_stone(stone);

    let mut result = 0;
    result += blink_recursive(a, remaining_blinks - 1, cache);

    if let Some(b) = b {
        result += blink_recursive(b, remaining_blinks - 1, cache);
    }

    cache.set(stone, remaining_blinks, result);

    result
}

#[inline(always)]
fn process_stone(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        return (1, None);
    }

    let mut power = 10;
    let mut j = 1;
    while power <= stone {
        power *= 10;
        j += 1;
    }
    if j & 1 == 0 {
        j /= 2;
        while j > 0 {
            power /= 10;
            j -= 1;
        }
        let left = stone / power;
        let right = stone - left * power;
        return (left, Some(right));
    }

    let new_stone = stone
        .checked_mul(2024)
        .expect("internal error: stone number type too small");
    return (new_stone, None);
}

fn solve(input: &String) -> (u64, u64) {
    let start = std::time::Instant::now();

    let stones = parse_line(&input);
    let len25 = blink(&stones, 25);
    let len75 = blink(&stones, 75);

    let time = start.elapsed();
    println!("Time: {:?}", &time);

    (len25, len75)
}

fn parse_line(input: &String) -> Vec<u64> {
    input
        .trim()
        .split(' ')
        .map(|word| word.parse::<u64>().expect("invalid input: not a number"))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "125 17".to_string();

        assert_eq!(solve(&input).0, 55312);
    }
}
