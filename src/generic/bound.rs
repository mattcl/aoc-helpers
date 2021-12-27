use std::fmt::{self, Display};
use std::hash::Hash;

use num::Num;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct Bound2D<T>
where
    T: Num + PartialOrd + Copy + Default + Hash,
{
    pub min_x: T,
    pub max_x: T,
    pub min_y: T,
    pub max_y: T,
}

impl<T> Bound2D<T>
where
    T: Num + PartialOrd + Copy + Default + Hash,
{
    pub fn new(min_x: T, max_x: T, min_y: T, max_y: T) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
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

impl<T> fmt::Display for Bound2D<T>
where
    T: Num + PartialOrd + Copy + Default + Hash + Display,
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
