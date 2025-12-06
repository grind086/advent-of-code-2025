use std::ops;

use day_06::*;

fn main() {
    let total = input_cols(INPUT)
        .into_iter()
        .map(solve_problem)
        .sum::<i64>();
    println!("{total}");
}

pub fn input_cols(input: &str) -> Vec<Vec<&str>> {
    let mut cols = Vec::new();
    for row in input.lines() {
        for (i, col) in row.split_whitespace().enumerate() {
            if cols.len() < i + 1 {
                cols.push(Vec::new());
            }
            cols[i].push(col);
        }
    }
    cols
}

pub fn solve_problem(problem: Vec<&str>) -> i64 {
    let (op, init): (fn(i64, i64) -> i64, i64) = match *problem.last().unwrap() {
        "+" => (<i64 as ops::Add>::add, 0),
        "*" => (<i64 as ops::Mul>::mul, 1),
        s => panic!("invalid op {s:?}"),
    };

    problem
        .iter()
        .take(problem.len() - 1)
        .map(|s| s.parse::<i64>().unwrap())
        .fold(init, |tot, val| op(tot, val))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input_solutions() {
        let solns = input_cols(TEST_INPUT)
            .into_iter()
            .map(solve_problem)
            .collect::<Vec<_>>();
        assert_eq!(solns, vec![33210, 490, 4243455, 401])
    }

    #[test]
    fn part1_test_input_checksum() {
        let total = input_cols(TEST_INPUT)
            .into_iter()
            .map(solve_problem)
            .sum::<i64>();
        assert_eq!(total, 4277556)
    }
}
