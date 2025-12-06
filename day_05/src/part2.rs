use day_05::*;

fn main() {
    let (db, _) = parse_input(INPUT);
    println!("{}", count_fresh_ids(db));
}
