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
    let (seeds, maps) = parse_input(input);
    solve(seeds, maps)
}

fn problem2_solution(input: &Vec<String>) -> usize {
    let (seed_ranges, maps) = parse_input(input);
    let seeds = seed_ranges
        .into_iter()
        .tuples()
        .flat_map(|(seed_start, len)| seed_start..(seed_start + len))
        .collect_vec();
    solve(seeds, maps)
}

fn solve(mut ids: Vec<usize>, maps: Vec<Vec<(usize, usize, usize)>>) -> usize {
    for map in maps {
        for id in ids.iter_mut() {
            *id = next_id(*id, &map)
        }
    }
    *ids.iter().min().unwrap()
}

fn next_id(id: usize, map: &Vec<(usize, usize, usize)>) -> usize {
    map.iter()
        .filter_map(|&(to_start, from_start, len)| {
            let offset = id.checked_sub(from_start)?;
            (offset < len).then_some(to_start + offset)
        })
        .next()
        .unwrap_or(id)
}

fn parse_input(input: &Vec<String>) -> (Vec<usize>, Vec<Vec<(usize, usize, usize)>>) {
    (
        input[0]
            .split(' ')
            .skip(1)
            .map(|n| n.parse::<usize>().unwrap())
            .collect_vec(),
        input[2..]
            .split(|line| line.is_empty())
            .map(|group| {
                group[1..]
                    .iter()
                    .map(|line| {
                        line.split(' ')
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect_tuple()
                            .unwrap()
                    })
                    .collect_vec()
            })
            .collect_vec(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data());
        assert_eq!(answer, 35);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data());
        assert_eq!(answer, 46);
    }
}
