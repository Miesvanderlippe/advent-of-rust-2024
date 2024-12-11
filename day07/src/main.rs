use clap::Parser;
use std::fs;
use std::path::PathBuf;

use nom::bytes::complete::{tag, take_while1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_hint(clap::ValueHint::FilePath))]
    input_file: PathBuf,
    #[arg(short, long)]
    display_solution: bool,
}

fn main() {
    let args = Args::parse();

    let file_contents = fs::read_to_string(args.input_file).expect("Unable to read file");

    let part_1_answer = solve_part_1(&file_contents);
    println!("Part 1: {part_1_answer}");
}

fn solve_part_1(calibration_doc: &str) -> usize {
    let mut sum = 0;
    for line in calibration_doc.lines() {
        let (_, cal) = parse_input_line(line).unwrap();

        if calculate_recursively(0, cal.calibration_sum, &cal.calibration_vectors) {
            sum += cal.calibration_sum;
        }
    }
    sum
}

struct CalibrationEquation {
    calibration_sum: usize,
    calibration_vectors: Vec<usize>,
}

fn from_num_str(input: &str) -> Result<usize, std::num::ParseIntError> {
    input.parse()
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn parse_input_line(input: &str) -> IResult<&str, CalibrationEquation> {
    let (input, calibration_sum) = map_res(take_while1(is_digit), from_num_str)(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, raw_calibration_vectors) = separated_list1(tag(" "), take_while1(is_digit))(input)?;

    Ok((
        input,
        CalibrationEquation {
            calibration_sum,
            // TODO: Learn how to map seperated_list1 with a parser for each item...
            calibration_vectors: raw_calibration_vectors
                .iter()
                .map(|&num_str| num_str.parse().unwrap())
                .collect(),
        },
    ))
}

fn calculate_recursively(sum: usize, limit: usize, remainder: &[usize]) -> bool {
    let multiply_result = sum * remainder[0];
    let addition_result = sum + remainder[0];

    if remainder.len() == 1 {
        multiply_result == limit || addition_result == limit
    } else {
        // Only calculate a branch if the result is not already too big.
        (limit >= multiply_result && calculate_recursively(multiply_result, limit, &remainder[1..]))
            || (limit >= addition_result
                && calculate_recursively(addition_result, limit, &remainder[1..]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_recursively() {
        let inputs: [(usize, &[usize]); 9] = [
            (190, &[10, 19]),
            (3267, &[81, 40, 27]),
            (83, &[17, 5]),
            (156, &[15, 6]),
            (7290, &[6, 8, 6, 15]),
            (161011, &[16, 10, 13]),
            (192, &[17, 8, 14]),
            (21037, &[9, 7, 18, 13]),
            (292, &[11, 6, 16, 20]),
        ];

        let expected_sum = 3749;
        let mut actual_sum = 0;

        for (limit, factors) in inputs {
            if calculate_recursively(0, limit, &factors) {
                actual_sum += limit;
            }
        }
        assert_eq!(actual_sum, expected_sum)
    }

    #[test]
    fn test_part1_example() {
        let example_sum = 3749;
        let example_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!(solve_part_1(&example_input), example_sum);
    }
}
