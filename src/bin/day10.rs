use crate::Direction::{DOWN, LEFT, RIGHT, UP};
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::mem::swap;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};
use tailcall::tailcall;

const INPUT_FILE: &str = concat!("./data/", env!("CARGO_BIN_NAME"), ".txt");

fn main() -> Result<()> {
    let input_file = File::open(Path::new(INPUT_FILE))?;
    let lines: Vec<String> = BufReader::new(input_file)
        .lines()
        .collect::<Result<Vec<String>, Error>>()?;

    println!(
        "problem1 = {}",
        solve(&lines, false).ok_or(anyhow!("No solution found"))?
    );
    println!(
        "problem2 = {}",
        solve(&lines, true).ok_or(anyhow!("No solution found"))?
    );
    Ok(())
}

const DIRS: [Direction; 4] = [UP, DOWN, LEFT, RIGHT];
const DIR_XS: [isize; 4] = [0, 0, -1, 1];
const DIR_YS: [isize; 4] = [-1, 1, 0, 0];

#[derive(Copy, Clone, Debug, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn solve(grid: &Vec<String>, part2: bool) -> Option<usize> {
    let start_pos = grid
        .iter()
        .enumerate()
        .find_map(|(y, line)| line.find("S").map(|x| (x, y)))?;
    let mut pipe = vec![start_pos];

    let half_pipe_len = DIRS.iter().find_map(|&d| {
        pipe.drain(1..);
        pipe_length(grid.as_slice(), &mut pipe, d)
    })?;

    if part2 {
        let (horizontal_walls_below, vertical_walls_to_right_of) = build_inner_walls(pipe)?;
        let outside_cells =
            calculate_outside_cells(grid, horizontal_walls_below, vertical_walls_to_right_of);
        Some(grid.len() * grid[0].len() - outside_cells.len())
    } else {
        Some(half_pipe_len)
    }
}

#[tailcall]
fn pipe_length(grid: &[String], visited: &mut Vec<(usize, usize)>, d: Direction) -> Option<usize> {
    let &s = visited.last()?;
    let (nx, ny) = next_pos(grid, s, d)?;
    visited.push((nx, ny));
    let n = &grid[ny][nx..=nx];
    match (n, d) {
        ("S", _) => Some(visited.len() >> 1),
        ("|", DOWN) => pipe_length(grid, visited, d),
        ("|", UP) => pipe_length(grid, visited, d),
        ("-", RIGHT) => pipe_length(grid, visited, d),
        ("-", LEFT) => pipe_length(grid, visited, d),
        ("L", DOWN) => pipe_length(grid, visited, RIGHT),
        ("L", LEFT) => pipe_length(grid, visited, UP),
        ("J", DOWN) => pipe_length(grid, visited, LEFT),
        ("J", RIGHT) => pipe_length(grid, visited, UP),
        ("F", UP) => pipe_length(grid, visited, RIGHT),
        ("F", LEFT) => pipe_length(grid, visited, DOWN),
        ("7", UP) => pipe_length(grid, visited, LEFT),
        ("7", RIGHT) => pipe_length(grid, visited, DOWN),
        _ => None,
    }
}

fn next_pos(grid: &[String], from: (usize, usize), d: Direction) -> Option<(usize, usize)> {
    let nx = from
        .0
        .checked_add_signed(DIR_XS[d as usize])
        .filter(|&v| v < grid[0].len())?;
    let ny = from
        .1
        .checked_add_signed(DIR_YS[d as usize])
        .filter(|&v| v < grid.len())?;
    Some((nx, ny))
}

fn build_inner_walls(
    mut pipe: Vec<(usize, usize)>,
) -> Option<(Vec<(usize, usize)>, Vec<(usize, usize)>)> {
    let mut horizontal_walls_below = vec![];
    let mut vertical_walls_to_right_of = vec![];
    let &top_left = pipe.iter().min()?;
    let mut i = pipe.iter().position(|&c| c == top_left)?;
    if pipe[i + 1].0 == top_left.0 {
        pipe.reverse();
        i = pipe.iter().position(|&c| c == top_left)?;
    }
    let mut in_d = UP;
    for (curr, next) in pipe[i..].iter().chain(&pipe[1..=i]).tuple_windows() {
        let dx = next.0 as isize - curr.0 as isize;
        let dy = next.1 as isize - curr.1 as isize;
        let out_d = DIRS[DIR_XS
            .iter()
            .copied()
            .zip(DIR_YS)
            .position(|c| c == (dx, dy))?];
        let walls = match (in_d, out_d) {
            (RIGHT, RIGHT) => vec![DOWN],
            (LEFT, LEFT) => vec![UP],
            (UP, UP) => vec![RIGHT],
            (DOWN, DOWN) => vec![LEFT],
            (RIGHT, UP) => vec![DOWN, RIGHT],
            (UP, LEFT) => vec![RIGHT, UP],
            (LEFT, DOWN) => vec![UP, LEFT],
            (DOWN, RIGHT) => vec![LEFT, DOWN],
            _ => vec![],
        };
        for w in walls {
            match w {
                UP => horizontal_walls_below.push((curr.0, curr.1 - 1)),
                DOWN => horizontal_walls_below.push(*curr),
                LEFT => vertical_walls_to_right_of.push((curr.0 - 1, curr.1)),
                RIGHT => vertical_walls_to_right_of.push(*curr),
            }
        }
        in_d = out_d;
    }
    Some((horizontal_walls_below, vertical_walls_to_right_of))
}

fn calculate_outside_cells(
    grid: &Vec<String>,
    horizontal_walls_below: Vec<(usize, usize)>,
    vertical_walls_to_right_of: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut visited = vec![];
    let mut from_spaces = vec![(0usize, 0usize)];
    let mut to_spaces = vec![];
    while !from_spaces.is_empty() {
        visited.extend_from_slice(from_spaces.as_slice());
        for &s in from_spaces.iter() {
            for d in DIRS {
                if let Some(n) = next_pos(grid, s, d) {
                    // check not already added
                    let already_visited = visited.contains(&n);

                    // check no wall blocking us
                    let is_blocked = match d {
                        UP => horizontal_walls_below.contains(&n),
                        DOWN => horizontal_walls_below.contains(&s),
                        LEFT => vertical_walls_to_right_of.contains(&n),
                        RIGHT => vertical_walls_to_right_of.contains(&s),
                    };

                    if !already_visited && !is_blocked {
                        to_spaces.push(n);
                    }
                }
            }
        }
        to_spaces.sort();
        to_spaces.dedup();
        from_spaces.clear();
        swap(&mut from_spaces, &mut to_spaces);
    }
    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    const PART1: &'static str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";

    #[test]
    fn problem1() {
        let answer = solve(&PART1.lines().map(|s| s.to_owned()).collect(), false).unwrap();
        assert_eq!(answer, 4);
    }

    const PART2: &'static str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn problem2() {
        let answer = solve(&PART2.lines().map(|s| s.to_owned()).collect(), true).unwrap();
        assert_eq!(answer, 10);
    }
}
