use std::{error::Error, fmt::Display};

pub(crate) type DayResult = Result<Parts, Box<dyn Error>>;

pub(crate) struct Parts {
    pub part1: Box<dyn Display>,
    pub part2: Box<dyn Display>,
}
