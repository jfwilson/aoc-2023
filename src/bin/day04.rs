use itertools::Itertools;
use regex::Regex;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

const INPUT_FILE: &str = concat!("./data/", env!("CARGO_BIN_NAME"), ".txt");

fn main() -> Result<(), Error> {
    let input_file = File::open(Path::new(INPUT_FILE))?;
    let lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .collect::<Result<Vec<String>, Error>>()?;

    println!("problem1 = {}", problem1_solution(&lines));
    println!("problem2 = {}", problem2_solution(&lines));
    Ok(())
}

fn problem1_solution(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|line| {
            let count = calc_win_count(line).unwrap();
            count
                .checked_sub(1)
                .map(|count| 1 << count)
                .unwrap_or_default()
        })
        .sum()
}

fn problem2_solution(input: &Vec<String>) -> usize {
    let mut counts: Vec<usize> = input.iter().map(|_| 1).collect_vec();
    for (i, line) in input.iter().enumerate() {
        let win_count = calc_win_count(line).unwrap();
        let card_count = counts[i];
        for j in 1..=win_count {
            counts[i + j] += card_count;
        }
    }
    counts.iter().sum()
}

fn calc_win_count(line: &str) -> Option<usize> {
    let (_, numbers_txt) = line.split(": ").collect_tuple()?;
    let (lhs, rhs) = numbers_txt
        .split(" | ")
        .map(parse_numbers)
        .collect_tuple()?;
    Some(rhs.iter().filter(|n| lhs.contains(n)).count())
}

fn parse_numbers(input: &str) -> Vec<usize> {
    let re = Regex::new(r"\d+").unwrap();
    re.find_iter(input)
        .map(|n| n.as_str().parse::<usize>().unwrap())
        .collect_vec()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 13);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 30);
    }
}
