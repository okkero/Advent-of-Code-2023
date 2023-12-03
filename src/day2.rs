use std::{io::BufRead, str::FromStr};

use anyhow::{bail, Context, Error, Result};

use crate::{Input, Solutions};

struct Game {
    id: u32,
    picks: Vec<Pick>,
}

struct Pick {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn is_possible(&self) -> bool {
        const QUERY_RED: u32 = 12;
        const QUERY_GREEN: u32 = 13;
        const QUERY_BLUE: u32 = 14;

        self.picks.iter().all(|pick| {
            pick.red <= QUERY_RED && pick.green <= QUERY_GREEN && pick.blue <= QUERY_BLUE
        })
    }

    fn power(&self) -> u32 {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for pick in &self.picks {
            red = red.max(pick.red);
            green = green.max(pick.green);
            blue = blue.max(pick.blue);
        }

        red * green * blue
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(": ");
        let id_part = split.next().context("Unable to read ID part")?;
        let data_part = split.next().context("Unable to read data part")?;

        let id = id_part[5..].parse().context("Unable to parse ID")?;

        let picks = data_part
            .split("; ")
            .map(|hand_str| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                for cube_str in hand_str.split(", ") {
                    let mut split = cube_str.split(' ');
                    let amount = split
                        .next()
                        .context("Unable to read amount part")?
                        .parse::<u32>()
                        .context("Unable to parse amount")?;
                    let color = split.next().context("Unable to read color part")?;
                    if color == "red" {
                        red += amount;
                    } else if color == "blue" {
                        blue += amount;
                    } else if color == "green" {
                        green += amount;
                    } else {
                        bail!("Invalid color");
                    }
                }

                Ok(Pick { red, green, blue })
            })
            .collect::<Result<Vec<_>>>()
            .context("Unable to parse cubes")?;

        Ok(Game { id, picks })
    }
}

#[derive(Default)]
pub struct Day2;

impl Solutions for Day2 {
    fn part1(&mut self, input: Input) -> Result<String> {
        let solution: u32 = input
            .lines()
            .map(|line| {
                let line = line?;
                let game: Game = line.parse().context("Unable to parse game")?;
                Ok(if game.is_possible() {
                    Some(game.id)
                } else {
                    None
                })
            })
            .filter_map(Result::transpose)
            .sum::<Result<_>>()?;

        Ok(solution.to_string())
    }

    fn part2(&mut self, input: Input) -> Result<String> {
        let solution: u32 = input
            .lines()
            .map(|line| {
                let line = line?;
                let game: Game = line.parse().context("Unable to parse game")?;
                Ok(game.power())
            })
            .sum::<Result<_>>()?;

        Ok(solution.to_string())
    }
}
