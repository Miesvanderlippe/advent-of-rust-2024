use std::fs;
use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_hint(clap::ValueHint::FilePath))]
    input_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let file_contents = fs::read_to_string(args.input_file).expect("Unable to read file");

    let part_1 = solve_part_1(&file_contents, &"XMAS").expect("Could not solve the puzzle");
    println!("Part 1 answer {part_1}");
}

fn solve_part_1(puzzle: &str, search: &str) -> Result<usize, String> {
    let inverse_search: String = search.chars().rev().collect();
    let puzzle_lines = puzzle.lines().collect::<Vec<_>>();
    let puzzle_width = match puzzle
        .chars()
        .enumerate()
        .find(|(_, c)| *c == '\n' || *c == '\r')
    {
        Some((width, _)) => width,
        None => return Err(String::from("Puzzle does not contain newline")),
    };

    let mut word_count = 0;
    let last_vertical_search = puzzle_lines.len() - search.len();
    let word_len = search.len();
    // Starting point of first diagonal doing /
    let first_diag = word_len - 1;
    // Starting point of last diagonal doing \
    let last_diag = puzzle_width - word_len + 1;
    let last_horiz_start = puzzle_width - word_len + 1;

    for (row, &line) in puzzle_lines.iter().enumerate() {
        for start in 0..last_horiz_start {
            let current_word = &line[start..(start + word_len)];
            if current_word == search || current_word == &inverse_search {
                word_count += 1;
            }
        }

        if last_vertical_search >= row {
            for col in 0..puzzle_width {
                let vertical_word: String = puzzle_lines[row..(row + word_len)]
                    .iter()
                    .map(|&l| l.chars().skip(col).next().unwrap())
                    .collect();

                if vertical_word == search || vertical_word == inverse_search {
                    word_count += 1;
                }
            }

            for col in 0..last_diag {
                let diagonal: String = puzzle_lines[row..(row + word_len)]
                    .iter()
                    .enumerate()
                    .map(|(offset, &l)| l.chars().skip(col + offset).next().unwrap())
                    .collect();

                if diagonal == search || diagonal == inverse_search {
                    word_count += 1;
                }
            }

            for col in first_diag..puzzle_width {
                let diagonal: String = puzzle_lines[row..(row + word_len)]
                    .iter()
                    .enumerate()
                    .map(|(offset, &l)| l.chars().skip(col - offset).next().unwrap())
                    .collect();
                if diagonal == search || diagonal == inverse_search {
                    word_count += 1;
                }
            }
        }
    }

    Ok(word_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_first_col() {
        let word = "XMAS";
        let puzzle = "XZZZ
MZZZ
AZZZ
SZZZ";
        let result = 1;
        let sum = solve_part_1(&puzzle, &word);
        assert_eq!(Ok(result), sum);
    }

    #[test]
    fn test_part_1() {
        let word = "XMAS";
        let puzzle = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let result = 18;
        let sum = solve_part_1(&puzzle, &word);
        assert_eq!(Ok(result), sum);
    }
}
