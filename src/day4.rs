use std::{collections::HashMap, io::BufRead, str::FromStr};

use anyhow::{Context, Error, Result};
use regex::Regex;

use crate::{Input, Solutions};

#[derive(Debug)]
struct Card {
    id: u32,
    numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let whitespace = Regex::new(" +")?;
        let card_id_divider = Regex::new(" *: *")?;
        let number_list_divider = Regex::new(r" *\| *")?;

        let mut split = card_id_divider.split(s);
        let id_part = split.next().context("Unable to read card ID")?;
        let id = whitespace
            .split(id_part)
            .nth(1)
            .context("Unable to parse card ID")?
            .parse()
            .context("Unable to parse card ID as number")?;
        let data_part = split.next().context("Unable to read card data")?;
        let mut split = number_list_divider.split(data_part);
        let numbers = whitespace
            .split(split.next().context("Unable to read card numbers")?)
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        let winning_numbers = whitespace
            .split(
                split
                    .next()
                    .context("Unable to read card winning numbers")?,
            )
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            id,
            numbers,
            winning_numbers,
        })
    }
}

impl Card {
    fn count_matches(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|number| self.winning_numbers.contains(number))
            .count() as _
    }
}

#[derive(Default)]
pub struct Day4;

impl Solutions for Day4 {
    fn part1(&mut self, input: Input) -> Result<String> {
        let solution: u32 = input
            .lines()
            .map(|line| {
                let card: Card = line?.parse()?;
                let matches = card.count_matches();

                Ok(if matches == 0 { 0 } else { 1 << (matches - 1) })
            })
            .sum::<Result<_>>()?;

        Ok(solution.to_string())
    }

    fn part2(&mut self, input: Input) -> Result<String> {
        let matches_by_card_id = input.lines().map(|line| {
            let card: Card = line?.parse()?;
            let matches = card.count_matches();

            Ok::<_, Error>((card.id, matches))
        });

        let mut copies_by_card_id = HashMap::new();
        for result in matches_by_card_id {
            let (card_id, matches) = result?;
            let copies = *copies_by_card_id.entry(card_id).or_insert(1);
            for i in 1..=matches {
                *copies_by_card_id.entry(card_id + i).or_insert(1) += copies;
            }
        }

        let solution: u32 = copies_by_card_id.into_values().sum();

        Ok(solution.to_string())
    }
}
