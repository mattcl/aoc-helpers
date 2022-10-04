use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

use crate::error::{AocError, Result};

#[macro_export]
#[deprecated]
macro_rules! aoc_bench {
    // "standard" solution with two distinct parts
    ($name:ident, $solver:ty, $part1_desc:literal, $part2_desc:literal) => {
        pub fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(<$solver>::solver_label());
            group.bench_function($part1_desc, |b| {
                let mut solver = <$solver>::instance();
                b.iter(|| solver.part_one())
            });
            group.bench_function($part2_desc, |b| {
                let mut solver = <$solver>::instance();
                b.iter(|| solver.part_two())
            });
            group.finish();
        }
    };
    // combined solution
    ($name:ident, $solver:ty, $combined_desc:literal) => {
        pub fn $name(c: &mut Criterion) {
            let mut group = c.benchmark_group(<$solver>::solver_label());
            group.bench_function($combined_desc, |b| {
                let mut solver = <$solver>::instance();
                b.iter(|| {
                    solver.part_one();
                    solver.part_two();
                })
            });
            group.finish();
        }
    };
}

/// Defines a set of benchmarks consisting of benches for each specified example
/// as well as a combined set of benchmarks
/// Example
/// ```ignore
/// use aoc_helpers::aoc_benches;
///
/// aoc_benches!{
///     (
///         day_001,
///         something_that_implements_Solution,
///         "Part 1 description",
///         "Part 2 description"
///     )
///     // ...
/// }
/// ```
#[macro_export]
#[deprecated]
macro_rules! aoc_benches {
    ($comb_seconds:literal, $(($name:ident, $solver:ty, $($description:literal),+)),+) => {
        use criterion::{criterion_group, Criterion};
        use std::time::Duration;

        $(
            aoc_helpers::aoc_bench!($name, $solver, $($description),+);
        )+

        pub fn aoc_combined(c: &mut Criterion) {
            let mut group = c.benchmark_group("Advent of Code");
            group.measurement_time(Duration::new($comb_seconds, 0));
            group.bench_function("Total runtime for all solutions, including parsing", |b| {
                b.iter(|| {
                    $(
                        <$solver>::solve();
                    )+
                })
            });
            group.finish();
        }

        criterion_group!(benches, $($name,)+ aoc_combined);
    };
    ($(($name:ident, $solver:ty, $($description:literal),+)),+) => {
        aoc_benches!{
            10, $( ($name, $solver, $($description),+)),+
        }
    };
}

/// Will attempt to load input from the specified `AOC_INPUT` file, otherwise
/// will default to loading the corresponding input file for the day given by
/// `default_day`.
///
/// ```no_run
/// use aoc_helpers::util::load_input;
/// let lines: Vec<String> = load_input("002").expect("could not load input");
/// ```
#[deprecated]
pub fn load_input(default_day: &str) -> Result<Vec<String>> {
    //
    // examples/003_toboggan-trajectory/input
    //
    load_external_input("AOC_INPUT").or_else(|e| {
        // If we errored because the var was not set, just return the
        // the default. Otherwise, we want to propagate the error because
        // it means that the var *was* set but we couldn't open/load the
        // file.
        match e {
            AocError::VarError(_) => load_named_input(default_day, "input"),
            _ => Err(e),
        }
    })
}

#[deprecated]
pub fn load_named_input(day: &str, name: &str) -> Result<Vec<String>> {
    //
    // examples/003_toboggan-trajectory/<name>
    //
    let examples_dir = Path::new("examples");
    for entry in fs::read_dir(examples_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && entry.file_name().into_string()?.starts_with(day) {
            if let Some(file) = path.join(name).to_str() {
                return load_lines(file);
            }
        }
    }

    Err(AocError::InputMissing(format!("{}: '{}'", day, name)))
}

#[deprecated]
pub fn load_external_input(key: &str) -> Result<Vec<String>> {
    let path = env::var(key)?;
    load_lines(&path)
}

#[deprecated]
pub fn load_lines(file: &str) -> Result<Vec<String>> {
    let mut lines = Vec::new();
    for line in BufReader::new(File::open(Path::new(file))?).lines() {
        lines.push(line?);
    }

    Ok(lines)
}

#[deprecated]
pub fn parse_input<T: FromStr>(
    input: &[String],
) -> std::result::Result<Vec<T>, <T as FromStr>::Err> {
    input.iter().map(|l| T::from_str(l)).collect()
}

#[deprecated]
pub fn test_input(input: &str) -> Vec<String> {
    input
        .trim()
        .split('\n')
        .map(|s| s.trim().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_conversion() {
        let expected = vec![
            "abcd".to_string(),
            "".to_string(),
            "".to_string(),
            "efgh".to_string(),
            "ijkl".to_string(),
            "mnop".to_string(),
            "".to_string(),
            "qrs".to_string(),
        ];

        let input = "
            abcd


            efgh
            ijkl
            mnop

            qrs
        ";
        assert_eq!(test_input(input), expected);
    }
}
