use std::{
    collections::{HashMap, HashSet},
    error::Error,
    str::FromStr,
};

use crate::daylib::{self, DayResult, Parts};

pub fn day4(input: &str) -> DayResult {
    let part1 = part1(input)?;
    let part2 = part2(input)?;

    Ok(Parts {
        part1: Box::new(format!("The pile of cards is worth {} points", part1)),
        part2: Box::new(format!("The pile of cards now has {} in it", part2)),
    })
}

struct Card {
    card_number: u8,
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

    fn duplicates(&self) -> impl Iterator<Item = u8> {
        let winning_numbers_i_have_count = self.winning_numbers.intersection(&self.numbers).count();

        self.card_number + 1..=self.card_number + winning_numbers_i_have_count as u8
    }
}

impl FromStr for Card {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon = s.find(':').ok_or("No colon")?;
        let pipe = s.find('|').ok_or("No pipe")?;
        let space = s.find(' ').ok_or("No space")?;
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
        let card_number = u8::from_str(s[space + 1..colon].trim())?;

        Ok(Card {
            card_number,
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

fn part2(input: &str) -> daylib::Result<usize> {
    let cards = input
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<Card>, _>>()?
        .into_iter()
        .map(|card| (card.card_number, card))
        .collect::<HashMap<u8, Card>>();

    let mut unevaluated_cards = cards.keys().cloned().collect::<Vec<u8>>();

    let mut evaluated_cards = Vec::new();

    while let Some(current) = unevaluated_cards.pop() {
        let this_card = &cards[&current];
        let new_cards = this_card.duplicates();
        unevaluated_cards.extend(new_cards);
        evaluated_cards.push(this_card);
    }

    Ok(evaluated_cards.len())
}

#[test]
fn test_card_score() {
    let card = Card {
        card_number: 1,
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
    assert_eq!(card.card_number, 3);
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

#[test]
fn test_duplicates() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    let card = Card::from_str(input).unwrap();
    let dups = card.duplicates().collect::<Vec<u8>>();

    assert_eq!(dups, vec![2, 3, 4, 5])
}

#[test]
fn test_part2() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    let result = part2(input).unwrap();

    assert_eq!(result, 30);
}
