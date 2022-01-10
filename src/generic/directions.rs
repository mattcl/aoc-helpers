//! This module contains several Enums representing different sets compass
//! directions.
use std::{convert::TryFrom, fmt, str::FromStr};

use crate::error::AocError;

/// Driections is an enum of both the Cardinal and Ordinal directions. It can
/// be parsed from various string representations
///
/// Example:
/// ```
/// use std::str::FromStr;
/// use aoc_helpers::generic::directions::Direction;
///
/// for v in ["North", "north", "N", "n"] {
///     assert_eq!(Direction::from_str(v).unwrap(), Direction::North);
/// }
///
/// for v in ["NorthEast", "northeast", "NE", "ne"] {
///     assert_eq!(Direction::from_str(v).unwrap(), Direction::NorthEast);
/// }
///
/// // etc..
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl FromStr for Direction {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_lowercase().as_str() {
            "north" | "n" => Self::North,
            "northeast" | "ne" => Self::NorthEast,
            "east" | "e" => Self::East,
            "southeast" | "se" => Self::SouthEast,
            "south" | "s" => Self::South,
            "southwest" | "sw" => Self::SouthWest,
            "west" | "w" => Self::West,
            "northwest" | "nw" => Self::NorthWest,
            _ => return Err(AocError::ParseDirectionError(s.to_string())),
        })
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = match self {
            Self::North => "North",
            Self::NorthEast => "NorthEast",
            Self::East => "East",
            Self::SouthEast => "SouthEast",
            Self::South => "South",
            Self::SouthWest => "SouthWest",
            Self::West => "West",
            Self::NorthWest => "NorthWest",
        };
        write!(f, "{}", out)
    }
}

impl From<Cardinal> for Direction {
    fn from(value: Cardinal) -> Self {
        Self::from(&value)
    }
}

impl From<&Cardinal> for Direction {
    fn from(value: &Cardinal) -> Self {
        match value {
            Cardinal::North => Self::North,
            Cardinal::South => Self::South,
            Cardinal::East => Self::East,
            Cardinal::West => Self::West,
        }
    }
}

impl From<HorizHex> for Direction {
    fn from(value: HorizHex) -> Self {
        Self::from(&value)
    }
}

impl From<&HorizHex> for Direction {
    fn from(value: &HorizHex) -> Self {
        match value {
            HorizHex::North => Self::North,
            HorizHex::NorthEast => Self::NorthEast,
            HorizHex::NorthWest => Self::NorthWest,
            HorizHex::South => Self::South,
            HorizHex::SouthEast => Self::SouthEast,
            HorizHex::SouthWest => Self::SouthWest,
        }
    }
}

impl From<VertHex> for Direction {
    fn from(value: VertHex) -> Self {
        Self::from(&value)
    }
}

impl From<&VertHex> for Direction {
    fn from(value: &VertHex) -> Self {
        match value {
            VertHex::East => Self::East,
            VertHex::NorthEast => Self::NorthEast,
            VertHex::SouthEast => Self::SouthEast,
            VertHex::West => Self::West,
            VertHex::NorthWest => Self::NorthWest,
            VertHex::SouthWest => Self::SouthWest,
        }
    }
}

/// Cardinal driections are North, South, East, and West. It can be pasrsed
/// much like the [Direction] enum.
///
/// Example:
/// ```
/// use std::str::FromStr;
/// use aoc_helpers::generic::directions::Cardinal;
///
/// for v in ["North", "north", "N", "n"] {
///     assert_eq!(Cardinal::from_str(v).unwrap(), Cardinal::North);
/// }
/// ```
///
/// Additionally, because these directions can be represented by a single char,
/// the Cardinal enum can also be made from chars, irrespective of case.
///
/// Example:
/// ```
/// use std::convert::TryFrom;
/// use aoc_helpers::generic::directions::Cardinal;
///
/// assert_eq!(Cardinal::try_from('n').unwrap(), Cardinal::North);
/// assert_eq!(Cardinal::try_from('N').unwrap(), Cardinal::North);
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Cardinal {
    North,
    South,
    East,
    West,
}

impl Cardinal {
    pub fn right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    pub fn left(&self) -> Self {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }
}

