use std::fmt::{self, Display};
use std::hash::Hash;

use num::{Num, Bounded, Unsigned, Integer};

use super::Location;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Bound2D<T>
where
    T: Num + Bounded + PartialOrd + Copy + Default + Hash,
{
    pub min_x: T,
    pub max_x: T,
    pub min_y: T,
    pub max_y: T,
}

impl<T> Bound2D<T>
where
    T: Num + Bounded + PartialOrd + Copy + Default + Hash,
{
    pub fn new(min_x: T, max_x: T, min_y: T, max_y: T) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    /// Useful for having an initial state that can be used for generating a
    /// bound via iteration. `x_min` and `y_min` will be set to
    /// `T::max_value()`, and `x_man` and `y_max` will be set to
    /// `T::min_value()`.
    ///
    /// Example
    /// ```
    /// use aoc_helpers::generic::Bound2D;
    ///
    /// let b: Bound2D<i32> = Bound2D::minmax();
    /// let expected = Bound2D::new(i32::MAX, i32::MIN, i32::MAX, i32::MIN);
    ///
    /// assert_eq!(b, expected);
    /// ```
    pub fn minmax() -> Self {
        Self::new(T::max_value(), T::min_value(), T::max_value(), T::min_value())
    }

    pub fn contains(&self, x: T, y: T) -> bool {
        self.min_x <= x && self.max_x >= x && self.min_y <= y && self.max_y >= y
    }

    pub fn width(&self) -> T {
        self.max_x - self.min_x + T::one()
    }

    pub fn height(&self) -> T {
        self.max_y - self.min_y + T::one()
    }

    pub fn area(&self) -> T {
        self.width() * self.height()
    }
}

// Special case for usize
impl Bound2D<usize>
{
    /// Translate a given location by subtracting `min_y` from `loc.row` and
    /// `min_x` from `loc.col`. This is a special-case of Bound2D<usize>
    pub fn translate(&self, loc: &Location) -> Location {
        (loc.row - self.min_y, loc.col - self.min_x).into()
    }
}

impl<T> fmt::Display for Bound2D<T>
where
    T: Num + Bounded + PartialOrd + Copy + Default + Hash + Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bounds: min (x: {}, y: {}), max (x: {}, y: {})",
            self.min_x, self.min_y, self.max_x, self.max_y,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn minmax() {
        let b = Bound2D::minmax();
        let expected = Bound2D::new(i32::MAX, i32::MIN, i32::MAX, i32::MIN);

        assert_eq!(b, expected);
    }

    #[test]
    fn contains() {
        let b = Bound2D::new(-1, 1, -10, 10);
        assert!(b.contains(0, 0));
        assert!(b.contains(-1, -10));
        assert!(b.contains(1, 10));
        assert!(b.contains(-1, 10));
        assert!(!b.contains(1, 110));
    }

    #[test]
    fn properties() {
        let b = Bound2D::new(-1, 1, -10, 10);
        assert_eq!(b.width(), 3);
        assert_eq!(b.height(), 21);
        assert_eq!(b.area(), b.width() * b.height());
    }

    #[test]
    fn display() {
        let b = Bound2D::new(-1, 1, -10, 10);
        let expected = "Bounds: min (x: -1, y: -10), max (x: 1, y: 10)".to_string();
        assert_eq!(b.to_string(), expected);
    }
}
