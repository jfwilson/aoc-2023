use std::num::ParseIntError;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

use anyhow::{bail, Result};
use itertools::Itertools;
use num_traits::Zero;

const INPUT_FILE: &str = concat!("./data/", env!("CARGO_BIN_NAME"), ".txt");

fn main() -> Result<()> {
    let input_file = File::open(Path::new(INPUT_FILE))?;
    let lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .collect::<Result<Vec<String>, Error>>()?;

    println!("problem1 = {}", problem_solution(&lines, false)?);
    println!("problem2 = {}", problem_solution(&lines, true)?);
    Ok(())
}

fn problem_solution(input: &Vec<String>, part2: bool) -> Result<i32> {
    input
        .into_iter()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|n| n.parse::<i32>())
                .collect::<Result<Vec<i32>, ParseIntError>>()?;
            extrapolate(numbers, part2)
        })
        .sum()
}

fn extrapolate(ns: Vec<i32>, part2: bool) -> Result<i32> {
    if ns.is_empty() {
        bail!("Not a valid sequence");
    } else if ns.iter().all(i32::is_zero) {
        Ok(0)
    } else {
        let diffs = ns.iter().tuple_windows().map(|(x, y)| y - x).collect_vec();
        Ok(if part2 {
            ns.first().unwrap() - extrapolate(diffs, part2)?
        } else {
            ns.last().unwrap() + extrapolate(diffs, part2)?
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem_solution(&load_test_data(), false).unwrap();
        assert_eq!(answer, 114);
    }

    #[test]
    fn problem2() {
        let answer = problem_solution(&load_test_data(), true).unwrap();
        assert_eq!(answer, 2);
    }
}
