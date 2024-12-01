use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use runner::AoCDay;

#[derive(Default)]
pub struct Day01 {
    left_ids: Vec<i64>,
    right_ids: Vec<i64>,
}

impl Day01 {
    fn parse_input_line(line: &str) -> (i64, i64) {
        let ids: Vec<i64> = line.split_whitespace()
            .filter(|s| !s.is_empty())
            .take(2)
            .filter_map(|s| s.parse().ok())
            .collect();
        (ids[0], ids[1])
    }

    fn total_distance(&self) -> i64 {
        let mut left = self.left_ids.clone();
        let mut right = self.right_ids.clone();
        left.sort_unstable();
        right.sort_unstable();

        left.iter().enumerate().fold(0, |acc, (i, left_id)| {
            let right_id = right[i];
            acc + (left_id - right_id).abs()
        })
    }
}

impl AoCDay for Day01 {
    fn part1(&self) {
        let result = self.total_distance();
        println!("Result:\t{}", result);
    }

    fn part2(&self) {
        todo!()
    }

    fn load_input(&mut self, path: &Path) -> anyhow::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let (left, right) = reader.lines()
            .map_while(Result::ok)
            .map(|line| Self::parse_input_line(&line))
            .collect();

        self.left_ids = left;
        self.right_ids = right;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::LazyLock;
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

    static EXAMPLE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("example_input.txt")
    });

    #[test]
    fn parse_input_line_test() {
        let line = EXAMPLE_INPUT.lines().next().unwrap();
        let expected = (3, 4);
        let actual = Day01::parse_input_line(line);

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_input_test() {
        let mut day = Day01::default();

        let expected = (vec![3,4,2,1,3,3], vec![4,3,5,3,9,3]);
        day.load_input(&EXAMPLE_PATH).unwrap();

        assert_eq!(expected.0, day.left_ids);
        assert_eq!(expected.1, day.right_ids);
    }

    #[test]
    fn total_distance_test() {
        let mut day = Day01::default();
        day.load_input(&EXAMPLE_PATH).unwrap();

        let expected = 11;
        let actual = day.total_distance();

        assert_eq!(expected, actual);
    }
}
