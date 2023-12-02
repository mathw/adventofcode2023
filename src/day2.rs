use crate::daylib::{DayResult, Parts, Result};
use lazy_static::lazy_static;
use regex::{Captures, Regex};

pub fn day2(input: &str) -> DayResult {
    let part1 = part1(input)?;

    Ok(Parts {
        part1: Box::new(format!("Sum of possible game IDs is {}", part1)),
        part2: Box::new(format!("not implemented")),
    })
}

#[derive(PartialEq, Eq, Debug)]
struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

impl Bag {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }

    fn possible_from(&self, other: &Bag) -> bool {
        self.red <= other.red && self.green <= other.green && self.blue <= other.blue
    }
}

struct Game {
    id: u32,
    bags: Vec<Bag>,
}

impl Game {
    fn possible_from(&self, reference: &Bag) -> bool {
        self.bags.iter().all(|b| b.possible_from(reference))
    }
}

fn parse_grab(input: &str) -> Result<Bag> {
    lazy_static! {
        static ref BLUE: Regex = Regex::new(r"(\d+) blue").unwrap();
        static ref RED: Regex = Regex::new(r"(\d+) red").unwrap();
        static ref GREEN: Regex = Regex::new(r"(\d+) green").unwrap();
    }

    let blue = parse_from_capture(BLUE.captures(input))?;
    let red = parse_from_capture(RED.captures(input))?;
    let green = parse_from_capture(GREEN.captures(input))?;

    return Ok(Bag::new(red, green, blue));
}

fn parse_grabs(input: &str) -> Result<Vec<Bag>> {
    input.split(";").map(|i| parse_grab(i)).collect()
}

fn parse_game(input: &str) -> Result<Game> {
    lazy_static! {
        static ref GAME: Regex = Regex::new(r"Game (\d+)").unwrap();
    }

    let id = parse_from_capture(GAME.captures(input))?;
    let bags = parse_grabs(input)?;

    Ok(Game { id, bags })
}

fn parse_from_capture(c: Option<Captures<'_>>) -> Result<u32> {
    let mut result = 0;
    if let Some(c) = c {
        let captured = c.get(1);
        if let Some(captured) = captured {
            result = captured.as_str().parse()?;
        }
    }
    Ok(result)
}

fn parse_input(input: &str) -> Result<Vec<Game>> {
    input.lines().map(parse_game).collect()
}

fn part1(input: &str) -> Result<u32> {
    let reference = Bag::new(12, 13, 14);
    let games = parse_input(input)?;
    let possibles = games.iter().filter(|g| g.possible_from(&reference));
    let result = possibles.map(|g| g.id).sum();
    Ok(result)
}

#[test]
fn test_parse_game() {
    let g = parse_game("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").unwrap();
    assert_eq!(g.id, 1);
    assert_eq!(g.bags.len(), 3);
    assert_eq!(g.bags[0].blue, 3);
    assert_eq!(g.bags[0].green, 0);
    assert_eq!(g.bags[0].red, 4);
    assert_eq!(g.bags[1].blue, 6);
    assert_eq!(g.bags[1].green, 2);
    assert_eq!(g.bags[1].red, 1);
    assert_eq!(g.bags[2].blue, 0);
    assert_eq!(g.bags[2].green, 2);
    assert_eq!(g.bags[2].red, 0);
}

#[cfg(test)]
static TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT).unwrap(), 8);
}
