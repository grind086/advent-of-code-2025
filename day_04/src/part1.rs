use day_04::*;

fn main() {
    let n = Grid::from_input(INPUT).count_accessible_rolls(4);
    println!("{n}")
}
