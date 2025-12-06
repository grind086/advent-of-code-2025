pub static INPUT: &str = include_str!("../input");

pub static TEST_INPUT: &str = concat!(
    "987654321111111\n",
    "811111111111119\n",
    "234234234234278\n",
    "818181911112111\n",
);

pub fn max_joltage_sum(input: &str) -> u32 {
    iter_bank_max_joltages(input).sum()
}

pub fn iter_bank_max_joltages(input: &str) -> impl Iterator<Item = u32> {
    input.lines().map(bank_max_joltage)
}

pub fn bank_max_joltage(bank: &str) -> u32 {
    let bank = bank.trim().as_bytes();
    let (i, i_val) = scan_max(bank);
    let (a, b) = if i == bank.len() - 1 {
        let (_, j_val) = scan_max(&bank[0..i]);
        (j_val, i_val)
    } else {
        let (_, j_val) = scan_max(&bank[i + 1..]);
        (i_val, j_val)
    };
    10 * (a - b'0') as u32 + (b - b'0') as u32
}

fn scan_max(bytes: &[u8]) -> (usize, u8) {
    // Note - `iter().max_by_key()` returns the *last* index of the maximum value
    let mut i = 0;
    let mut max = bytes[0];
    for (j, &val) in bytes.iter().enumerate() {
        if val > max {
            i = j;
            max = val;
        }
    }
    (i, max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input_max_joltages() {
        assert_eq!(
            iter_bank_max_joltages(TEST_INPUT).collect::<Vec<_>>(),
            vec![98, 89, 78, 92]
        );
    }

    #[test]
    fn part1_test_input_max_joltage() {
        assert_eq!(max_joltage_sum(TEST_INPUT), 357);
    }

    #[test]
    fn max_digit_can_repeat() {
        assert_eq!(bank_max_joltage("124444266221211"), 66);
    }
}
