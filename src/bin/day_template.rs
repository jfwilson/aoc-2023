use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

const INPUT_FILE: &str = concat!("./data/", env!("CARGO_BIN_NAME"), ".txt");

fn main() -> Result<()> {
    let input_file = File::open(Path::new(INPUT_FILE))?;
    let lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .collect::<Result<Vec<String>, Error>>()?;

    println!("problem1 = {}", problem1_solution(&lines)?);
    println!("problem2 = {}", problem2_solution(&lines)?);
    Ok(())
}

fn problem1_solution(input: &Vec<String>) -> Result<usize> {
    Ok(input.into_iter().dedup().count())
}

fn problem2_solution(input: &Vec<String>) -> Result<usize> {
    input
        .into_iter()
        .dedup_with_count()
        .map(|tuple| tuple.0)
        .max()
        .ok_or(anyhow!("max of empty input"))
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "A
A
B
B
B";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 2);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 3);
    }
}
