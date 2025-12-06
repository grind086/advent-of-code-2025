pub static INPUT: &str = include_str!("../input");

pub static TEST_INPUT: &str = concat!(
    "..@@.@@@@.\n",
    "@@@.@.@.@@\n",
    "@@@@@.@.@@\n",
    "@.@@@@..@.\n",
    "@@.@@@@.@@\n",
    ".@@@@@@@.@\n",
    ".@.@.@.@@@\n",
    "@.@@@.@@@@\n",
    ".@@@@@@@@.\n",
    "@.@.@@@.@.\n",
);

pub struct Grid<T> {
    width: isize,
    values: Vec<T>,
}

impl Grid<bool> {
    pub fn from_input(input: &str) -> Self {
        let mut width = 0;
        let mut values = Vec::new();
        for line in input.lines() {
            let bytes = line.trim().as_bytes();
            if width == 0 {
                width = bytes.len();
            } else {
                assert_eq!(bytes.len(), width);
            }

            values.extend(bytes.iter().map(|b| match b {
                b'@' => true,
                b'.' => false,
                _ => panic!("invalid input"),
            }));
        }

        Self {
            width: width as _,
            values,
        }
    }

    pub fn iter_accessible_rolls(
        &self,
        max_neighbors: usize,
    ) -> impl Iterator<Item = (isize, isize)> {
        self.iter_coords()
            .filter(|&(x, y)| *self.get(x, y).unwrap_or(&false))
            .filter(move |&(x, y)| self.count_neighbors_eq(x, y, true) < max_neighbors)
    }

    pub fn count_accessible_rolls(&self, max_neighbors: usize) -> usize {
        self.iter_accessible_rolls(max_neighbors).count()
    }
}

impl<T> Grid<T> {
    pub fn get(&self, x: isize, y: isize) -> Option<&T> {
        (x >= 0 && y >= 0 && x < self.width)
            .then(|| x + y * self.width)
            .and_then(|i| self.values.get(i as usize))
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = (isize, isize)> {
        (0..self.values.len() as isize / self.width)
            .flat_map(move |y| (0..self.width).map(move |x| (x, y)))
    }

    pub fn iter_neighbors(&self, x: isize, y: isize) -> impl Iterator<Item = &T> {
        (-1..=1)
            .flat_map(|dy| (-1..=1).map(move |dx| (dx, dy)))
            .filter(|(dx, dy)| *dx != 0 || *dy != 0)
            .filter_map(move |(dx, dy)| self.get(x + dx, y + dy))
    }

    pub fn count_neighbors_eq(&self, x: isize, y: isize, val: T) -> usize
    where
        T: Eq,
    {
        self.iter_neighbors(x, y).filter(|v| **v == val).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_input_accessible_rolls() {
        let grid = Grid::from_input(TEST_INPUT);
        assert_eq!(grid.count_accessible_rolls(4), 13);
    }

    #[test]
    fn part1_test_input_roll_1() {
        let grid = Grid::from_input(TEST_INPUT);
        assert_eq!(grid.count_neighbors_eq(2, 0, true), 3);
    }
}
