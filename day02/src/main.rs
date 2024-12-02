use std::{fs, path::PathBuf};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_hint(clap::ValueHint::FilePath))]
    input_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let file_contents = fs::read_to_string(args.input_file).expect("Unable to read file");
    let reactor = Reactor::try_from_text(&file_contents).expect("Could not parse your reactor");

    let count = reactor
        .data
        .iter()
        .map(|r| check_row_safety(&r))
        .filter(|f| f == &ReactorSafety::Safe)
        .count();

    println!("Safe rows {count}");
}

static MAXIMUM_MEASUREMENT_DELTA: usize = 3;

#[derive(PartialEq, Debug)]
enum ReactorSafety {
    Safe,
    UnsafeDelta,
    UnevenSlope,
    NoSlope,
}

#[derive(Debug)]
struct Reactor {
    data: Vec<Vec<usize>>,
}

impl Reactor {
    fn try_from_text(text: &str) -> Result<Reactor, String> {
        let mut data = vec![];

        for row in text.lines().map(parse_reactor_line_into_vec) {
            data.push(row?);
        }

        Ok(Reactor { data })
    }
}

fn parse_reactor_line_into_vec(line: &str) -> Result<Vec<usize>, String> {
    match line
        .split_ascii_whitespace()
        .map(str::parse::<usize>)
        .collect()
    {
        Ok(result) => Ok(result),
        Err(error) => Err(format!("Paring of {line} failed with error {error:?}")),
    }
}

fn check_row_safety(reactor_row: &[usize]) -> ReactorSafety {
    let mut row_iter = reactor_row.iter().peekable();

    if let Some(&first) = row_iter.next() {
        let mut prev = first;
        let sloping_up = row_iter.peek().is_some_and(|&next| prev > *next);

        while let Some(&col) = row_iter.next() {
            if prev.abs_diff(col) > MAXIMUM_MEASUREMENT_DELTA {
                return ReactorSafety::UnsafeDelta;
            }
            if prev == col {
                return ReactorSafety::NoSlope;
            }
            if (prev > col) != sloping_up {
                return ReactorSafety::UnevenSlope;
            }

            prev = col;
        }
    }

    ReactorSafety::Safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsafe_slope() {
        let slope = [0, 1, 0, 1, 2];
        let result = check_row_safety(&slope);
        assert_eq!(result, ReactorSafety::UnevenSlope);
    }

    #[test]
    fn test_safe_slope() {
        let slope = [0, 1, 2, 3, 4];
        let result = check_row_safety(&slope);
        assert_eq!(result, ReactorSafety::Safe);
    }

    #[test]
    fn test_safe_flat_start_slope() {
        let slope = [0, 0, 1, 2, 3];
        let result = check_row_safety(&slope);
        assert_eq!(result, ReactorSafety::NoSlope);
    }

    #[test]
    fn test_safe_plateau_slope() {
        let slope = [0, 1, 2, 3, 3];
        let result = check_row_safety(&slope);
        assert_eq!(result, ReactorSafety::NoSlope);
    }

    #[test]
    fn test_unsafe_delta() {
        let slope = [0, 1, 2, 3, 7];
        let result = check_row_safety(&slope);
        assert_eq!(result, ReactorSafety::UnsafeDelta);
    }

    #[test]
    fn test_unsafe_delta_with_uneven_slope() {
        // The unsafe delta should take prevelance over the uneven slope
        let slope = [1, 2, 3, 4, 0];
        let result = check_row_safety(&slope);
        assert_eq!(result, ReactorSafety::UnsafeDelta);
    }

    #[test]
    fn test_part_1() {
        let test_input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let test_output = 2;

        let reactor = Reactor::try_from_text(&test_input).unwrap();

        let count = reactor
            .data
            .iter()
            .map(|r| check_row_safety(r))
            .filter(|f| f == &ReactorSafety::Safe)
            .count();

        assert_eq!(count, test_output)
    }
}
