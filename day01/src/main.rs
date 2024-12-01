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
    let parsed_lists = parse_input(&file_contents).expect("Failed to parse file");
    let result = calc_part_1(&parsed_lists);
    println!("Total is: {}", result)
}

struct ParsedLists {
    left: Vec<usize>,
    right: Vec<usize>,
}

fn parse_input(raw_text: &str) -> Result<ParsedLists, String> {
    let mut left = vec![];
    let mut right = vec![];

    for line in raw_text.lines() {
        match line.split_once(' ') {
            Some((l, r)) => match (l.trim().parse::<usize>(), r.trim().parse::<usize>()) {
                (Ok(l_u), Ok(r_u)) => {
                    left.push(l_u);
                    right.push(r_u)
                }
                _ => return Err(format!("Failed to parse left {} right {}", l, r)),
            },
            None => return Err(format!("Could not split {}", line)),
        }
    }

    left.sort();
    right.sort();

    Ok(ParsedLists { left, right })
}

fn calc_part_1(parsed_lists: &ParsedLists) -> usize {
    parsed_lists
        .left
        .iter()
        .zip(&parsed_lists.right)
        .map(|(left, right)| left.abs_diff(*right))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let test_input = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
        let test_output = 11;

        let parsed_lists = parse_input(&test_input).unwrap();
        let result = calc_part_1(&parsed_lists);

        assert_eq!(result, test_output)
    }
}
