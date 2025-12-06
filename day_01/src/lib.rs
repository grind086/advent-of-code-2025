use std::{cmp::Ordering, str::FromStr};

use anyhow::Error;

pub static INPUT: &str = include_str!("../input");

pub static TEST_INPUT: &str = concat!(
    "L68\n", "L30\n", "R48\n", "L5\n", "R60\n", "L55\n", "L1\n", "L99\n", "R14\n", "L82\n",
);

/// The state of a dial with values 0-99.
pub struct State(i32);

impl State {
    pub const fn starting_at(n: i32) -> Self {
        Self(((n % 100) + 100) % 100)
    }

    pub fn get(&self) -> i32 {
        self.0
    }

    /// Applies the [`Instruction`], and returns the number of times the dial
    /// touched zero while rotating.
    pub fn apply(&mut self, inst: Instruction) -> i32 {
        match inst {
            Instruction::Left(n) => {
                let k = n / 100;
                let m = n - 100 * k;
                let v = self.0 - m;
                if v < 0 {
                    let was_zero = self.0 == 0;
                    self.0 = 100 + v;
                    if was_zero { k } else { k + 1 }
                } else {
                    self.0 = v;
                    k
                }
            }
            Instruction::Right(n) => {
                let k = n / 100;
                let m = n - 100 * k;
                let v = self.0 + m;
                match v.cmp(&100) {
                    Ordering::Less => {
                        self.0 = v;
                        k
                    }
                    Ordering::Equal => {
                        self.0 = 0;
                        k
                    }
                    Ordering::Greater => {
                        self.0 = v - 100;
                        k + 1
                    }
                }
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
    Left(i32),
    /// Rotate CW (to higher numbers).
    Right(i32),
}

impl FromStr for Instruction {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = s[1..].parse()?;
        match s.as_bytes()[0] {
            b'L' => Ok(Self::Left(n)),
            b'R' => Ok(Self::Right(n)),
            _ => Err(Error::msg("malformed instruction")),
        }
    }
}
