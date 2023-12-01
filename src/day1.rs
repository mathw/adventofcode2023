use lazy_static::lazy_static;
use regex::Regex;
use std::error::Error;

use crate::daylib::{DayResult, Parts};

pub fn day1(input: &str) -> DayResult {
    let part1 = part1(input)?;
    let part2 = part2(input)?;

    Ok(Parts {
        part1: Box::new(format!("The sum of the digits is {}", part1)),
        part2: Box::new(format!("The sum of the wordy digits is {}", part2)),
    })
}

fn part1(input: &str) -> Result<u32, Box<dyn Error>> {
    let numbers = input
        .lines()
        .map(parse_line)
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(numbers.into_iter().sum())
}

fn parse_line(line: &str) -> Result<u32, Box<dyn Error>> {
    let mut first_digit: Option<char> = None;
    let mut last_digit: Option<char> = None;

    for c in line.chars() {
        if c.is_ascii_digit() {
            first_digit = Some(c);
            break;
        }
    }

    for c in line.chars().rev() {
        if c.is_ascii_digit() {
            last_digit = Some(c);
            break;
        }
    }

    if let (Some(first), Some(last)) = (first_digit, last_digit) {
        let concatenated = format!("{}{}", first, last);
        let parsed = concatenated.parse::<u32>()?;
        Ok(parsed)
    } else {
        Err(format!("Unable to parse two digits from line '{}'", line).into())
    }
}

fn part2(input: &str) -> Result<u32, Box<dyn Error>> {
    let numbers = input
        .lines()
        .map(parse_line_part2)
        .collect::<Result<Vec<u32>, _>>()?;

    Ok(numbers.into_iter().sum())
}

fn find_first_in_line(line: &str, from: usize) -> Option<(usize, String)> {
    lazy_static! {
        static ref DIGIT: Regex =
            Regex::new(r"(1|2|3|4|5|6|7|8|9|0|one|two|three|four|five|six|seven|eight|nine)")
                .unwrap();
    }

    let captures = DIGIT.captures_at(line, from)?;

    if captures.len() == 0 {
        return None;
    }

    let first = captures.get(1)?;

    Some((first.start(), first.as_str().to_owned()))
}

fn parse_line_part2(line: &str) -> Result<u32, Box<dyn Error>> {
    let (first_index, first_string) = find_first_in_line(line, 0)
        .ok_or_else(|| format!("Unable to match any digits in '{}'", line))?;

    let mut start_index = first_index + 1;
    let mut last_string = first_string.clone();

    while let Some((i, s)) = find_first_in_line(&line, start_index) {
        start_index = i + 1;
        last_string = s;
    }

    let first = match_to_num(&first_string)?;
    let last = match_to_num(&last_string)?;

    let concatenated = format!("{}{}", first, last);
    let parsed = concatenated.parse::<u32>()?;
    Ok(parsed)
}

fn match_to_num(m: &str) -> Result<u32, Box<dyn Error>> {
    match m {
        "1" | "one" => Ok(1),
        "2" | "two" => Ok(2),
        "3" | "three" => Ok(3),
        "4" | "four" => Ok(4),
        "5" | "five" => Ok(5),
        "6" | "six" => Ok(6),
        "7" | "seven" => Ok(7),
        "8" | "eight" => Ok(8),
        "9" | "nine" => Ok(9),
        "0" => Ok(0),
        _ => Err(format!("Unmatched regex capture {}", m).into()),
    }
}

#[cfg(test)]
static TEST_PART1_INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

#[cfg(test)]
static TEST_PART2_INPUT: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

#[test]
fn test_parse_line() {
    let result = parse_line("1abc2").unwrap();
    assert_eq!(result, 12);
    let result = parse_line("treb7uchet").unwrap();
    assert_eq!(result, 77);
}

#[test]
fn test_part1() {
    let result = part1(TEST_PART1_INPUT).unwrap();
    assert_eq!(result, 142);
}

#[test]
fn test_part2() {
    let result = part2(TEST_PART1_INPUT).unwrap();
    assert_eq!(result, 142);
    let result = part2(TEST_PART2_INPUT).unwrap();
    assert_eq!(result, 281);
}

#[test]
fn test_part2_line() {
    let input = "treb7uchet";
    let result = part2(input).unwrap();
    assert_eq!(result, 77);
}
#[test]
fn test_part2_line2() {
    let input = "eight9fhstbssrplmdlncmmqqnklb39ninejz";
    let result = part2(input).unwrap();
    assert_eq!(result, 89);
}
#[test]
fn test_part2_line3() {
    let input = "kdkjqdkvgs2";
    let result = part2(input).unwrap();
    assert_eq!(result, 22);
}
#[test]
fn test_part2_line4() {
    let result = part2("eightwo").unwrap();
    assert_eq!(result, 82);
}
