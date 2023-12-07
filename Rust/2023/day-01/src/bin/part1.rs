fn main() {
    let input = std::fs::read_to_string("../inputs/01-input").unwrap();
    println!("{}", solve(&input));
}

pub fn solve(input: &str) -> String {
    let mut total: u32 = 0;
    for line in input.lines() {
        let first = line.chars().find(|c| c.is_numeric()).unwrap();
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        let calibration_value = format!("{first}{last}").parse::<u32>().unwrap();
        total += calibration_value;
    }
    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet".to_string();
        assert_eq!(solve(&input), "142");
    }
}
