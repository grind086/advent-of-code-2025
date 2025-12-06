use std::str::FromStr;

use day_01::*;

fn main() {
    let mut state = State::starting_at(50);
    let mut count = 0;
    for line in INPUT.lines() {
        let inst = Instruction::from_str(line).unwrap();
        count += state.apply(inst);
        if state.is_zero() {
            count += 1;
        }
    }
    println!("{count}");
}
