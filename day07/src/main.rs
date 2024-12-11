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

        let solutions = calculate_recursively(
            0,
            cal.calibration_sum,
            &mut Solution::new(),
            &cal.calibration_vectors,
        );
        if solutions.len() >= 1 {
            sum += cal.calibration_sum;
        }
    }
    sum
}

#[derive(Clone)]
enum Operand {
    Add,
    Multiply,
}

#[derive(Clone)]
struct Solution {
    inner_data: Vec<Operand>,
}

impl Solution {
    fn add_operand(&mut self, new_operand: Operand) {
        self.inner_data.push(new_operand)
    }
    fn new() -> Self {
        Solution { inner_data: vec![] }
    }
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

fn calculate_recursively(
    sum: usize,
    limit: usize,
    solution: &mut Solution,
    remainder: &[usize],
) -> Vec<Solution> {
    let multiply_result = sum * remainder[0];
    let addition_result = sum + remainder[0];

    if remainder.len() == 1 {
        if multiply_result == limit && addition_result == limit {
            let mut m_solution = solution.clone();
            m_solution.add_operand(Operand::Multiply);

            let mut a_solution = solution.clone();
            a_solution.add_operand(Operand::Add);

            return vec![m_solution, a_solution];
        } else if multiply_result == limit {
            let mut m_solution = solution.clone();
            m_solution.add_operand(Operand::Multiply);

            return vec![m_solution];
        } else if addition_result == limit {
            let mut a_solution = solution.clone();
            a_solution.add_operand(Operand::Add);

            return vec![a_solution];
        } else {
            vec![]
        }
    } else {
        if limit >= multiply_result && limit >= addition_result {
            let mut m_solution = solution.clone();
            m_solution.add_operand(Operand::Multiply);

            let mut a_solution = solution.clone();
            a_solution.add_operand(Operand::Add);

            return vec![
                calculate_recursively(multiply_result, limit, &mut m_solution, &remainder[1..]),
                calculate_recursively(addition_result, limit, &mut a_solution, &remainder[1..]),
            ]
            .into_iter()
            .flatten()
            .collect();
        } else if limit >= multiply_result {
            let mut m_solution = solution.clone();
            m_solution.add_operand(Operand::Multiply);
            return calculate_recursively(multiply_result, limit, &mut m_solution, &remainder[1..]);
        } else if limit >= addition_result {
            let mut a_solution = solution.clone();
            a_solution.add_operand(Operand::Add);
            return calculate_recursively(addition_result, limit, &mut a_solution, &remainder[1..]);
        } else {
            vec![]
        }
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
            let solutions = calculate_recursively(0, limit, &mut Solution::new(), &factors);
            if solutions.len() >= 1 {
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