impl fmt::Display for Cardinal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Direction::from(self).fmt(f)
    }
}

impl FromStr for Cardinal {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match Direction::from_str(s)? {
            Direction::North => Self::North,
            Direction::South => Self::South,
            Direction::East => Self::East,
            Direction::West => Self::West,
            _ => return Err(AocError::ParseDirectionError(s.to_string())),
        })
    }
}

impl TryFrom<char> for Cardinal {
    type Error = AocError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'n' => Ok(Self::North),
            's' => Ok(Self::South),
            'e' => Ok(Self::East),
            'w' => Ok(Self::West),
            _ => Err(AocError::ParseDirectionError(value.to_string())),
        }
    }
}

/// HorizHex is an enum of compass directions that represent valid faces of a
/// hexagon with flat edges north and south.
///
/// See the following diagram:
/// ```text
///        n
///      +---+
/// nw  /     \  ne
///    +       +
/// sw  \     /  se
///      +---+
///        s
/// ```
/// It can be parsed from the standard set of direction strings
///
/// Example:
/// ```
/// use std::str::FromStr;
/// use aoc_helpers::generic::directions::HorizHex;
///
/// for v in ["North", "north", "N", "n"] {
///     assert_eq!(HorizHex::from_str(v).unwrap(), HorizHex::North);
/// }
///
/// for v in ["NorthEast", "northeast", "NE", "ne"] {
///     assert_eq!(HorizHex::from_str(v).unwrap(), HorizHex::NorthEast);
/// }
///
/// // etc..
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum HorizHex {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
}

impl FromStr for HorizHex {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match Direction::from_str(s)? {
            Direction::North => Self::North,
            Direction::NorthEast => Self::NorthEast,
            Direction::NorthWest => Self::NorthWest,
            Direction::South => Self::South,
            Direction::SouthEast => Self::SouthEast,
            Direction::SouthWest => Self::SouthWest,
            _ => return Err(AocError::ParseDirectionError(s.to_string())),
        })
    }
}

impl fmt::Display for HorizHex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Direction::from(self).fmt(f)
    }
}

/// VertHex is an enum of compass directions that represent valid faces of a
/// hexagon with flat edges west and east.
///
/// See the following diagram:
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
/// It can be parsed from the standard set of direction strings
///
/// Example:
/// ```
/// use std::str::FromStr;
/// use aoc_helpers::generic::directions::VertHex;
///
/// for v in ["East", "east", "E", "e"] {
///     assert_eq!(VertHex::from_str(v).unwrap(), VertHex::East);
/// }
///
/// for v in ["NorthEast", "northeast", "NE", "ne"] {
///     assert_eq!(VertHex::from_str(v).unwrap(), VertHex::NorthEast);
/// }
///
/// // etc..
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum VertHex {
    East,
    NorthEast,
    SouthEast,
    West,
    NorthWest,
    SouthWest,
}

impl FromStr for VertHex {
    type Err = AocError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match Direction::from_str(s)? {
            Direction::East => Self::East,
            Direction::NorthEast => Self::NorthEast,
            Direction::SouthEast => Self::SouthEast,
            Direction::West => Self::West,
            Direction::NorthWest => Self::NorthWest,
            Direction::SouthWest => Self::SouthWest,
            _ => return Err(AocError::ParseDirectionError(s.to_string())),
        })
    }
}

impl fmt::Display for VertHex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Direction::from(self).fmt(f)
    }
}

#[cfg(test)]
mod tests {
    mod cardinal {
        use super::super::*;

        #[test]
        fn parsing() {
            for v in ["North", "north", "N", "n"] {
                assert_eq!(Cardinal::from_str(v).unwrap(), Cardinal::North);
            }

            for v in ["South", "south", "S", "s"] {
                assert_eq!(Cardinal::from_str(v).unwrap(), Cardinal::South);
            }

            for v in ["East", "east", "E", "e"] {
                assert_eq!(Cardinal::from_str(v).unwrap(), Cardinal::East);
            }

            for v in ["West", "west", "W", "w"] {
                assert_eq!(Cardinal::from_str(v).unwrap(), Cardinal::West);
            }
        }
    }
}
