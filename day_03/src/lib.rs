pub static INPUT: &str = include_str!("../input");

pub static TEST_INPUT: &str = concat!(
    "987654321111111\n",
    "811111111111119\n",
    "234234234234278\n",
    "818181911112111\n",
);

pub fn max_joltage_sum(input: &str, size: usize) -> u64 {
    iter_bank_max_joltages(input, size).sum()
}

pub fn iter_bank_max_joltages(input: &str, size: usize) -> impl Iterator<Item = u64> {
    input
        .lines()
        .map(move |bank| bank_max_joltage(bank.trim().as_bytes(), size))
}

pub fn bank_max_joltage(bank: &[u8], size: usize) -> u64 {
    fn inner(bank: &[u8], size: usize, clamp_max: Option<usize>) -> u64 {
        if size == 0 {
            return 0;
        }

        let scan_max_index = clamp_max.unwrap_or(bank.len());
        let (i, i_val) = scan_max(&bank[0..scan_max_index]);
        if i + size <= bank.len() {
            (i_val - b'0') as u64 * 10u64.pow((size - 1) as _)
                + inner(&bank[i + 1..], size - 1, None)
        } else {
            inner(bank, size, Some(i))
        }
    }

    inner(bank, size, None)
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
            iter_bank_max_joltages(TEST_INPUT, 2).collect::<Vec<_>>(),
            vec![98, 89, 78, 92]
        );
    }

    #[test]
    fn part1_test_input_max_joltage() {
        assert_eq!(max_joltage_sum(TEST_INPUT, 2), 357);
    }

    #[test]
    fn max_digit_can_repeat() {
        assert_eq!(bank_max_joltage(b"124444266221211", 2), 66);
    }

    #[test]
    fn part2_test_input_max_joltages() {
        assert_eq!(
            iter_bank_max_joltages(TEST_INPUT, 12).collect::<Vec<_>>(),
            vec![987654321111, 811111111119, 434234234278, 888911112111]
        );
    }

    #[test]
    fn part2_test_input_max_joltage() {
        assert_eq!(max_joltage_sum(TEST_INPUT, 12), 3121910778619);
    }
}
