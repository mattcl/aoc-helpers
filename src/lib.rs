//! This crate provides common functionality and structures to aid in solving
//! Advent of Code problems. This is very much geared towards the way I solve
//! these problems, so there are no guarantees of compatibility.
pub use solution::Solution;
pub use util::load_input;

pub mod error;
pub mod generic;
pub mod util;
pub mod solution;
