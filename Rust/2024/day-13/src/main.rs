fn main() {
    let input = std::fs::read_to_string("../inputs/13-input").unwrap();

    let (part1, part2) = solve(&input);
    println!("Part 1: {}", &part1);
    println!("Part 2: {}", &part2);
}

#[derive(Debug)]
struct Offset {
    x: u64,
    y: u64,
}

impl Offset {
    #[inline(always)]
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Position {
    x: u64,
    y: u64,
}

impl Position {
    #[inline(always)]
    fn new(x: u64, y: u64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct ClawMachine {
    a: Offset,
    b: Offset,
    prize: Position,
}

impl ClawMachine {
    const BUTTON_A_COST: u64 = 3;
    const BUTTON_B_COST: u64 = 1;

    #[inline(always)]
    fn new(a: Offset, b: Offset, prize: Position) -> Self {
        Self { a, b, prize }
    }

    fn solve(&self) -> (u64, u64) {
        let part1 = Self::solve_part(&self.a, &self.b, &self.prize).unwrap_or(0);
        let prize2 = Position::new(self.prize.x + 10000000000000, self.prize.y + 10000000000000);
        let part2 = Self::solve_part(&self.a, &self.b, &prize2).unwrap_or(0);

        (part1, part2)
    }

    fn solve_part(a: &Offset, b: &Offset, prize: &Position) -> Option<u64> {
        let num = prize.x as i64 * b.y as i64 - prize.y as i64 * b.x as i64;
        let denom = b.y as i64 * a.x as i64 - b.x as i64 * a.y as i64;

        let na = if num / denom * denom == num {
            num / denom
        } else {
            return None;
        };

        let num = prize.x as i64 - a.x as i64 * na;
        let denom = b.x as i64;

        let nb = if num / denom * denom == num {
            num / denom
        } else {
            return None;
        };

        assert!(na >= 0 && nb >= 0);
        return Some(na as u64 * Self::BUTTON_A_COST + nb as u64 * Self::BUTTON_B_COST);
    }
}

fn solve(input: &String) -> (u64, u64) {
    let start = std::time::Instant::now();

    let machines = input.split("\n\n").map(|chunk| {
        let mut lines = chunk.lines();
        // line 1
        let line = lines.next().expect("invalid input");
        assert!(line.starts_with("Button A: X+"));
        let (x, y) = line["Button A: X+".len()..].split_once(", Y+").expect("invalid input");
        let x = x.parse::<u64>().expect("invalid input");
        let y = y.parse::<u64>().expect("invalid input");
        let a = Offset::new(x, y);
        // line 2
        let line = lines.next().expect("invalid input");
        assert!(line.starts_with("Button B: X+"));
        let (x, y) = line["Button B: X+".len()..].split_once(", Y+").expect("invalid input");
        let x = x.parse::<u64>().expect("invalid input");
        let y = y.parse::<u64>().expect("invalid input");
        let b = Offset::new(x, y);
        // line 3
        let line = lines.next().expect("invalid input");
        assert!(line.starts_with("Prize: X="));
        let (x, y) = line["Prize: X=".len()..].split_once(", Y=").expect("invalid input");
        let x = x.parse::<u64>().expect("invalid input");
        let y = y.parse::<u64>().expect("invalid input");
        let prize = Position::new(x, y);
        ClawMachine::new(a, b, prize)
    }).collect::<Vec<_>>();

    let costs = machines.iter().map(|machine| machine.solve()).fold((0, 0), |acc, costs| (acc.0 + costs.0, acc.1 + costs.1));

    let time = start.elapsed();
    println!("Time: {:?}", &time);

    costs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279".to_string();

        assert_eq!(solve(&input).0, 480);
    }
}
