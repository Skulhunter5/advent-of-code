use crate::Day;
pub use day01::Day01;
pub use day02::Day02;

mod day01;
mod day02;

pub fn get_all() -> Vec<Box<dyn Day>> {
    vec![Box::new(Day01), Box::new(Day02)]
}

pub fn iter() -> std::vec::IntoIter<Box<dyn Day>> {
    get_all().into_iter()
}
