use std::{cell::RefCell, io::BufRead, rc::Rc};

use anyhow::Result;
use itertools::Itertools;

use crate::{Input, Solutions};

#[derive(Debug, Default)]
struct Schematic {
    grid: Vec<Tile>,
    numbers: Vec<Rc<RefCell<NumberTile>>>,
    width: usize,
}

impl Schematic {
    fn read(input: Input) -> Result<Self> {
        let mut this = Self::default();

        for line in input.lines() {
            let line = line?;

            let mut parsing_number = None;
            for char in line.chars() {
                let tile = match char {
                    '.' => {
                        parsing_number = None;
                        Tile::Empty
                    }
                    '0'..='9' => {
                        let number_tile = parsing_number.get_or_insert_with(|| {
                            let number_tile = Rc::new(RefCell::new(NumberTile {
                                value: 0,
                                start: this.grid.len(),
                                length: 0,
                            }));
                            this.numbers.push(number_tile.clone());
                            number_tile
                        });
                        let mut tile_data_ref = number_tile.borrow_mut();

                        tile_data_ref.value *= 10;
                        tile_data_ref.value += char as u32 - '0' as u32;
                        tile_data_ref.length += 1;

                        Tile::Number(number_tile.clone())
                    }
                    '*' => {
                        parsing_number = None;
                        Tile::Gear
                    }
                    _ => {
                        parsing_number = None;
                        Tile::Unknown
                    }
                };

                this.grid.push(tile);
                this.width = line.len();
            }
        }

        Ok(this)
    }

    fn height(&self) -> usize {
        self.grid.len() / self.width
    }

    fn index_to_coords(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn coords_to_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn adjacent_indices(&self, index: usize) -> Vec<usize> {
        let (x, y) = self.index_to_coords(index);

        let mut indices = Vec::new();
        for x in x.saturating_sub(1)..=(x + 1).min(self.width - 1) {
            for y in y.saturating_sub(1)..=(y + 1).min(self.height() - 1) {
                indices.push(self.coords_to_index(x, y));
            }
        }

        indices
    }
}

#[derive(Debug)]
enum Tile {
    Empty,
    Number(Rc<RefCell<NumberTile>>),
    Gear,
    Unknown,
}

#[derive(Debug)]
struct NumberTile {
    value: u32,
    start: usize,
    length: usize,
}

impl NumberTile {
    fn adjacent_indices(&self, schematic: &Schematic) -> Vec<usize> {
        let (start_x, start_y) = schematic.index_to_coords(self.start);
        let (end_x, end_y) = schematic.index_to_coords(self.start + self.length - 1);
        assert_eq!(start_y, end_y);

        let mut indices = Vec::new();
        for x in start_x.saturating_sub(1)..=(end_x + 1).min(schematic.width - 1) {
            for y in start_y.saturating_sub(1)..=(end_y + 1).min(schematic.height() - 1) {
                indices.push(schematic.coords_to_index(x, y));
            }
        }

        indices
    }

    fn is_part_number(&self, schematic: &Schematic) -> bool {
        self.adjacent_indices(schematic)
            .into_iter()
            .any(|index| matches!(schematic.grid[index], Tile::Unknown))
    }
}

#[derive(Default)]
pub struct Day3;

impl Solutions for Day3 {
    fn part1(&mut self, input: Input) -> Result<String> {
        let schematic = Schematic::read(input)?;
        let solution: u32 = schematic
            .numbers
            .iter()
            .filter_map(|number_tile| {
                let tile_data_ref = number_tile.borrow();
                if tile_data_ref.is_part_number(&schematic) {
                    Some(tile_data_ref.value)
                } else {
                    None
                }
            })
            .sum();

        Ok(solution.to_string())
    }

    fn part2(&mut self, input: Input) -> Result<String> {
        let schematic = Schematic::read(input)?;
        let solution: u32 = schematic
            .grid
            .iter()
            .enumerate()
            .filter_map(|(index, tile)| {
                if !matches!(tile, Tile::Gear) {
                    return None;
                }

                let adjacent_numbers = schematic
                    .adjacent_indices(index)
                    .into_iter()
                    .map(|adjacent_index| &schematic.grid[adjacent_index])
                    .filter_map(|tile| match tile {
                        Tile::Number(number_tile) => Some(number_tile),
                        _ => None,
                    })
                    .unique_by(|rc| Rc::as_ptr(rc))
                    .map(|number_tile| number_tile.borrow().value)
                    .collect::<Vec<_>>();
                if adjacent_numbers.len() != 2 {
                    return None;
                }

                Some(adjacent_numbers.into_iter().product::<u32>())
            })
            .sum();

        Ok(solution.to_string())
    }
}
