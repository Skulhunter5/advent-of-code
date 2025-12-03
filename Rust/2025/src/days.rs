use crate::Day;

mod day01;

pub fn get_all() -> Vec<Box<dyn Day>>{
    vec![
        Box::new(day01::Day01),
    ]
}
