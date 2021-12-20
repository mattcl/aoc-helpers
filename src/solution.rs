use serde::Serialize;
use serde_json;
use std::{env, fmt::Display};

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
#[derive(Debug, Serialize)]
pub struct Solution<T, G>
where
    T: Display + Serialize,
    G: Display + Serialize,
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
/// assert_eq!(default.part_one, expected.part_one);
/// assert_eq!(default.part_two, expected.part_two);
/// ```
impl Default for Solution<&str, &str> {
    fn default() -> Self {
        Solution::new("not implemented", "not implemented")
    }
}

impl<T, G> Solution<T, G>
where
    T: Display + Serialize,
    G: Display + Serialize,
{
    pub fn new(part_one: T, part_two: G) -> Self {
        Self { part_one, part_two }
    }
}

impl<T, G> Display for Solution<T, G>
where
    T: Display + Serialize,
    G: Display + Serialize,
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
    T: Display + Serialize,
    G: Display + Serialize,
{
    fn from(value: (T, G)) -> Self {
        Self::new(value.0, value.1)
    }
}
