use num_traits::ToPrimitive;
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

const INPUT_FILE: &str = concat!("./data/", env!("CARGO_BIN_NAME"), ".txt");

fn main() -> Result<()> {
    let input_file = File::open(Path::new(INPUT_FILE))?;
    let lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .collect::<Result<Vec<String>>>()?;

    println!("problem1 = {}", problem1_solution(&lines));
    println!("problem2 = {}", problem2_solution(&lines));
    Ok(())
}

fn problem1_solution(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter_map(|line| {
            let first = line.chars().find_map(|c| c.to_digit(10))?;
            let second = line.chars().rev().find_map(|c| c.to_digit(10))?;
            (first * 10 + second).to_usize()
        })
        .sum()
}

const NUMBERS: [[&'static str; 2]; 10] = [
    ["0", "zero"],
    ["1", "one"],
    ["2", "two"],
    ["3", "three"],
    ["4", "four"],
    ["5", "five"],
    ["6", "six"],
    ["7", "seven"],
    ["8", "eight"],
    ["9", "nine"],
];

fn problem2_solution(input: &Vec<String>) -> usize {
    input
        .iter()
        .map(|line| {
            let mut first_digit: (usize, usize) = (usize::MAX, usize::MAX);
            let mut last_digit: (usize, usize) = (usize::MIN, usize::MIN);
            for (n, patterns) in NUMBERS.iter().enumerate() {
                for &pattern in patterns {
                    if let Some(i) = line.find(pattern) {
                        first_digit = (i, n).min(first_digit);
                    }
                    if let Some(i) = line.rfind(pattern) {
                        last_digit = (i, n).max(last_digit);
                    }
                }
            }
            first_digit.1 * 10 + last_digit.1
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 142);
    }

    const SECOND_INPUT: &'static str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    fn load_second_data() -> Vec<String> {
        SECOND_INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_second_data());
        assert_eq!(answer, 281);
    }
}
