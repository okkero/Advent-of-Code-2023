use std::{cmp::Ordering, io::BufRead, str::FromStr};

use anyhow::{bail, Context, Error, Result};

use crate::{Input, Solutions};

struct Round {
    hand: Hand,
    bid: u32,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let cards = self.cards.iter().filter(|card| card.0 != 0);

        let mut jokers = 5;
        let mut value_counts = [0u32; 15];
        for card in cards {
            value_counts[card.0 as usize] += 1;
            jokers -= 1;
        }

        if value_counts.contains(&5) {
            HandType::FiveOfAKind
        } else if value_counts.contains(&4) {
            HandType::FourOfAKind
        } else if value_counts.contains(&3) {
            if value_counts.contains(&2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else {
            let pairs = value_counts.iter().filter(|&&n| n == 2).count();
            match pairs {
                2 => HandType::TwoPairs,
                1 => HandType::OnePair,
                _ => HandType::HighCard,
            }
        }
        .joke(jokers)
    }
}

impl FromStr for Round {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let hand = split.next().context("Unable to read hand")?.parse()?;
        let bid = split.next().context("Unable to read bid")?.parse()?;

        Ok(Self { hand, bid })
    }
}

impl FromStr for Hand {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cards = [Card(0); 5];
        for (i, card_result) in s.chars().map(Card::try_from).enumerate() {
            cards[i] = card_result?;
        }

        Ok(Self { cards })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let type_order = self.hand_type().cmp(&other.hand_type());
        if type_order != Ordering::Equal {
            return type_order;
        }

        for (card1, card2) in self.cards.iter().zip(other.cards.iter()) {
            let card_order = card1.cmp(card2);
            if card_order != Ordering::Equal {
                return card_order;
            }
        }

        Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn joke(self, jokers: u32) -> Self {
        match (&self, jokers) {
            (HandType::HighCard, 1) => HandType::OnePair,
            (HandType::HighCard, 2) => HandType::ThreeOfAKind,
            (HandType::HighCard, 3) => HandType::FourOfAKind,
            (HandType::HighCard, 4) => HandType::FiveOfAKind,
            (HandType::HighCard, 5) => HandType::FiveOfAKind,
            (HandType::OnePair, 1) => HandType::ThreeOfAKind,
            (HandType::OnePair, 2) => HandType::FourOfAKind,
            (HandType::OnePair, 3) => HandType::FiveOfAKind,
            (HandType::TwoPairs, 1) => HandType::FullHouse,
            (HandType::ThreeOfAKind, 1) => HandType::FourOfAKind,
            (HandType::ThreeOfAKind, 2) => HandType::FiveOfAKind,
            (HandType::FourOfAKind, 1) => HandType::FiveOfAKind,
            _ => self,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Card(u32);

impl TryFrom<char> for Card {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        let value = match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '2'..='9' => c as u32 - '0' as u32,
            _ => bail!("Invalid card char"),
        };

        Ok(Self(value))
    }
}

#[derive(Default)]
pub struct Day7;

impl Solutions for Day7 {
    fn part1(&mut self, input: Input) -> Result<String> {
        let mut rounds = input
            .lines()
            .map(|line| line?.parse::<Round>())
            .collect::<Result<Vec<_>>>()?;
        rounds.sort_by_key(|round| round.hand);

        let solution: u32 = rounds
            .into_iter()
            .enumerate()
            .map(|(index, round)| (index + 1) as u32 * round.bid)
            .sum();

        Ok(solution.to_string())
    }

    fn part2(&mut self, input: Input) -> Result<String> {
        let mut rounds = input
            .lines()
            .map(|line| {
                let mut round = line?.parse::<Round>()?;
                // Using 0 to mean joker, why not
                round.hand.cards = round
                    .hand
                    .cards
                    .map(|card| if card.0 == 11 { Card(0) } else { card });
                Ok(round)
            })
            .collect::<Result<Vec<_>>>()?;
        rounds.sort_by_key(|round| round.hand);

        let solution: u32 = rounds
            .into_iter()
            .enumerate()
            .map(|(index, round)| (index + 1) as u32 * round.bid)
            .sum();

        Ok(solution.to_string())
    }
}
