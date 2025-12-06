use day_06::*;

fn main() {
    let total = input_cols(INPUT)
        .into_iter()
        .map(parse_problem)
        .sum::<i64>();
    println!("{total}");
}
