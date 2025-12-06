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

pub fn count_fresh_ids(db: FreshIngredientDb) -> usize {
    optimize_ingredient_db(db)
        .into_iter()
        .map(|r| r.end() - r.start() + 1)
        .map(|n| n as usize)
        .sum()
}

/// Turns the provided overlapping ranges into a sorted list of non-overlapping ranges
fn optimize_ingredient_db(mut db: FreshIngredientDb) -> FreshIngredientDb {
    db.sort_by(|a, b| a.start().cmp(b.start()).then(a.end().cmp(b.end())));

    let old_len = db.len();
    let mut new_db = vec![];

    let mut db_iter = db.into_iter();
    let (mut cur_min, mut cur_max) = {
        let v = db_iter.next().unwrap();
        (*v.start(), *v.end())
    };
    for r in db_iter {
        if *r.start() <= cur_max {
            cur_max = cur_max.max(*r.end());
        } else {
            new_db.push(cur_min..=cur_max);
            cur_min = *r.start();
            cur_max = *r.end();
        }
    }
    new_db.push(cur_min..=cur_max);

    println!("Optimize: {old_len} -> {}", new_db.len());

    new_db
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

    #[test]
    fn part2_test_input_total_fresh_id_count() {
        let (db, _) = parse_input(TEST_INPUT);
        assert_eq!(count_fresh_ids(db), 14);
    }
}
