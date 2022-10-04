use serde::Serialize;
use serde_json;
use std::{
    convert::TryFrom,
    env,
    fmt::{Debug, Display},
};

use crate::load_input;

/// This struct enables printing a given solution in either plaintext or JSON,
/// depending on the presence of the `AOC_OUTPUT_JSON` ENV var. Its main purpose
/// is to standardize the output for consuption by the CI system.
///
/// # Usage
///
/// ```
/// use serde_json;
/// use aoc_helpers::Solution;
/// let s = Solution::new("hello world", 12345);
/// println!("{}", s);
///
/// assert_eq!(
///     s.to_string(),
///     "part 1: hello world\npart 2: 12345"
/// );
/// assert_eq!(
///     serde_json::to_string(&s).unwrap(),
///     "{\"part_one\":\"hello world\",\"part_two\":12345}".to_string()
/// );
/// ```
#[derive(Debug, Serialize, PartialEq)]
#[deprecated]
pub struct Solution<T, G>
where
    T: Display + Serialize + PartialEq,
    G: Display + Serialize + PartialEq,
{
    pub part_one: T,
    pub part_two: G,
}

/// The default implementation of `Solution` is as follows:
/// ```
/// use aoc_helpers::Solution;
/// let default = Solution::default();
/// let expected = Solution::new("not implemented", "not implemented");
///
/// assert_eq!(default, expected);
/// ```
impl Default for Solution<&str, &str> {
    fn default() -> Self {
        Solution::new("not implemented", "not implemented")
    }
}

impl<T, G> Solution<T, G>
where
    T: Display + Serialize + PartialEq,
    G: Display + Serialize + PartialEq,
{
    pub fn new(part_one: T, part_two: G) -> Self {
        Self { part_one, part_two }
    }
}

impl<T, G> Display for Solution<T, G>
where
    T: Display + Serialize + PartialEq,
    G: Display + Serialize + PartialEq,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if env::var("AOC_OUTPUT_JSON").is_ok() {
            // So it's probably not idiomatic to panic here, but, I want this
            // specific behavior in this specific case. I would not do this in
            // code destined for production
            write!(
                f,
                "{}",
                serde_json::to_string(&self).expect("unable to convert self to json")
            )
        } else {
            write!(f, "part 1: {}\npart 2: {}", self.part_one, self.part_two)
        }
    }
}

impl<T, G> From<(T, G)> for Solution<T, G>
where
    T: Display + Serialize + PartialEq,
    G: Display + Serialize + PartialEq,
{
    fn from(value: (T, G)) -> Self {
        Self::new(value.0, value.1)
    }
}

/// This trait is intended to enforce a standard interface for solutions such
/// that they are easier to consume by various components (examples, tests,
/// benchmarks, etc.).
#[deprecated]
pub trait Solver: TryFrom<Vec<String>>
where
    <Self as TryFrom<Vec<String>>>::Error: Debug,
{
    /// The title of this puzzle. Used for displaying in benchmarks and whatnot
    const ID: &'static str;

    /// The numerical day associated with this puzzle. Used for input loading
    /// and labeling.
    const DAY: usize;

    /// The type of the solution for part one
    type P1: Display + Serialize + PartialEq;

    /// The type of the solution for part two
    type P2: Display + Serialize + PartialEq;

    /// Produces the solution for part one. May panic. The reference to self
    /// here is mutable because it's possible the solve may have to mutate the
    /// solver for convenience or otherwise.
    fn part_one(&mut self) -> Self::P1;

    /// Produces the solution for part two. May panic. The reference to self
    /// here is mutable because it's possible the solve may have to mutate the
    /// solver for convenience or otherwise.
    fn part_two(&mut self) -> Self::P2;

    /// Returns a complete label for this puzzle in the form `001 my puzzle id`
    fn solver_label() -> String {
        format!(
            "{} {}",
            <Self as Solver>::solver_day(),
            <Self as Solver>::ID
        )
    }

    /// Returns the [String] representaiton of the day, zero-padded to len 3
    fn solver_day() -> String {
        format!("{:03}", <Self as Solver>::DAY)
    }

    /// Attempts to load input based on the DAY of this solver. This function
    /// can panic!
    fn load_input() -> Vec<String> {
        let day = <Self as Solver>::solver_day();
        load_input(&day).expect("could not load input")
    }

    /// Attempts to construct an instance of this solver from using the default
    /// input path determined by `ID`. This function can panic! This is
    /// necessary for selectively solving part one or part two for benchmarks.
    fn instance() -> Self {
        Self::try_from(Self::load_input()).expect("could not parse input")
    }

    /// Attempts to load the input and produces the combined part one and two
    /// [Solution]. This function will panic if the input can not be loaded or
    /// if the implementor cannot be constructed from the input. This may panic
    /// in the event the `part_one` or `part_two` implementations panic.
    /// Basically, you have to assume this function can panic!
    fn solve() -> Solution<Self::P1, Self::P2> {
        let mut solver = <Self as Solver>::instance();

        Solution::new(solver.part_one(), solver.part_two())
    }
}

#[cfg(test)]
mod tests {
    mod solver {
        use crate::error::AocError;

        use super::super::*;

        #[derive(Debug, Clone, Copy, Default)]
        pub struct Foo {}

        impl TryFrom<Vec<String>> for Foo {
            type Error = AocError;

            fn try_from(_: Vec<String>) -> Result<Self, Self::Error> {
                Ok(Foo {})
            }
        }

        impl Solver for Foo {
            const ID: &'static str = "A name";
            const DAY: usize = 1;

            type P1 = usize;
            type P2 = usize;

            fn part_one(&mut self) -> Self::P1 {
                0
            }

            fn part_two(&mut self) -> Self::P2 {
                1
            }
        }

        #[test]
        fn basics() {
            assert_eq!(Foo::solver_day(), String::from("001"));
            assert_eq!(Foo::solver_label(), String::from("001 A name"));
        }
    }
}
