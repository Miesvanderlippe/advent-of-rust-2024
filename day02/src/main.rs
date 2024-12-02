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
        .into_iter()
        .map(check_row_safety)
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
    row_size: usize,
    data: Vec<usize>,
}

struct ReactorIterator<'a> {
    reactor: &'a Reactor,
    cur: usize,
}

impl Reactor {
    fn try_from_text(text: &str) -> Result<Reactor, String> {
        let mut data: Vec<usize> = vec![];
        let mut lines = text.lines();

        if let Some(first) = lines.next() {
            let row_size = parse_reactor_line_into_vec(&mut data, first)?;

            while let Some(row) = lines.next() {
                let count = parse_reactor_line_into_vec(&mut data, row)?;
                if count != row_size {
                    return Err(format!(
                        "Row {row} had size {count} but expected {row_size}"
                    ));
                }
            }

            Ok(Reactor { row_size, data })
        } else {
            Err(String::from("Empty reactor"))
        }
    }
}

impl<'a> IntoIterator for &'a Reactor {
    type Item = &'a [usize];

    type IntoIter = ReactorIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ReactorIterator {
            reactor: &self,
            cur: 0,
        }
    }
}

impl<'a> Iterator for ReactorIterator<'a> {
    type Item = &'a [usize];

    fn next(&mut self) -> Option<Self::Item> {
        self.cur += 1;

        if self.reactor.data.len() >= self.cur * self.reactor.row_size {
            Some(
                &self.reactor.data
                    [(self.cur - 1) * self.reactor.row_size..self.cur * self.reactor.row_size],
            )
        } else {
            None
        }
    }
}

fn parse_reactor_line_into_vec(vec: &mut Vec<usize>, line: &str) -> Result<usize, String> {
    let mut count = 0;

    for number in line.split_ascii_whitespace() {
        count += 1;
        match number.parse::<usize>() {
            Ok(number) => vec.push(number),
            Err(err) => return Err(format!("Could not parse {number} with error {err}")),
        }
    }

    Ok(count)
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
            .into_iter()
            .map(check_row_safety)
            .filter(|f| f == &ReactorSafety::Safe)
            .count();

        assert_eq!(count, test_output)
    }
}
