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
    let mut cur_width = 0;
    let mut col_widths = Vec::new();
    for &b in input.lines().last().unwrap().as_bytes() {
        if b != b' ' && !(cur_width == 0 && col_widths.is_empty()) {
            col_widths.push(cur_width);
            cur_width = 0;
        }
        cur_width += 1;
    }
    col_widths.push(cur_width);

    let mut cols = vec![Vec::new(); col_widths.len()];
    for mut row in input.lines() {
        for (c, &width) in col_widths.iter().enumerate() {
            let d = if c < cols.len() - 1 { 1 } else { 0 };
            cols[c].push(&row[..width - d]);
            row = &row[width..];
        }
    }

    cols
}

pub fn solve_problem(problem: Vec<&str>) -> i64 {
    let (op, mut tot): (fn(i64, i64) -> i64, i64) = match problem.last().unwrap().as_bytes()[0] {
        b'+' => (<i64 as ops::Add>::add, 0),
        b'*' => (<i64 as ops::Mul>::mul, 1),
        s => panic!("invalid op {:?}", s as char),
    };

    let mut buf = Vec::new();
    for c in (0..problem[0].len()).rev() {
        buf.clear();
        for r in 0..problem.len() - 1 {
            let byte = problem[r].as_bytes()[c];
            if byte.is_ascii_digit() {
                buf.push(byte);
            }
        }

        let val: i64 = str::from_utf8(&buf).unwrap().parse().unwrap();
        tot = op(tot, val);
    }

    tot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2_test_input_solutions() {
        let solns = input_cols(TEST_INPUT)
            .into_iter()
            .map(solve_problem)
            .collect::<Vec<_>>();
        assert_eq!(solns, vec![8544, 625, 3253600, 1058])
    }

    #[test]
    fn part2_test_input_checksum() {
        let total = input_cols(TEST_INPUT)
            .into_iter()
            .map(solve_problem)
            .sum::<i64>();
        assert_eq!(total, 3263827)
    }
}
