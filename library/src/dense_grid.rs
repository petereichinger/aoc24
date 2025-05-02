use std::fmt::{Display, Write};

use crate::coordinate::Coord;

pub struct DenseGrid {
    pub data: Vec<Vec<char>>,
}

impl DenseGrid {
    pub fn from_string(string: impl ToString) -> Self {
        let string = string.to_string();

        if string.is_empty() {
            panic!("no empty strings allowed")
        }

        let data: Vec<Vec<_>> = string.lines().map(|line| line.chars().collect()).collect();
        Self { data }
    }

    pub fn get(&self, coord: &Coord) -> Option<char> {
        if coord.x < 0 || coord.y < 0 {
            return None;
        }
        let x = coord.x as usize;
        let y = coord.y as usize;
        self.data.get(y).and_then(|row| row.get(x)).copied()
    }

    pub fn height(&self) -> usize {
        self.data.len()
    }

    pub fn width(&self) -> usize {
        self.data.get(0).unwrap().len()
    }
}

impl Display for DenseGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.data.iter().try_for_each(|row| {
            let result = row.iter().try_for_each(|col| f.write_char(*col));
            result.and_then(|_| f.write_char('\n'))
        })
    }
}
