use std::io::BufRead;

use anyhow::{Context, Result};

use crate::{Input, Solutions};

const DIGIT_TABLE: [(&str, u32); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn parse_digit(slice: &str) -> Option<u32> {
    DIGIT_TABLE
        .into_iter()
        .find(|(match_string, _)| slice.starts_with(match_string))
        .map(|(_, digit)| digit)
}

fn first_digit(s: &str) -> Option<u32> {
    (0..s.len()).find_map(|start_index| parse_digit(&s[start_index..]))
}

fn last_digit(s: &str) -> Option<u32> {
    (0..s.len())
        .rev()
        .find_map(|start_index| parse_digit(&s[start_index..]))
}

#[derive(Default)]
pub struct Day1;

impl Solutions for Day1 {
    fn part1(&mut self, input: Input) -> Result<String> {
        let solution: u32 = input
            .lines()
            .map(|line| {
                let line = line?;
                let mut digits = line.chars().filter_map(|c| c.to_digit(10));
                let first = digits.next().context("No digits in input")?;
                let last = digits.last().unwrap_or(first);

                Ok(first * 10 + last)
            })
            .sum::<Result<_>>()?;

        Ok(solution.to_string())
    }

    fn part2(&mut self, input: Input) -> Result<String> {
        let solution: u32 = input
            .lines()
            .map(|line| {
                let line = line?;
                let first = first_digit(&line).context("No digits in input")?;
                let last = last_digit(&line).unwrap_or(first);

                Ok(first * 10 + last)
            })
            .sum::<Result<_>>()?;

        Ok(solution.to_string())
    }
}
