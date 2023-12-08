use anyhow::Result;
use itertools::Itertools;
use num_integer::lcm;
use std::collections::HashMap;
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
    let (moves, network) = parse(input);
    let mut pos = "AAA";
    let mut count = 0;
    while pos != "ZZZ" {
        pos = network[pos][moves[count % moves.len()]];
        count += 1;
    }
    Ok(count)
}

fn problem2_solution(input: &Vec<String>) -> Result<usize> {
    let (moves, network) = parse(input);
    let mut acc = 1;
    println!("Moves length {}", moves.len());
    for &p in network.keys().filter(|&n| n.ends_with("A")) {
        let (init, cycle) = search_from(&moves, &network, p);
        let zs = cycle.iter().positions(|&p| p.ends_with("Z")).collect_vec();
        println!(
            "Starting at {:?}, solution = {} + n * {} + {:?}",
            init[0],
            init.len(),
            cycle.len(),
            zs
        );
        acc = lcm(acc, cycle.len());
    }
    Ok(acc)
}

fn parse(input: &Vec<String>) -> (Vec<usize>, HashMap<&str, [&str; 2]>) {
    let moves = input[0]
        .chars()
        .map(|c| match c {
            'L' => 0usize,
            _ => 1usize,
        })
        .collect_vec();
    let network: HashMap<&str, [&str; 2]> = input[2..]
        .iter()
        .map(|line| (&line[0..3], [&line[7..10], &line[12..15]]))
        .collect();
    (moves, network)
}

fn search_from<'a>(
    moves: &Vec<usize>,
    network: &'a HashMap<&str, [&str; 2]>,
    start: &'a str,
) -> (Vec<&'a str>, Vec<&'a str>) {
    let mut visited = vec![];
    let mut current = start;
    loop {
        let offset = visited.len() % moves.len();
        let current_pair = (current, offset);
        if visited.contains(&current_pair) {
            let mut i = visited.iter();
            let initial = i
                .by_ref()
                .peeking_take_while(|&v| v != &current_pair)
                .map(|&v| v.0)
                .collect_vec();
            let cycle = i.map(|&v| v.0).collect_vec();
            return (initial, cycle);
        }
        visited.push(current_pair);
        current = network[current][moves[offset]];
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT_1: &'static str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn problem1() {
        let answer = problem1_solution(&INPUT_1.lines().map(|s| s.to_owned()).collect()).unwrap();
        assert_eq!(answer, 2);
    }

    const INPUT_2: &'static str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn problem2() {
        let answer = problem2_solution(&INPUT_2.lines().map(|s| s.to_owned()).collect()).unwrap();
        assert_eq!(answer, 6);
    }
}
