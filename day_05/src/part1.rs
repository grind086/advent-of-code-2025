use day_05::*;

fn main() {
    let (db, lines) = parse_input(INPUT);
    println!("{}", iter_fresh_ingredients(&db, lines).count());
}
