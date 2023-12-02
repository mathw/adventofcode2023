use std::{error::Error, fmt::Display};

pub(crate) type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub(crate) type DayResult = Result<Parts>;

pub(crate) struct Parts {
    pub part1: Box<dyn Display>,
    pub part2: Box<dyn Display>,
}
