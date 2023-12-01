pub mod part1;
pub mod part2;

pub fn load_input() -> String {
    std::fs::read_to_string("../inputs/01-input").unwrap()
}
