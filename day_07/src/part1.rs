use day_07::*;

fn main() {
    let mut lines = INPUT.lines();

    let init = lines.next().unwrap();
    let mut state: Vec<bool> = init
        .trim()
        .as_bytes()
        .iter()
        .map(|b| match *b {
            b'S' => true,
            b'.' => false,
            _ => panic!("Only S and . in first line"),
        })
        .collect();

    let mut splits = 0;
    for line in lines {
        for (i, b) in line.trim().as_bytes().iter().enumerate() {
            match *b {
                b'.' => {}
                b'^' => {
                    if state[i] {
                        state[i] = false;
                        state[i + 1] = true;
                        state[i - 1] = true;
                        splits += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    println!("{splits}");
}
