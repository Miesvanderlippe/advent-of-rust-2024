use std::fmt::{self, Display, Formatter};
use std::fs;
use std::path::PathBuf;

use clap::Parser;
use nom::bytes::complete::{tag, take_while1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, value_hint(clap::ValueHint::FilePath))]
    input_file: PathBuf,
}

fn main() {
    let args = Args::parse();

    let file_contents = fs::read_to_string(args.input_file).expect("Unable to read file");
    let (rules, manual) = parse_input(&file_contents);

    let part_1_answer = solve_part_1(&rules, &manual);
    println!("Part 1: {part_1_answer}");

    let part_2_answer = solve_part_2(&rules, &manual);
    println!("Part 2: {part_2_answer}");
}

#[derive(PartialEq, Debug, Clone)]
struct PageOrderingRule {
    left: usize,
    right: usize,
}

impl PageOrderingRule {
    fn is_relevant_rule(&self, x: usize, y: usize) -> bool {
        (self.left == x && self.right == y) || (self.left == y && self.right == x)
    }
}

impl Display for PageOrderingRule {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.left, self.right)
    }
}

trait PageOrderingRules {
    fn get_relevant_rules(&self, pages: &[usize]) -> Vec<&PageOrderingRule>;
    fn has_correct_order(&self, pages: &[usize]) -> bool;
    fn sort_pages(&self, pages: &[usize]) -> Vec<usize>;
}

impl PageOrderingRules for Vec<PageOrderingRule> {
    fn get_relevant_rules(&self, pages: &[usize]) -> Vec<&PageOrderingRule> {
        self.iter()
            .filter(|&r| pages.contains(&r.left) && pages.contains(&r.right))
            .collect()
    }

    fn has_correct_order(&self, pages: &[usize]) -> bool {
        let relevant_rules = self.get_relevant_rules(pages);

        relevant_rules.iter().all(|rule| {
            pages.iter().position(|num| *num == rule.right)
                > pages.iter().position(|num| *num == rule.left)
        })
    }

    fn sort_pages(&self, pages: &[usize]) -> Vec<usize> {
        let relevant_rules = self.get_relevant_rules(&pages);

        let mut result = Vec::from(pages);
        result.sort_by(|a, b| {
            match relevant_rules
                .iter()
                .filter(|&&r| r.is_relevant_rule(*a, *b))
                .next()
            {
                Some(rule) => {
                    if *a == rule.left {
                        std::cmp::Ordering::Greater
                    } else {
                        std::cmp::Ordering::Less
                    }
                }
                None => std::cmp::Ordering::Equal,
            }
        });

        result
    }
}

enum ParseMode {
    Rules,
    Pages,
}

fn parse_input(input: &str) -> (Vec<PageOrderingRule>, Vec<Vec<usize>>) {
    let mut rules: Vec<PageOrderingRule> = vec![];
    let mut manual: Vec<Vec<usize>> = vec![];
    let mut mode: ParseMode = ParseMode::Rules;

    for line in input.lines() {
        match mode {
            ParseMode::Rules => {
                if line.is_empty() && rules.len() > 0 {
                    mode = ParseMode::Pages;
                } else {
                    rules.push(parse_ordering_rule(&line).unwrap().1);
                }
            }
            ParseMode::Pages => {
                manual.push(parse_pages(&line).unwrap().1);
            }
        }
    }
    (rules, manual)
}

fn solve_part_1(rules: &Vec<PageOrderingRule>, manual: &Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    for pagelist in manual {
        if rules.has_correct_order(&pagelist) {
            let middle_page = pagelist.get(pagelist.len().div_euclid(2)).unwrap();
            count += middle_page;
        }
    }

    count
}

fn solve_part_2(rules: &Vec<PageOrderingRule>, manual: &Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    for pagelist in manual {
        if !rules.has_correct_order(&pagelist) {
            let sorted_pages = rules.sort_pages(&pagelist);
            let middle_page = sorted_pages.get(sorted_pages.len().div_euclid(2)).unwrap();
            count += middle_page;
        }
    }

    count
}

fn from_num_str(input: &str) -> Result<usize, std::num::ParseIntError> {
    input.parse()
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn parse_ordering_rule(input: &str) -> IResult<&str, PageOrderingRule> {
    let (remainder, (high, low)) = separated_pair(
        map_res(take_while1(is_digit), from_num_str),
        tag("|"),
        map_res(take_while1(is_digit), from_num_str),
    )(input)?;

    Ok((
        remainder,
        PageOrderingRule {
            left: high,
            right: low,
        },
    ))
}

fn parse_pages(input: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(tag(","), map_res(take_while1(is_digit), from_num_str))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_parse_rule() {
        let result = parse_ordering_rule("47|53");
        assert_eq!(
            result,
            Ok((
                "",
                PageOrderingRule {
                    left: 47,
                    right: 53
                }
            ))
        );
    }

    #[test]
    fn test_parse_pages() {
        let result = parse_pages("75,47,61,53,29");
        assert_eq!(result, Ok(("", vec![75, 47, 61, 53, 29])));
    }

    #[test]
    fn test_part_1_first_col() {
        let (rules, manual) = parse_input(EXAMPLE_INPUT);
        let solution = solve_part_1(&rules, &manual);

        assert_eq!(solution, 143);
    }

    #[test]
    fn test_page_sort() {
        let ruleset_1 = vec![
            PageOrderingRule { left: 1, right: 2 },
            PageOrderingRule { left: 3, right: 4 },
            PageOrderingRule { left: 4, right: 1 },
        ];

        // Problematic operations
        //                               1 | 5
        // 1    5                        5 | 10
        // 1    5     10
        // 1    5     10                 5 | 9
        // 1    5 9   10
        // 1    5 9   10                 8 | 10
        // 1    5 9 8 10
        // 1    5 9 8 10                 8 | 9

        // The two ends of the book are easy to find

        // The only left with for which no right exists
        // The only right with for which no left exists

        // What rule to apply next?

        assert_eq!(vec![3, 4, 1, 2], ruleset_1.sort_pages(&vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_part_2() {
        let (rules, manual) = parse_input(EXAMPLE_INPUT);
        let solution = solve_part_2(&rules, &manual);

        assert_eq!(solution, 123);
    }
}
