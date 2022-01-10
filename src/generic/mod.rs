//! This groups various types and functions commonly required for AoC problems.
//!
//! Many of the types contained in this module are re-exported here for
//! convenience.
pub use self::bound::Bound2D;
pub use self::grid::Grid;
pub use self::location::Location;
pub use self::location::HorizHexLoc;
pub use self::location::VertHexLoc;

// pub
pub mod directions;
pub mod grid;
pub mod location;
pub mod pathing;
pub mod prelude;

// "private"
mod bound;
