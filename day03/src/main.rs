use std::fs;
use std::path::PathBuf;

use clap::Parser;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_while1;
use nom::combinator::map_res;
use nom::IResult;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_hint(clap::ValueHint::FilePath))]
    input_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let file_contents = fs::read_to_string(args.input_file).expect("Unable to read file");

    let part_1 = solve_part_1(&file_contents);
    println!("Part 1 anser {part_1}");
}

#[derive(PartialEq, Debug)]
struct Mul {
    x: usize,
    y: usize,
}

fn from_num_str(input: &str) -> Result<usize, std::num::ParseIntError> {
    input.parse()
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn parse_mul(input: &str) -> IResult<&str, Mul> {
    let (input, _) = tag("mul(")(input)?;
    let (input, x) = map_res(take_while1(is_digit), from_num_str)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = map_res(take_while1(is_digit), from_num_str)(input)?;
    let (input, _) = tag(")")(input)?;

    Ok((input, Mul { x, y }))
}

fn solve_part_1(input: &str) -> usize {
    let mut input_slice = input;
    let mut sum = 0;

    while input_slice.len() >= 8 {
        match parse_mul(input_slice) {
            Ok((remainder, mul)) => {
                input_slice = remainder;
                sum += mul.x * mul.y;
            }
            Err(_) => {
                input_slice = &input_slice[1..];
            }
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_tag() {
        let tag = "mul(1,2)";
        let result = parse_mul(&tag);
        assert_eq!(result, Ok(("", Mul { x: 1, y: 2 })));
    }

    #[test]
    fn test_part_1_example() {
        let memory = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = 161;
        let sum = solve_part_1(&memory);
        assert_eq!(result, sum);
    }
}
