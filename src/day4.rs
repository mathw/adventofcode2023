use std::{collections::HashSet, error::Error, str::FromStr};

use crate::daylib::{self, DayResult, Parts};

pub fn day4(input: &str) -> DayResult {
    let part1 = part1(input)?;

    Ok(Parts {
        part1: Box::new(format!("The pile of cards is worth {} points", part1)),
        part2: Box::new(format!("no")),
    })
}

struct Card {
    winning_numbers: HashSet<u8>,
    numbers: HashSet<u8>,
}

impl Card {
    fn score(&self) -> u32 {
        let winning_numbers_i_have_count = self.winning_numbers.intersection(&self.numbers).count();
        match winning_numbers_i_have_count {
            0 => 0,
            1 => 1,
            _ => 2_u32.pow(
                (winning_numbers_i_have_count - 1)
                    .try_into()
                    .expect("Winning number count doesn't fit in a u32"),
            ),
        }
    }
}

impl FromStr for Card {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon = s.find(':').ok_or("No colon")?;
        let pipe = s.find('|').ok_or("No pipe")?;
        let winning_numbers_str = s[colon + 1..pipe].trim();
        let have_numbers_str = s[pipe + 1..].trim();
        let winning_numbers = winning_numbers_str
            .split_ascii_whitespace()
            .map(u8::from_str)
            .collect::<Result<HashSet<u8>, _>>()?;
        let numbers = have_numbers_str
            .split_ascii_whitespace()
            .map(u8::from_str)
            .collect::<Result<HashSet<u8>, _>>()?;
        Ok(Card {
            winning_numbers,
            numbers,
        })
    }
}

fn part1(input: &str) -> daylib::Result<u32> {
    Ok(input
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<Card>, _>>()?
        .iter()
        .map(Card::score)
        .sum())
}

#[test]
fn test_card_score() {
    let card = Card {
        winning_numbers: vec![41, 48, 83, 86, 17].into_iter().collect(),
        numbers: vec![83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect(),
    };

    assert_eq!(card.score(), 8);
}

#[test]
fn test_parse_card() {
    let input = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
    let card = Card::from_str(input).unwrap();
    assert_eq!(card.numbers, HashSet::from([69, 82, 63, 72, 16, 21, 14, 1]));
    assert_eq!(card.winning_numbers, HashSet::from([1, 21, 53, 59, 44]));
}

#[test]
fn test_part1() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let result = part1(input).unwrap();

    assert_eq!(result, 13);
}
