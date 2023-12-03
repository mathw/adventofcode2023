use crate::daylib::{DayResult, Parts, Result};

pub fn day3(input: &str) -> DayResult {
    let part1 = part1(input)?;

    Ok(Parts {
        part1: Box::new(format!("The sum of the part numbers is {}", part1)),
        part2: Box::new("Not implemented".to_string()),
    })
}

fn get_part_numbers(input: &str) -> Result<Vec<u32>> {
    fn is_symbol(c: char) -> bool {
        c != '.'
    }
    let mut extended_lines = input
        .lines()
        .map(|line| format!(".{}.", line))
        .collect::<Vec<String>>();

    let line_length = extended_lines[0].len();
    extended_lines.insert(0, ".".repeat(line_length));
    extended_lines.push(".".repeat(line_length));

    enum State {
        LookingForNumber,
        ParsingNumber,
    }

    let mut state = State::LookingForNumber;
    let mut current_start = 0;

    let mut part_numbers = Vec::new();

    for (line_number, line) in extended_lines.iter().enumerate() {
        for (char_number, c) in line.chars().enumerate() {
            match state {
                State::LookingForNumber => {
                    if c.is_ascii_digit() {
                        state = State::ParsingNumber;
                        current_start = char_number;
                    }
                }
                State::ParsingNumber => {
                    if !c.is_ascii_digit() {
                        // got the end of the number
                        state = State::LookingForNumber;

                        let current_number = line[current_start..char_number].parse::<u32>()?;

                        let left = line.chars().nth(current_start - 1).unwrap() != '.';
                        let right = c != '.';
                        let above = extended_lines[line_number - 1]
                            [current_start - 1..=char_number]
                            .chars()
                            .any(is_symbol);
                        let below = extended_lines[line_number + 1]
                            [current_start - 1..=char_number]
                            .chars()
                            .any(is_symbol);

                        if left || right || above || below {
                            part_numbers.push(current_number);
                        }
                    }
                }
            }
        }

        // numbers can't wrap
        state = State::LookingForNumber;
    }

    Ok(part_numbers)
}

#[cfg(test)]
static TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

#[test]
fn test_parse() {
    assert_eq!(
        get_part_numbers(TEST_INPUT).unwrap(),
        vec![467, 35, 633, 617, 592, 755, 664, 598]
    );
}

#[test]
fn test_parse_right_edges() {
    assert_eq!(get_part_numbers("...2").unwrap(), vec![]);
    assert_eq!(get_part_numbers("..*2").unwrap(), vec![2]);
    assert_eq!(get_part_numbers("..%.\n...2").unwrap(), vec![2]);
    assert_eq!(get_part_numbers("...2\n...*").unwrap(), vec![2]);
    assert_eq!(get_part_numbers("...2\n....").unwrap(), vec![]);
}

#[test]
fn test_parse_right() {
    assert_eq!(get_part_numbers(".2.").unwrap(), vec![]);
    assert_eq!(get_part_numbers(".2*").unwrap(), vec![2]);
}

#[test]
fn test_parse_left() {
    assert_eq!(get_part_numbers("*2.").unwrap(), vec![2]);
}

#[test]
fn test_parse_topleft() {
    assert_eq!(get_part_numbers("*..\n.2.").unwrap(), vec![2]);
}

#[test]
fn test_parse_topright() {
    assert_eq!(get_part_numbers("..*\n.2.").unwrap(), vec![2]);
}

#[test]
fn test_parse_botleft() {
    assert_eq!(get_part_numbers(".2.\n*..").unwrap(), vec![2]);
}

#[test]
fn test_parse_botright() {
    assert_eq!(get_part_numbers(".2.\n..*").unwrap(), vec![2]);
}

fn part1(input: &str) -> Result<u32> {
    Ok(get_part_numbers(input)?.into_iter().sum())
}

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), 4361);
}
