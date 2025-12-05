use std::str::FromStr;

use anyhow::Error;

static INPUT: &str = include_str!("../input");

fn main() {
    let mut state = State::starting_at(50);
    let mut count = 0;
    for line in INPUT.lines() {
        let inst = Instruction::from_str(line).unwrap();
        if state.apply(inst).is_zero() {
            count += 1;
        }
    }
    println!("{count}");
}

/// The state of a dial with values 0-99.
struct State(i32);

impl State {
    pub const fn starting_at(n: i32) -> Self {
        Self(((n % 100) + 100) % 100)
    }

    /// Applies the [`Instruction`], and returns a reference to the state.
    pub fn apply(&mut self, inst: Instruction) -> &Self {
        self.0 = match inst {
            Instruction::Left(n) => (((self.0 - i32::from(n)) % 100) + 100) % 100,
            Instruction::Right(n) => (self.0 + i32::from(n)) % 100,
        };
        self
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

/// A dial rotation instruction.
enum Instruction {
    /// Rotate CCW (to lower numbers).
    Left(u16),
    /// Rotate CW (to higher numbers).
    Right(u16),
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.as_bytes()[0] {
            b'L' => Ok(s[1..].parse().map(Self::Left)?),
            b'R' => Ok(s[1..].parse().map(Self::Right)?),
            _ => Err(Error::msg("malformed instruction")),
        }
    }
}
