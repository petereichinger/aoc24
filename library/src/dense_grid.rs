use std::fmt::{Display, Write};

use crate::coordinate::Coord;

pub struct DenseGrid {
    pub data: Vec<Vec<char>>,
}

impl DenseGrid {
    pub fn from_string(string: impl ToString) -> Self {
        let string = string.to_string();

        let data: Vec<Vec<_>> = string.lines().map(|line| line.chars().collect()).collect();
        Self { data }
    }

    pub fn get(&self, coord: &Coord) -> Option<char> {
        self.data
            .get(coord.y)
            .and_then(|row| row.get(coord.x))
            .copied()
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
