use std::ops::RangeInclusive;

pub static INPUT: &str = include_str!("../input");

pub static TEST_INPUT: &str = concat!(
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,",
    "1698522-1698528,446443-446449,38593856-38593862,565653-565659,",
    "824824821-824824827,2121212118-2121212124",
);

pub fn count_invalid_ids(input: &str) -> usize {
    iter_invalid_ids(input).count()
}

pub fn sum_invalid_ids(input: &str) -> u64 {
    iter_invalid_ids(input).sum()
}

pub fn iter_invalid_ids(input: &str) -> impl Iterator<Item = u64> {
    iter_id_ranges(input).flat_map(|range| range.filter(is_invalid_id))
}

pub fn iter_id_ranges(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    input.trim().split(',').map(|s| {
        let (a, b) = s.split_once('-').unwrap();
        a.parse().unwrap()..=b.parse().unwrap()
    })
}

pub fn is_invalid_id(n: &u64) -> bool {
    let s = n.to_string();
    s.len().is_multiple_of(2) && s[0..s.len() / 2] == s[s.len() / 2..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_invalid_count() {
        assert_eq!(count_invalid_ids(TEST_INPUT), 8);
    }

    #[test]
    fn test_input_invalid_sum() {
        assert_eq!(sum_invalid_ids(TEST_INPUT), 1227775554);
    }

    #[test]
    fn day_01_result() {
        assert_eq!(sum_invalid_ids(INPUT), 40055209690);
    }
}
