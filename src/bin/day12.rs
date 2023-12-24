use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::borrow::Cow;
use std::collections::HashMap;
use std::num::ParseIntError;
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

    println!("problem1 = {}", solve(&lines, false)?);
    println!("problem2 = {}", solve(&lines, true)?);
    Ok(())
}

fn solve(input: &Vec<String>, part2: bool) -> Result<usize> {
    input
        .iter()
        .map(|line| {
            let (lhs, rhs) = line
                .split_whitespace()
                .collect_tuple()
                .ok_or(anyhow!("Unexpected line format"))?;
            let ns = rhs
                .split(",")
                .map(|n| n.parse::<usize>())
                .collect::<Result<Vec<usize>, ParseIntError>>()?;

            let (pattern, lengths) = if part2 {
                (
                    Cow::Owned((0..5).map(|_| lhs).join("?")),
                    (0..5).flat_map(|_| &ns).copied().collect_vec(),
                )
            } else {
                (Cow::Borrowed(lhs), ns)
            };
            let count = feasible_count(&pattern, &lengths);
            println!("{} {:?} == {}", lhs, lengths, count);
            Ok(count)
        })
        .sum()
}

fn feasible_count(pattern: &str, lengths: &[usize]) -> usize {
    feasible_count_inner(pattern, lengths, &mut HashMap::new())
}

fn feasible_count_inner(
    pattern: &str,
    lengths: &[usize],
    visited: &mut HashMap<usize, usize>,
) -> usize {
    let hash = (pattern.len() << 32) + lengths.len();
    if let Some(&c) = visited.get(&hash) {
        return c;
    };

    let count = if pattern.is_empty() {
        if lengths.is_empty() {
            1
        } else {
            0
        }
    } else if pattern.starts_with(".") {
        feasible_count_inner(&pattern[1..], lengths, visited)
    } else {
        let &n = lengths.get(0).unwrap_or(&usize::MAX);
        let train_count = if pattern.len() == n && lengths.len() == 1 {
            if can_be_train(&pattern[0..n]) {
                1
            } else {
                0
            }
        } else if pattern.len() > n {
            if can_be_train(&pattern[0..n]) && pattern[n..].starts_with(['.', '?']) {
                feasible_count_inner(&pattern[n + 1..], &lengths[1..], visited)
            } else {
                0
            }
        } else {
            0
        };
        if pattern.starts_with("#") {
            train_count
        } else {
            let skip_count = feasible_count_inner(&pattern[1..], lengths, visited);
            train_count + skip_count
        }
    };
    visited.insert(hash, count);
    count
}

fn can_be_train(pattern: &str) -> bool {
    pattern.chars().all(|c| c == '?' || c == '#')
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn feasible_count_test() {
        assert_eq!(feasible_count(".#.##....###.####", &[1, 2, 3, 4]), 1);
        assert_eq!(feasible_count(".#.##....###.####", &[1, 2, 4, 3]), 0);
    }

    #[test]
    fn problem1() {
        let answer = solve(&load_test_data(), false).unwrap();
        assert_eq!(answer, 21);
    }

    #[test]
    fn problem2() {
        let answer = solve(&load_test_data(), true).unwrap();
        assert_eq!(answer, 525152);
    }
}
