use itertools::Itertools;
use std::str::FromStr;

use crate::error::{AocError, Result};

/// A `Location` specifies a pair of [usize], [usize] representing a `row` and
/// `column` respectively. Primarily this is used to interact with [GridLike](super::grid::GridLike)
/// things, and has some additional functionality beyond a generic `Point`,
/// like the ability to iterate over neighboring `Locations`
#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

impl Location {
    const ORTH_LOCS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Given a number of rows, get the representation of this [Location] as
    /// an index into a row-major grid
    pub fn as_rm_index(&self, num_rows: usize) -> usize {
        self.row * num_rows + self.col
    }

    /// Given an index and number of rows, construct a [Location] from an index
    /// into a row-major grid
    pub fn from_rm_index(idx: usize, num_rows: usize) -> Self {
        Location::new(idx / num_rows, idx % num_rows)
    }

    pub fn manhattan_dist(&self, other: &Self) -> usize {
        ((self.row as i64 - other.row as i64).abs() + (self.col as i64 - other.col as i64).abs())
            as usize
    }

    /// Yields an iterator over the eight neighbors of this Location, ignoring
    /// locations where row/col would be negative
    pub fn neighbors(&self) -> impl Iterator<Item = Self> {
        let current_row = self.row as i64;
        let current_col = self.col as i64;
        (-1..=1)
            .cartesian_product(-1..=1)
            .into_iter()
            .filter_map(move |(r, c)| {
                if (r == -1 && current_row == 0)
                    || (c == -1 && current_col == 0)
                    || (r == 0 && c == 0)
                {
                    None
                } else {
                    Some(Self::from((
                        (current_row + r) as usize,
                        (current_col + c) as usize,
                    )))
                }
            })
    }

    /// Yields an iterator over just the north, south, east, and west neighbors
    /// of this location, ignoring locations where row/col would be negative
    pub fn orthogonal_neighbors(&self) -> impl Iterator<Item = Self> {
        let current_row = self.row as i64;
        let current_col = self.col as i64;
        Self::ORTH_LOCS.iter().filter_map(move |(r, c)| {
            if (*r == -1 && current_row == 0) || (*c == -1 && current_col == 0) {
                None
            } else {
                Some(Self::from((
                    (current_row + r) as usize,
                    (current_col + c) as usize,
                )))
            }
        })
    }

    pub fn north(&self) -> Option<Location> {
        if self.row == 0 {
            return None;
        }

        Some((self.row - 1, self.col).into())
    }

    pub fn south(&self) -> Option<Location> {
        Some((self.row + 1, self.col).into())
    }

    pub fn west(&self) -> Option<Location> {
        if self.col == 0 {
            return None;
        }

        Some((self.row, self.col - 1).into())
    }

    pub fn east(&self) -> Option<Location> {
        Some((self.row, self.col + 1).into())
    }
}

impl From<(usize, usize)> for Location {
    fn from(value: (usize, usize)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl FromStr for Location {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(',');
        let row: usize = parts
            .next()
            .ok_or_else(|| AocError::ParseLocationError(format!("missing row: {}", s)))?
            .trim()
            .parse()?;
        let col: usize = parts
            .next()
            .ok_or_else(|| AocError::ParseLocationError(format!("missing col: {}", s)))?
            .trim()
            .parse()?;
        Ok(Self::new(row, col))
    }
}

#[cfg(test)]
mod tests {
    mod location {
        use std::collections::HashSet;
        use std::iter::FromIterator;

        use super::super::*;

        #[test]
        fn as_rm_index() {
            let l = Location::from_rm_index(5, 5);
            assert_eq!(l.as_rm_index(5), 5);

            let l = Location::from_rm_index(44, 5);
            assert_eq!(l.as_rm_index(5), 44);
        }

        #[test]
        fn from_rm_index() {
            let l = Location::from_rm_index(5, 5);
            assert_eq!(l, Location::new(1, 0));

            let l = Location::from_rm_index(6, 5);
            assert_eq!(l, Location::new(1, 1));

            let l = Location::from_rm_index(11, 5);
            assert_eq!(l, Location::new(2, 1));
        }

        #[test]
        fn neighbors() {
            let l = Location::from((1, 1));
            let neighbors: HashSet<Location> = l.neighbors().collect();
            let expected: HashSet<Location> = HashSet::from_iter(
                vec![
                    Location::new(0, 0),
                    Location::new(0, 1),
                    Location::new(0, 2),
                    Location::new(1, 0),
                    Location::new(1, 2),
                    Location::new(2, 0),
                    Location::new(2, 1),
                    Location::new(2, 2),
                ]
                .into_iter(),
            );
            assert_eq!(neighbors.len(), 8);
            assert_eq!(neighbors, expected);

            let l = Location::from((0, 0));
            let neighbors: HashSet<Location> = l.neighbors().collect();
            let expected: HashSet<Location> = HashSet::from_iter(
                vec![
                    Location::new(0, 1),
                    Location::new(1, 1),
                    Location::new(1, 0),
                ]
                .into_iter(),
            );
            assert_eq!(neighbors.len(), 3);
            assert_eq!(neighbors, expected);

            let l = Location::from((0, 1));
            let neighbors: HashSet<Location> = l.neighbors().collect();
            let expected: HashSet<Location> = HashSet::from_iter(
                vec![
                    Location::new(0, 0),
                    Location::new(0, 2),
                    Location::new(1, 0),
                    Location::new(1, 1),
                    Location::new(1, 2),
                ]
                .into_iter(),
            );
            assert_eq!(neighbors.len(), 5);
            assert_eq!(neighbors, expected);
        }

        #[test]
        fn orthogonal_neighbors() {
            let l = Location::from((1, 1));
            let neighbors: HashSet<Location> = l.orthogonal_neighbors().collect();
            let expected: HashSet<Location> = HashSet::from_iter(
                vec![
                    Location::new(0, 1),
                    Location::new(1, 0),
                    Location::new(1, 2),
                    Location::new(2, 1),
                ]
                .into_iter(),
            );
            assert_eq!(neighbors.len(), 4);
            assert_eq!(neighbors, expected);

            let l = Location::from((0, 0));
            let neighbors: HashSet<Location> = l.orthogonal_neighbors().collect();
            let expected: HashSet<Location> =
                HashSet::from_iter(vec![Location::new(0, 1), Location::new(1, 0)].into_iter());
            assert_eq!(neighbors.len(), 2);
            assert_eq!(neighbors, expected);

            let l = Location::from((0, 1));
            let neighbors: HashSet<Location> = l.orthogonal_neighbors().collect();
            let expected: HashSet<Location> = HashSet::from_iter(
                vec![
                    Location::new(0, 0),
                    Location::new(0, 2),
                    Location::new(1, 1),
                ]
                .into_iter(),
            );
            assert_eq!(neighbors.len(), 3);
            assert_eq!(neighbors, expected);
        }

        #[test]
        fn north() {
            let l = Location::new(2, 2);
            assert_eq!(l.north(), Some(Location::new(1, 2)));

            let l = Location::new(0, 2);
            assert_eq!(l.north(), None);
        }

        #[test]
        fn south() {
            let l = Location::new(2, 2);
            assert_eq!(l.south(), Some(Location::new(3, 2)));
        }

        #[test]
        fn east() {
            let l = Location::new(2, 2);
            assert_eq!(l.east(), Some(Location::new(2, 3)));
        }

        #[test]
        fn west() {
            let l = Location::new(2, 2);
            assert_eq!(l.west(), Some(Location::new(2, 1)));

            let l = Location::new(2, 0);
            assert_eq!(l.west(), None);
        }
    }
}
