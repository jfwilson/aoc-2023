use anyhow::Result;
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
    let times = input[0]
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<usize>());
    let dists = input[1]
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<usize>());
    times
        .zip(dists)
        .map(|(t, d)| Ok(count_winners(t?, d?)))
        .product()
}

fn problem2_solution(input: &Vec<String>) -> Result<usize> {
    let time = input[0]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<usize>()?;
    let dist = input[1]
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse::<usize>()?;
    Ok(count_winners(time, dist))
}

fn count_winners(t: usize, d: usize) -> usize {
    (1..t)
        .filter(|charge_time| {
            let optimal_distance = (t - charge_time) * charge_time;
            optimal_distance > d
        })
        .count()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "Time:      7  15   30
Distance:  9  40  200";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 288);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 71503);
    }
}
