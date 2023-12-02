use itertools::Itertools;
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
            let (id, draws) = parse_line(line)?;
            draws
                .iter()
                .all(|&(r, g, b)| r <= 12 && g <= 13 && b <= 14)
                .then_some(id)
        })
        .sum()
}

fn problem2_solution(input: &Vec<String>) -> usize {
    input
        .iter()
        .filter_map(|line| {
            let (_, draws) = parse_line(line)?;
            let (r, g, b) = draws.iter().fold((0, 0, 0), |(ar, ag, ab), &(dr, dg, db)| {
                (ar.max(dr), ag.max(dg), ab.max(db))
            });
            Some(r * g * b)
        })
        .sum()
}

fn parse_line(line: &str) -> Option<(usize, Vec<(usize, usize, usize)>)> {
    let (game_id_txt, cubes_txt) = line.split(": ").collect_tuple()?;
    let game_id = usize::from_str_radix(&game_id_txt[5..], 10).ok()?;
    let cubes: Vec<(usize, usize, usize)> = cubes_txt
        .split("; ")
        .map(|draw_txt| {
            draw_txt
                .split(' ')
                .tuples()
                .fold((0, 0, 0), |(r, g, b), (count_txt, colour)| {
                    let count = usize::from_str_radix(count_txt, 10).unwrap();
                    if colour.starts_with("red") {
                        (r + count, g, b)
                    } else if colour.starts_with("green") {
                        (r, g + count, b)
                    } else {
                        (r, g, b + count)
                    }
                })
        })
        .collect_vec();
    Some((game_id, cubes))
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 8);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 2286);
    }
}
