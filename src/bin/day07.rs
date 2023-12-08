use anyhow::Result;
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
    total_score(input, false)
}

fn problem2_solution(input: &Vec<String>) -> Result<usize> {
    total_score(input, true)
}

fn total_score(lines: &Vec<String>, jokers: bool) -> Result<usize> {
    let mut cards: Vec<([u8; 7], usize)> = lines
        .iter()
        .map(|line| parse_line(line, jokers))
        .collect::<Result<Vec<([u8; 7], usize)>>>()?;
    cards.sort();
    Ok(cards
        .into_iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) * bid)
        .sum())
}

fn parse_line(line: &str, jokers: bool) -> Result<([u8; 7], usize)> {
    let bid = line[6..].parse::<usize>()?;
    let values = line.as_bytes()[0..5]
        .iter()
        .map(|b| match b {
            b'A' => 14,
            b'K' => 13,
            b'Q' => 12,
            b'J' => jokers.then_some(0).unwrap_or(11),
            b'T' => 10,
            b => b - b'0',
        })
        .collect_vec();
    let mut value_counts = values.iter().counts();
    let joker_count = value_counts.remove(&0).unwrap_or_default() as u8;
    let mut counts = value_counts.values().map(|c| *c as u8).collect_vec();
    counts.sort();
    let mut result: [u8; 7] = Default::default();
    result[0] = counts.pop().unwrap_or_default() + joker_count;
    result[1] = counts.pop().unwrap_or_default();
    result[2..].copy_from_slice(values.as_slice());
    Ok((result, bid))
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn parse_jacks() {
        let answer = parse_line("33TTT 540", false).unwrap();
        assert_eq!(answer, ([3, 2, 3, 3, 10, 10, 10], 540));
        let answer = parse_line("KTJJT 999", false).unwrap();
        assert_eq!(answer, ([2, 2, 13, 10, 11, 11, 10], 999));
    }

    #[test]
    fn parse_jokers() {
        let answer = parse_line("33TTT 540", true).unwrap();
        assert_eq!(answer, ([3, 2, 3, 3, 10, 10, 10], 540));
        let answer = parse_line("KTJJT 999", true).unwrap();
        assert_eq!(answer, ([4, 1, 13, 10, 0, 0, 10], 999));
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 6440);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 5905);
    }
}
