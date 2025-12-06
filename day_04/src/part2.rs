use day_04::*;

fn main() {
    let n = Grid::from_input(INPUT).remove_all_accessible_rolls(4);
    println!("{n}")
}
