use regex::Regex;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::ops::{Range, RangeInclusive};
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
    let re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    for (y, line) in input.iter().enumerate() {
        let ys = expand_range_inc(y..=y, 0..input.len());
        for m in re.find_iter(line) {
            let xs = expand_range(m.range(), 0..line.len());

            if has_symbol(&input[ys.clone()], xs) {
                let n = m.as_str().parse::<usize>().unwrap();
                sum += n;
            }
        }
    }
    sum
}

fn has_symbol(input: &[String], xs: RangeInclusive<usize>) -> bool {
    let re = Regex::new(r"[^.\d]").unwrap();
    input.iter().any(|line| re.is_match(&line[xs.clone()]))
}

fn problem2_solution(input: &Vec<String>) -> usize {
    let re = Regex::new(r"\d+").unwrap();
    let mut gears: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (y, line) in input.iter().enumerate() {
        let ys = expand_range_inc(y..=y, 0..input.len());
        for m in re.find_iter(line) {
            let xs = expand_range(m.range(), 0..line.len());
            let n = m.as_str().parse::<usize>().unwrap();

            for gy in ys.clone() {
                for gx in xs.clone() {
                    if input[gy][gx..].starts_with("*") {
                        let gear_nums = match gears.entry((gx, gy)) {
                            Entry::Occupied(o) => o.into_mut(),
                            Entry::Vacant(v) => v.insert(vec![]),
                        };
                        gear_nums.push(n);
                    }
                }
            }
        }
    }
    gears
        .values()
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums[0] * nums[1])
        .sum()
}

fn expand_range_inc(input: RangeInclusive<usize>, bounds: Range<usize>) -> RangeInclusive<usize> {
    expand_range(*input.start()..(*input.end() + 1), bounds)
}

fn expand_range(input: Range<usize>, bounds: Range<usize>) -> RangeInclusive<usize> {
    input
        .start
        .checked_sub(1)
        .unwrap_or_default()
        .max(bounds.start)..=input.end.min(bounds.end - 1)
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 4361);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 467835);
    }
}
