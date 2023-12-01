pub fn solve(input: &str) -> u32 {
    let mut total: u32 = 0;
    for line in input.lines() {
        let first = line.chars().find(|c| c.is_numeric()).unwrap();
        let last = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        println!("{line}: {first} {last}");
        let calibration_value = format!("{first}{last}").parse::<u32>().unwrap();
        total += calibration_value;
    }
    total
}
