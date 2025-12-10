use crate::Day;
pub use day01::Day01;
pub use day02::Day02;
pub use day03::Day03;
pub use day04::Day04;

mod day01;
mod day02;
mod day03;
mod day04;

pub fn get_all() -> Vec<Box<dyn Day>> {
    vec![
        Box::new(Day01),
        Box::new(Day02),
        Box::new(Day03),
        Box::new(Day04),
    ]
}

pub fn iter() -> std::vec::IntoIter<Box<dyn Day>> {
    get_all().into_iter()
}
