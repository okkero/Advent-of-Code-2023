use std::{io::BufRead, str::FromStr};

use anyhow::{Context, Error, Result};
use itertools::Itertools;

use crate::{Input, Solutions};

struct Seeds {
    seeds: Vec<u64>,
}

struct Almanac {
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

struct Map {
    ranges: Vec<MapRange>,
}

struct MapRange {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl Map {
    fn map(&self, source: u64) -> u64 {
        self.ranges
            .iter()
            .find(|range| {
                source >= range.source_start && source < range.source_start + range.length
            })
            .map(|range| range.destination_start + source - range.source_start)
            .unwrap_or(source)
    }
}

impl FromStr for Seeds {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Seeds {
            seeds: s[7..]
                .split(' ')
                .map(|s| Ok(s.parse()?))
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

impl Seeds {
    fn expand_ranges(&self) -> impl Iterator<Item = u64> + '_ {
        self.seeds
            .iter()
            .copied()
            .tuples()
            .flat_map(|(start, length)| start..(start + length))
    }
}

impl Almanac {
    fn parse(lines: &mut impl Iterator<Item = Result<String>>) -> Result<Self> {
        let seed_to_soil = Self::parse_map(lines).context("Unable to parse seed to soil map")?;
        lines.next();
        let soil_to_fertilizer =
            Self::parse_map(lines).context("Unable to parse soil to fertilizer map")?;
        lines.next();
        let fertilizer_to_water =
            Self::parse_map(lines).context("Unable to parse fertilizer to water map")?;
        lines.next();
        let water_to_light =
            Self::parse_map(lines).context("Unable to parse water to light map")?;
        lines.next();
        let light_to_temperature =
            Self::parse_map(lines).context("Unable to parse light to temperature map")?;
        lines.next();
        let temperature_to_humidity =
            Self::parse_map(lines).context("Unable to parse temperature to humidity map")?;
        lines.next();
        let humidity_to_location =
            Self::parse_map(lines).context("Unable to parse humidity to location map")?;

        Ok(Self {
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }

    fn parse_map(lines: &mut impl Iterator<Item = Result<String>>) -> Result<Map> {
        lines.next();
        let ranges = lines
            .take_while(|line| !line.as_ref().map(String::is_empty).unwrap_or_default())
            .map(|line| {
                let line = line?;
                let mut split = line.split(' ');
                let destination_start = split
                    .next()
                    .context("Unable to read destination start")?
                    .parse()?;
                let source_start = split
                    .next()
                    .context("Unable to read source start")?
                    .parse()?;
                let length = split.next().context("Unable to read length")?.parse()?;

                Ok(MapRange {
                    destination_start,
                    source_start,
                    length,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Map { ranges })
    }

    fn location_from_seed(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.map(seed);
        let fertilizer = self.soil_to_fertilizer.map(soil);
        let water = self.fertilizer_to_water.map(fertilizer);
        let light = self.water_to_light.map(water);
        let temperature = self.light_to_temperature.map(light);
        let humidity = self.temperature_to_humidity.map(temperature);

        self.humidity_to_location.map(humidity)
    }
}

#[derive(Default)]
pub struct Day5;

impl Solutions for Day5 {
    fn part1(&mut self, input: Input) -> Result<String> {
        // Part 1 is incorrect, I think. Let me explain:
        // I had completed part 1 and submitted the correct answer, and thought I was done with it.
        // Later, I was struggling to get part 2 to give me the correct result. Upon rereading the
        // prompt for part 1 it turns out I had missed a crucial piece of information: "Any source
        // numbers that aren't mapped correspond to the same destination number." Upon fixing this,
        // I got the correct answer for part 2, but curiously part 1 stopped giving me the correct
        // answer, and I find that odd. It might just be some other piece of information I have
        // missed or misinterpreted, or some off by one error, but at this point I'm not going to
        // spend any more energy investigating it. Day 5 is done.

        let (seeds, almanac) = Self::parse_input(input)?;
        let solution =
            Self::find_lowest_location(seeds.seeds, &almanac).context("No location found")?;

        Ok(solution.to_string())
    }

    fn part2(&mut self, input: Input) -> Result<String> {
        let (seeds, almanac) = Self::parse_input(input)?;
        let solution = Self::find_lowest_location(seeds.expand_ranges(), &almanac)
            .context("No location found")?;

        Ok(solution.to_string())
    }
}

impl Day5 {
    fn parse_input(input: Input) -> Result<(Seeds, Almanac)> {
        let mut lines = input.lines().map(|line| Ok(line?));
        let seeds: Seeds = lines
            .next()
            .context("Unable to read seeds line")??
            .parse()?;
        lines.next();
        let almanac = Almanac::parse(&mut lines)?;

        Ok((seeds, almanac))
    }

    fn find_lowest_location(
        seeds: impl IntoIterator<Item = u64>,
        almanac: &Almanac,
    ) -> Option<u64> {
        seeds
            .into_iter()
            .map(|seed| almanac.location_from_seed(seed))
            .min()
    }
}
