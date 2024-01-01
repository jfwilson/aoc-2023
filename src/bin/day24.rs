use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use nalgebra::{matrix, vector, SMatrix};
use num_bigint::BigInt;
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

    println!(
        "problem1 = {}",
        problem1_solution(&lines, 200000000000000, 400000000000000)?
    );
    println!("problem2 = {}", problem2_solution(&lines)?);
    Ok(())
}

fn problem1_solution(input: &Vec<String>, min: isize, max: isize) -> Result<usize> {
    let parsed = parse_input(input)?;

    let crossover_count = parsed
        .iter()
        .tuple_combinations()
        .filter(|&(&l, &r)| {
            // crossover point is where    l.0 + t * l.1 = r.0 + u * r.1
            //  => u = (l.0 + t * l.1 - r.0)[x] / r.1[x] = (l.0 + t * l.1 - r.0)[y] / r.1[y]
            //  =>     (l.0 + t * l.1 - r.0)[x] * r.1[y] = (l.0 + t * l.1 - r.0)[y] * r.1[x]
            //   t * (l.1[x] * r.1[y] - l.1[y] * r.1[x]) = (l.0 - r.0)[y] * r.1[x] - (l.0 - r.0)[x] * r.1[y]

            let delta = array_sub(l.0, r.0);
            let mut l_numer = dot_product(delta, [-r.1[1], r.1[0], 0]);
            let mut r_numer = dot_product(delta, [-l.1[1], l.1[0], 0]);
            let mut denom = l.1[0] * r.1[1] - l.1[1] * r.1[0];
            if denom.is_negative() {
                l_numer = -l_numer;
                r_numer = -r_numer;
                denom = -denom;
            }
            let future_for_l = l_numer.is_positive();
            let future_for_r = r_numer.is_positive();
            let x = BigInt::from(denom) * l.0[0] + BigInt::from(l_numer) * l.1[0];
            let y = BigInt::from(denom) * l.0[1] + BigInt::from(l_numer) * l.1[1];
            let range = (BigInt::from(denom) * min)..=(BigInt::from(denom) * max);

            future_for_l && future_for_r && range.contains(&x) && range.contains(&y)
        })
        .count();

    Ok(crossover_count)
}

fn problem2_solution(input: &Vec<String>) -> Result<isize> {
    let parsed = parse_input(input)?;

    // define   x = [      xo       ,       xv       ,  t_a, t_b, t_c ]
    //       f(x) = [x(t_a) - a(t_a), x(t_b) - b(t_b), x(t_c) - c(t_c)]
    // where x(t) = xo + t * xv
    // find x s.t. f(x) == 0

    let (&a, &b, &c) = parsed
        .iter()
        .take(3)
        .collect_tuple()
        .ok_or(anyhow!("Not enough input data"))?;

    let mut x: SMatrix<f64, 9, 1> = vector![0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0, 2.0, 3.0];

    for _ in 0..20 {
        let f_x: SMatrix<f64, 9, 1> = vector![
            x[0] + x[6] * x[3] - a.0[0] as f64 - x[6] * a.1[0] as f64,
            x[1] + x[6] * x[4] - a.0[1] as f64 - x[6] * a.1[1] as f64,
            x[2] + x[6] * x[5] - a.0[2] as f64 - x[6] * a.1[2] as f64,
            x[0] + x[7] * x[3] - b.0[0] as f64 - x[7] * b.1[0] as f64,
            x[1] + x[7] * x[4] - b.0[1] as f64 - x[7] * b.1[1] as f64,
            x[2] + x[7] * x[5] - b.0[2] as f64 - x[7] * b.1[2] as f64,
            x[0] + x[8] * x[3] - c.0[0] as f64 - x[8] * c.1[0] as f64,
            x[1] + x[8] * x[4] - c.0[1] as f64 - x[8] * c.1[1] as f64,
            x[2] + x[8] * x[5] - c.0[2] as f64 - x[8] * c.1[2] as f64,
        ];

        let mut f_derivative: SMatrix<f64, 9, 9> = matrix![
            1.0, 0.0, 0.0, x[6], 0.0, 0.0, x[3] - a.1[0] as f64, 0.0, 0.0;
            0.0, 1.0, 0.0, 0.0, x[6], 0.0, x[4] - a.1[1] as f64, 0.0, 0.0;
            0.0, 0.0, 1.0, 0.0, 0.0, x[6], x[5] - a.1[2] as f64, 0.0, 0.0;
            1.0, 0.0, 0.0, x[7], 0.0, 0.0, 0.0, x[3] - b.1[0] as f64, 0.0;
            0.0, 1.0, 0.0, 0.0, x[7], 0.0, 0.0, x[4] - b.1[1] as f64, 0.0;
            0.0, 0.0, 1.0, 0.0, 0.0, x[7], 0.0, x[5] - b.1[2] as f64, 0.0;
            1.0, 0.0, 0.0, x[8], 0.0, 0.0, 0.0, 0.0, x[3] - c.1[0] as f64;
            0.0, 1.0, 0.0, 0.0, x[8], 0.0, 0.0, 0.0, x[4] - c.1[1] as f64;
            0.0, 0.0, 1.0, 0.0, 0.0, x[8], 0.0, 0.0, x[5] - c.1[2] as f64;
        ];

        if !f_derivative.try_inverse_mut() {
            bail!("Could not invert matrix")
        }

        x -= f_derivative * f_x;
    }

    Ok((x[0] + x[1] + x[2]).round() as isize)
}

fn parse_input(input: &Vec<String>) -> Result<Vec<([isize; 3], [isize; 3])>> {
    input
        .iter()
        .map(|line| {
            let (p, v) = line.split_once(" @ ").ok_or(anyhow!("Invalid line"))?;
            Ok((parse_triple(p)?, parse_triple(v)?))
        })
        .collect::<Result<Vec<([isize; 3], [isize; 3])>>>()
}

fn parse_triple(p: &str) -> Result<[isize; 3]> {
    let ps = p
        .split(", ")
        .map(|n| n.parse::<isize>())
        .collect::<Result<Vec<isize>, ParseIntError>>()?;
    ps.try_into().map_err(|v| anyhow!("Incorrect size {:?}", v))
}

fn array_sub(mut lhs: [isize; 3], rhs: [isize; 3]) -> [isize; 3] {
    for (l, r) in lhs.iter_mut().zip(rhs) {
        *l -= r;
    }
    lhs
}

fn dot_product(lhs: [isize; 3], rhs: [isize; 3]) -> isize {
    lhs.into_iter().zip(rhs).map(|(l, r)| l * r).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &'static str = "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3";

    fn load_test_data() -> Vec<String> {
        INPUT.lines().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn problem1() {
        let answer = problem1_solution(&load_test_data(), 7, 27).unwrap();
        assert_eq!(answer, 2);
    }

    #[test]
    fn problem2() {
        let answer = problem2_solution(&load_test_data()).unwrap();
        assert_eq!(answer, 47);
    }
}
