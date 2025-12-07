use day_07::*;

fn main() {
    let mut lines = INPUT.lines();

    let init = lines.next().unwrap();
    let mut state: Vec<u64> = init
        .trim()
        .as_bytes()
        .iter()
        .map(|b| match *b {
            b'S' => 1,
            b'.' => 0,
            _ => panic!("Only S and . in first line"),
        })
        .collect();

    let mut splits = 0;
    for line in lines {
        for (i, b) in line.trim().as_bytes().iter().enumerate() {
            match *b {
                b'.' => {}
                b'^' => {
                    if state[i] != 0 {
                        state[i + 1] += state[i];
                        state[i - 1] += state[i];
                        state[i] = 0;
                        splits += 1;
                    }
                }
                _ => unreachable!(),
            }
        }
    }

    println!("{splits}");
    println!("{}", state.iter().copied().sum::<u64>());
}
