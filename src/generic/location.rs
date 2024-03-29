use itertools::Itertools;
use std::{marker::PhantomData, str::FromStr};

use crate::error::{AocError, Result};

use super::directions::{HorizHexDir, VertHexDir};

/// A `Location` specifies a pair of [usize], [usize] representing a `row` and
/// `column` respectively. Primarily this is used to interact with [GridLike](super::grid::GridLike)
/// things, and has some additional functionality beyond a generic `Point`,
/// like the ability to iterate over neighboring `Locations`
#[derive(Debug, Clone, Copy, Default, Hash, Eq, PartialEq)]
pub struct Location {
    pub row: usize,
    pub col: usize,
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.row
            .cmp(&other.row)
            .then_with(|| self.col.cmp(&other.col))
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Location {
    const ORTH_LOCS: [(i64, i64); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }

    /// Given a number of rows, get the representation of this [Location] as
    /// an index into a row-major grid
    pub fn as_rm_index(&self, num_cols: usize) -> usize {
        self.row * num_cols + self.col
    }

    /// Given an index and number of rows, construct a [Location] from an index
    /// into a row-major grid
    pub fn from_rm_index(idx: usize, num_cols: usize) -> Self {
        Location::new(idx / num_cols, idx % num_cols)
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

    pub fn south_east(&self) -> Option<Location> {
        Some((self.row + 1, self.col + 1).into())
    }

    pub fn south_west(&self) -> Option<Location> {
        if self.col == 0 {
            return None;
        }

        Some((self.row + 1, self.col - 1).into())
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

/// Underlying axial-coordinate representation of a location in a hexagonal grid.
///
/// For a more detailed explanation, see [this](https://www.redblobgames.com/grids/hexagons/).
///
/// In this case, the generic parameter `T` is a marker for determining which
/// implementation to use for functions that operate on neighbors of the
/// location. *The only valid markers are [HorizHexDir] and [VertHexDir].* The
/// [HorizHexLoc] and [VertHexLoc] type aliases are provided for convenience.
///
/// # Examples
/// ```
/// use aoc_helpers::generic::HorizHexLoc;
///
/// let hex = HorizHexLoc::default();
///
/// assert_eq!(hex, HorizHexLoc::from((0, 0)));
/// ```
/// note: [HorizHexLoc] and [VertHexLoc] are not equivalent even if they have
/// the same values for `q` and `r` because their marker types are different.
///
/// Due to orientation differences, getting the "same" neighbor direction does
/// not always result in the same modifications to internal representation.
/// ```
/// use aoc_helpers::generic::{HorizHexLoc, VertHexLoc};
/// use aoc_helpers::generic::directions::{HorizHexDir, VertHexDir};
///
/// let h_hex = HorizHexLoc::default();
/// let v_hex = VertHexLoc::default();
///
/// // Note how the South East neighbor has a different internal representation
/// // between the horizontal and vertical implementations
/// assert_eq!(h_hex.get_neighbor(&HorizHexDir::SouthEast), HorizHexLoc::from((1, 0)));
/// assert_eq!(v_hex.get_neighbor(&VertHexDir::SouthEast), VertHexLoc::from((0, 1)));
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct HexLocation<T> {
    pub q: i64,
    pub r: i64,
    _orientation: PhantomData<T>,
}

impl<T> Default for HexLocation<T> {
    fn default() -> Self {
        (0, 0).into()
    }
}

impl<T> From<(i64, i64)> for HexLocation<T> {
    fn from(v: (i64, i64)) -> Self {
        Self {
            q: v.0,
            r: v.1,
            _orientation: PhantomData,
        }
    }
}

impl<T> HexLocation<T> {
    /// Construct a new [HexLocation]. Equivalent to `(q, r).into()`
    ///
    /// # Examples
    /// ```
    /// use aoc_helpers::generic::HorizHexLoc;
    ///
    /// let hex = HorizHexLoc::new(1, 3);
    ///
    /// assert_eq!(hex, HorizHexLoc::from((1, 3)));
    /// ```
    pub fn new(q: i64, r: i64) -> Self {
        (q, r).into()
    }

    /// Returns this location's `q` value.
    pub fn q(&self) -> i64 {
        self.q
    }

    /// Returns this location's `r` value.
    pub fn r(&self) -> i64 {
        self.r
    }

    /// Returns this location's `s` value.
    ///
    /// While we do not store `s`, it can be caluclated as `-q - r`.
    pub fn s(&self) -> i64 {
        -self.q - self.r
    }

    /// Returns the distance to another location as an [i64].
    ///
    /// note: the return type of [i64] was chosen for convenience. Values
    /// returned by this funciton will always be positive or zero.
    ///
    /// # Example
    /// ```
    /// use aoc_helpers::generic::{HorizHexLoc, VertHexLoc};
    ///
    /// let h_hex = HorizHexLoc::new(3, -2);
    /// let v_hex = VertHexLoc::new(3, -2);
    ///
    /// // Distances are equivalent regardless of orientation
    /// assert_eq!(HorizHexLoc::default().distance(&h_hex), 3);
    /// assert_eq!(VertHexLoc::default().distance(&v_hex), 3);
    /// ```
    pub fn distance(&self, other: &Self) -> i64 {
        ((self.q - other.q).abs()
            + (self.q + self.r - other.q - other.r).abs()
            + (self.r - other.r).abs())
            / 2
    }
}

/// A [HexLocation] where North and South are flat faces
///
/// See diagram:
/// ```text
///        n
///      +---+
/// nw  /     \  ne
///    +       +
/// sw  \     /  se
///      +---+
///        s
/// ```
///
/// Orientation-dependent methods like [`get_neighbor`](HorizHexLoc::get_neighbor) have slightly
/// different implementations between this and [VertHexLoc].
pub type HorizHexLoc = HexLocation<HorizHexDir>;

impl HexLocation<HorizHexDir> {
    /// An array of [HorizHexDir] for use when iterating over neighbors.
    ///
    /// The order in this case is `N -> NE -> SE -> S -> SW -> NW`
    pub const NEIGHBOR_ORDER: [HorizHexDir; 6] = [
        HorizHexDir::North,
        HorizHexDir::NorthEast,
        HorizHexDir::SouthEast,
        HorizHexDir::South,
        HorizHexDir::SouthWest,
        HorizHexDir::NorthWest,
    ];
    /// Given a reference to a [HorizHexDir], return the neighbor in that direction.
    ///
    /// # Examples
    /// ```
    /// use aoc_helpers::generic::HorizHexLoc;
    /// use aoc_helpers::generic::directions::HorizHexDir;
    ///
    /// let loc = HorizHexLoc::from((1, 1));
    /// assert_eq!(loc.get_neighbor(&HorizHexDir::NorthEast), HorizHexLoc::from((2, 0)));
    /// assert_eq!(loc.get_neighbor(&HorizHexDir::SouthEast), HorizHexLoc::from((2, 1)));
    /// ```
    pub fn get_neighbor(&self, dir: &HorizHexDir) -> Self {
        match dir {
            HorizHexDir::North => (self.q, self.r - 1).into(),
            HorizHexDir::NorthEast => (self.q + 1, self.r - 1).into(),
            HorizHexDir::NorthWest => (self.q - 1, self.r).into(),
            HorizHexDir::South => (self.q, self.r + 1).into(),
            HorizHexDir::SouthEast => (self.q + 1, self.r).into(),
            HorizHexDir::SouthWest => (self.q - 1, self.r + 1).into(),
        }
    }

    /// Yields an iterator over the neighbors of this location.
    ///
    /// The order in this case is `N -> NE -> SE -> S -> SW -> NW`
    pub fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        Self::NEIGHBOR_ORDER
            .iter()
            .map(move |dir| self.get_neighbor(dir))
    }
}

/// A [HexLocation] where West and East are flat faces
///
/// See diagram:
/// ```text
///       +
///      / \
/// nw  /   \  ne
///    /     \
///   +       +
///   |       |
/// w |       | e
///   |       |
///   +       +
///    \     /
/// sw  \   /  se
///      \ /
///       +
/// ```
/// Orientation-dependent methods like [`get_neighbor`](VertHexLoc::get_neighbor) have slightly
/// different implementations between this and [HorizHexLoc].
pub type VertHexLoc = HexLocation<VertHexDir>;

impl HexLocation<VertHexDir> {
    /// An array of [VertHexDir] for use when iterating over neighbors.
    ///
    /// The order in this case is `E -> NE -> NW -> W -> SW -> SE`
    pub const NEIGHBOR_ORDER: [VertHexDir; 6] = [
        VertHexDir::East,
        VertHexDir::NorthEast,
        VertHexDir::NorthWest,
        VertHexDir::West,
        VertHexDir::SouthWest,
        VertHexDir::SouthEast,
    ];

    /// Given a reference to a [VertHexDir], return the neighbor in that direction.
    ///
    /// # Examples
    /// ```
    /// use aoc_helpers::generic::VertHexLoc;
    /// use aoc_helpers::generic::directions::VertHexDir;
    ///
    /// let loc = VertHexLoc::from((1, 1));
    /// assert_eq!(loc.get_neighbor(&VertHexDir::NorthEast), VertHexLoc::from((2, 0)));
    /// assert_eq!(loc.get_neighbor(&VertHexDir::SouthEast), VertHexLoc::from((1, 2)));
    /// ```
    pub fn get_neighbor(&self, dir: &VertHexDir) -> Self {
        match dir {
            VertHexDir::East => (self.q + 1, self.r).into(),
            VertHexDir::NorthEast => (self.q + 1, self.r - 1).into(),
            VertHexDir::SouthEast => (self.q, self.r + 1).into(),
            VertHexDir::West => (self.q - 1, self.r).into(),
            VertHexDir::NorthWest => (self.q, self.r - 1).into(),
            VertHexDir::SouthWest => (self.q - 1, self.r + 1).into(),
        }
    }

    /// Yields an iterator over the neighbors of this location.
    ///
    /// The order in this case is `E -> NE -> NW -> W -> SW -> SE`
    pub fn neighbors(&self) -> impl Iterator<Item = Self> + '_ {
        Self::NEIGHBOR_ORDER
            .iter()
            .map(move |dir| self.get_neighbor(dir))
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
