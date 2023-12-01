mod day1;
mod daylib;

use std::{env::args, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args();
    if args.len() < 2 {
        return Err("Bad! Must give day number argument!".into());
    }

    let day_number: u8 = args.nth(1).unwrap().parse::<u8>()?;

    let result = match day_number {
        1 => day1::day1(include_str!("inputs/day1.txt")),
        _ => Err(format!("Bad! I don't know how to run day {}!", day_number).into()),
    }?;

    println!("Result for day {}:", day_number);
    println!("{}", result.part1);
    println!("{}", result.part2);

    Ok(())
}
