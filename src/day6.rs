use std::io::BufRead;

use anyhow::{Context, Error, Result};

use crate::{Input, Solutions};

#[derive(Debug)]
struct Race {
    time_ms: u64,
    record_distance_mm: u64,
}

fn parse_races(input: Input) -> Result<Vec<Race>> {
    let mut lines = input.lines();
    let times_line = lines.next().context("Unable to read times")??;
    let distances_line = lines.next().context("Unable to read distances")??;

    let times = words(&times_line)
        .skip(1)
        .map(|s| Ok::<_, Error>(s.parse()?));
    let distances = words(&distances_line)
        .skip(1)
        .map(|s| Ok::<_, Error>(s.parse()?));

    times
        .zip(distances)
        .map(|(time_ms, record_distance_mm)| {
            Ok(Race {
                time_ms: time_ms?,
                record_distance_mm: record_distance_mm?,
            })
        })
        .collect()
}

fn parse_the_one_true_race(input: Input) -> Result<Race> {
    let mut lines = input.lines();
    let time_line = lines.next().context("Unable to read times")??;
    let distance_line = lines.next().context("Unable to read distances")??;

    let time: u64 = words(&time_line).skip(1).collect::<String>().parse()?;
    let record_distance = words(&distance_line).skip(1).collect::<String>().parse()?;

    Ok(Race {
        time_ms: time,
        record_distance_mm: record_distance,
    })
}

/// Get the lower and upper bounds of how long in ms you need to hold the button in order to reach
/// a distance of at least `min_distance_mm` in a race that lasts `race_time_ms`.
///
/// The function of (race_time, hold_time) to distance is a quadratic polynomial, so this
/// implementation uses the quadratic formula to find the hold times given a distance.
fn hold_time_ms_bounds(race_time_ms: u64, min_distance_mm: u64) -> (u64, u64) {
    let sqrt_part = ((race_time_ms * race_time_ms - 4 * min_distance_mm) as f64).sqrt();

    (
        ((race_time_ms as f64 - sqrt_part) / 2.0).ceil() as _,
        ((race_time_ms as f64 + sqrt_part) / 2.0) as _,
    )
}

fn words(s: &str) -> impl Iterator<Item = &str> {
    s.split(' ').filter(|s| !s.is_empty())
}

#[derive(Default)]
pub struct Day6;

impl Solutions for Day6 {
    fn part1(&mut self, input: Input) -> Result<String> {
        let races = parse_races(input)?;
        let solution: u64 = races
            .into_iter()
            .map(|race| hold_time_ms_bounds(race.time_ms, race.record_distance_mm + 1))
            .map(|(lower, upper)| upper - lower + 1)
            .product();

        Ok(solution.to_string())
    }

    fn part2(&mut self, input: Input) -> Result<String> {
        let race = parse_the_one_true_race(input)?;
        let (lower, upper) = hold_time_ms_bounds(race.time_ms, race.record_distance_mm + 1);
        let solution = upper - lower + 1;

        Ok(solution.to_string())
    }
}
