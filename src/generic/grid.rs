use itertools::Itertools;

use crate::error::{AocError, Result};
use std::{convert::TryFrom, fmt::{self, Debug, Display}};

use super::Location;

/// The `GridLike` trait denotes that the implementor can be indexed via
/// [Location] like it was a grid of `rows * cols`.
pub trait GridLike {
    type Item;

    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn get(&self, location: &Location) -> Option<&Self::Item>;
    fn get_mut(&mut self, location: &Location) -> Option<&mut Self::Item>;

    /// Sets the value for the given [Location], if possible. Returns `true` if
    /// the set happened.
    fn set(&mut self, location: &Location, value: Self::Item) -> bool {
        self.get_mut(location).map(|v| *v = value).is_some()
    }

    /// Return a [Location] corresponding to the top left corner of this grid
    /// (the "first" element in the grid). This will always be
    /// `Location::default()`
    fn top_left(&self) -> Location {
        Location::default()
    }

    /// Return a [Location] corresponding to the bottom right corner of this grid
    /// (the "last" element in the grid).
    fn bottom_right(&self) -> Location {
        Location::new(self.rows() - 1, self.cols() - 1)
    }

    /// Return the total number of cells contained in this grid-like thing.
    fn size(&self) -> usize {
        self.rows() * self.cols()
    }

    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

/// The `Scalable` trait allows for a GridLike object to be scaled (tiled) in
/// both width and height. It was useful for problem 15 of the 2021 AoC.
pub trait Scalable: GridLike {
    fn scaled_bottom_right(&self, scale: usize) -> Location {
        Location::new(self.rows() * scale - 1, self.cols() * scale - 1)
    }

    fn get_scaled<F>(
        &self,
        location: &Location,
        scale: usize,
        scale_fn: F,
    ) -> Option<<Self as GridLike>::Item>
    where
        F: Fn(&<Self as GridLike>::Item, usize, usize) -> <Self as GridLike>::Item,
    {
        // we're out of bounds here
        let r_fac = location.row / self.rows();
        let c_fac = location.col / self.cols();
        if r_fac >= scale || c_fac >= scale {
            return None;
        }

        let row = location.row % self.rows();
        let col = location.col % self.cols();
        self.get(&Location::new(row, col))
            .map(|v| scale_fn(v, r_fac, c_fac))
    }
}

/// A generic representation of a 2D vector of locations, indexed by [Location].
#[derive(Debug, Clone, Default)]
pub struct Grid<T> {
    pub locations: Vec<Vec<T>>,
    pub rows: usize,
    pub cols: usize,
}

impl<T> Grid<T>
where
    T: Debug + Clone,
{
    pub fn new(locations: Vec<Vec<T>>) -> Self {
        let rows = locations.len();
        let cols = locations.first().map(|r| r.len()).unwrap_or_default();

        Grid {
            locations,
            rows,
            cols,
        }
    }
}

impl<T> GridLike for Grid<T>
where
    T: Debug + Clone,
{
    type Item = T;

    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn get(&self, location: &Location) -> Option<&Self::Item> {
        self.locations
            .get(location.row)
            .and_then(|r| r.get(location.col))
    }

    fn get_mut(&mut self, location: &Location) -> Option<&mut Self::Item> {
        self.locations
            .get_mut(location.row)
            .and_then(|r| r.get_mut(location.col))
    }
}

impl<T> Scalable for Grid<T> where T: Debug + Clone {}

impl<T> TryFrom<Vec<Vec<T>>> for Grid<T>
where
    T: Debug + Clone,
{
    type Error = AocError;

    fn try_from(value: Vec<Vec<T>>) -> Result<Self> {
        let rows = value.len();
        let cols = value.get(0).map(|c| c.len()).unwrap_or_default();

        if value.iter().any(|c| c.len() != cols) {
            return Err(AocError::GridConstructionError(
                "Not all rows are the same length".into(),
            ));
        }

        Ok(Self {
            locations: value,
            rows,
            cols,
        })
    }
}

impl<T> fmt::Display for Grid<T>
where
    T: Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = self.locations
            .iter()
            .map(|row| row.iter().map(|item| item.to_string()).collect::<String>())
            .join("\n");
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type GTest = Grid<usize>;

    #[test]
    fn general() {
        let empty = GTest::default();
        assert!(empty.is_empty());

        let values: Vec<Vec<usize>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let grid = GTest::try_from(values).expect("could not construct grid");
        assert!(!grid.is_empty());
        assert_eq!(grid.size(), 12);
        assert_eq!(grid.rows(), 3);
        assert_eq!(grid.cols(), 4);

        assert_eq!(grid.get(&Location::new(0, 0)), Some(&1));
        assert_eq!(grid.get(&Location::new(2, 3)), Some(&12));
        assert_eq!(grid.get(&Location::new(1, 2)), Some(&7));

        assert_eq!(grid.top_left(), Location::new(0, 0));
        assert_eq!(grid.bottom_right(), Location::new(2, 3));
    }

    #[test]
    fn mutating() {
        let values: Vec<Vec<usize>> = vec![vec![1, 2, 3, 4], vec![5, 6, 7, 8], vec![9, 10, 11, 12]];
        let mut grid = GTest::try_from(values).expect("could not construct grid");
        assert_eq!(grid.get(&Location::new(0, 0)), Some(&1));

        assert!(grid.set(&Location::new(0, 0), 222));
        assert_eq!(grid.get(&Location::new(0, 0)), Some(&222));
    }

    #[test]
    fn scale() {
        let values: Vec<Vec<usize>> = vec![vec![8]];
        let grid = GTest::try_from(values).expect("could not construct grid");
        let scale = 5;
        let scale_fn = |num: &usize, r_fac, c_fac| {
            let mut v = num + r_fac + c_fac;
            if v > 9 {
                v = v % 10 + 1;
            }
            v
        };

        assert_eq!(
            grid.get_scaled(&Location::new(0, 0), scale, scale_fn),
            Some(8)
        );
        assert_eq!(
            grid.get_scaled(&Location::new(1, 1), scale, scale_fn),
            Some(1)
        );
        assert_eq!(
            grid.get_scaled(&Location::new(1, 4), scale, scale_fn),
            Some(4)
        );
        assert_eq!(
            grid.get_scaled(&Location::new(2, 2), scale, scale_fn),
            Some(3)
        );
        assert_eq!(
            grid.get_scaled(&Location::new(3, 3), scale, scale_fn),
            Some(5)
        );
        assert_eq!(
            grid.get_scaled(&Location::new(4, 4), scale, scale_fn),
            Some(7)
        );
    }
}
