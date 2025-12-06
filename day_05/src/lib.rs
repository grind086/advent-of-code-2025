use std::ops::RangeInclusive;

pub static INPUT: &str = include_str!("../input");

pub static TEST_INPUT: &str = concat!(
    "3-5\n", "10-14\n", "16-20\n", "12-18\n", "\n", "1\n", "5\n", "8\n", "11\n", "17\n", "32"
);

pub type FreshIngredientDb = Vec<RangeInclusive<i64>>;

pub fn parse_input(input: &str) -> (FreshIngredientDb, &str) {
    let (db_lines, ingredient_lines) = input.split_once("\n\n").unwrap();
    (parse_ingredient_db(db_lines), ingredient_lines)
}

fn parse_ingredient_db(lines: &str) -> FreshIngredientDb {
    lines
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            a.parse().unwrap()..=b.parse().unwrap()
        })
        .collect()
}

pub fn iter_fresh_ingredients(db: &FreshIngredientDb, lines: &str) -> impl Iterator<Item = i64> {
    iter_ingredients(lines).filter(|id| db.iter().any(|r| r.contains(id)))
}

fn iter_ingredients(lines: &str) -> impl Iterator<Item = i64> {
    lines.lines().map(|l| l.parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input_fresh_ids() {
        let (db, lines) = parse_input(TEST_INPUT);
        assert_eq!(
            iter_fresh_ingredients(&db, lines).collect::<Vec<_>>(),
            vec![5, 11, 17]
        );
    }

    #[test]
    fn part1_test_input_fresh_count() {
        let (db, lines) = parse_input(TEST_INPUT);
        assert_eq!(iter_fresh_ingredients(&db, lines).count(), 3);
    }
}
