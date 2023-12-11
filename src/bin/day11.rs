use anyhow::Result;
use itertools::Itertools;
use std::cmp::{max, min};
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

    println!("problem1 = {}", solve(&lines, 1)?);
    println!("problem2 = {}", solve(&lines, 1000000 - 1)?);
    Ok(())
}

fn solve(input: &Vec<String>, multiplier: usize) -> Result<usize> {
    let mut empty_cols = vec![true; input[0].len()];
    let mut galaxies = vec![];
    let empty_rows = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars().positions(|c| c == '#').fold(true, |_, x| {
                empty_cols[x] = false;
                galaxies.push((x, y));
                false
            })
        })
        .collect_vec();
    Ok(galaxies
        .iter()
        .tuple_combinations()
        .map(|(&(g1_x, g1_y), &(g2_x, g2_y))| {
            let min_x = min(g1_x, g2_x);
            let max_x = max(g1_x, g2_x);
            let dx = g1_x.abs_diff(g2_x)
                + multiplier * empty_cols[min_x..max_x].iter().filter(|&b| *b).count();
            let dy = g1_y.abs_diff(g2_y)
                + multiplier * empty_rows[g1_y..g2_y].iter().filter(|&b| *b).count();
            dx + dy
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = solve(&load_test_data(), 1).unwrap();
        assert_eq!(answer, 374);
    }

    #[test]
    fn problem2() {
        let answer = solve(&load_test_data(), 9).unwrap();
        assert_eq!(answer, 1030);
    }
}
