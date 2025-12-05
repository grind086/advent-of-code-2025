use std::str::FromStr;

use anyhow::Error;

pub static INPUT: &str = include_str!("../input");

/// The state of a dial with values 0-99.
pub struct State(i32);

impl State {
    pub const fn starting_at(n: i32) -> Self {
        Self(((n % 100) + 100) % 100)
    }

    /// Applies the [`Instruction`], and returns a reference to the state.
    pub fn apply(&mut self, inst: Instruction) -> u16 {
        match inst {
            Instruction::Left(n) => {
                let prev = self.0;
                self.0 = (((self.0 - i32::from(n)) % 100) + 100) % 100;
                n / 100 + (prev < self.0) as u16
            }
            Instruction::Right(n) => {
                let prev = self.0;
                self.0 = (self.0 + i32::from(n)) % 100;
                n / 100 + (prev > self.0) as u16
            }
        }
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

/// A dial rotation instruction.
pub enum Instruction {
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
