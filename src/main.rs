mod day1;
mod day2;

use std::{env, fs::File, io::BufReader};

use anyhow::{anyhow, Context, Result};

use crate::{day1::Day1, day2::Day2};

pub type Input = BufReader<File>;

pub trait Solutions {
    fn part1(&mut self, input: Input) -> Result<String>;
    fn part2(&mut self, input: Input) -> Result<String>;
}

struct Day {
    day: usize,
    solutions: Box<dyn Solutions>,
}

impl Day {
    fn run(&mut self) -> Result<()> {
        println!("====== Day {} ======", self.day);
        println!("Part 1: {}", self.solutions.part1(read_input(self.day)?)?);
        println!("Part 2: {}", self.solutions.part2(read_input(self.day)?)?);

        Ok(())
    }
}

fn day(day: usize) -> Option<Day> {
    fn load<S>() -> Option<Box<dyn Solutions>>
    where
        S: Solutions + Default + 'static,
    {
        Some(Box::<S>::default())
    }

    let solutions = match day {
        1 => load::<Day1>(),
        2 => load::<Day2>(),
        _ => None,
    }?;

    Some(Day { day, solutions })
}

fn main() -> Result<()> {
    let day_number = env::args()
        .nth(1)
        .ok_or(anyhow!(
            "Too few arguments. Please pass a day number as the first argument."
        ))?
        .parse()
        .context("Unable to parse first argument as number. Please pass a day number as the first argument.")?;

    let mut day = day(day_number).ok_or(anyhow!("Day not found"))?;
    day.run()?;

    Ok(())
}

fn read_input(day: usize) -> Result<Input> {
    Ok(BufReader::new(File::open(format!(
        "puzzle-input/day{day}"
    ))?))
}
