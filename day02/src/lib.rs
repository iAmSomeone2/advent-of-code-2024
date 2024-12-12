use aoc_day::AoCDay;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Direction {
    Increasing,
    Decreasing,
}

#[derive(Default, Clone)]
struct Report {
    levels: Vec<i64>,
}

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, ()> {
        let levels: Vec<i64> = s
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        Ok(Report { levels })
    }
}

impl Report {
    fn levels_are_safe(levels: &[i64]) -> bool {
        let mut index = 0;
        let mut prev_direction: Option<Direction> = None;
        for pair in levels.windows(2) {
            let val0 = (pair[0], index);
            let val1 = (pair[1], index + 1);
            let diff = val1.0 - val0.0;

            let direction = if diff >= 0 {
                Direction::Increasing
            } else {
                Direction::Decreasing
            };

            if !(1..=3).contains(&diff.unsigned_abs()) {
                return false;
            }

            if let Some(prev_dir) = prev_direction {
                if direction != prev_dir {
                    return false;
                }
            }

            prev_direction = Some(direction);
            index += 1;
        }

        true
    }

    /// Checks if a report is "safe" per the instructions in Day 02 - Part 1
    fn is_safe(&self) -> bool {
        Report::levels_are_safe(&self.levels)
    }

    /// Checks if a report is "safe" per the instructions in Day 02 - Part 2
    ///
    /// A single "bad" level may be removed in this implementation
    fn is_safe_dampened(&self) -> bool {
        if Report::levels_are_safe(&self.levels) {
            return true;
        };

        let mut dampened_levels = Vec::with_capacity(self.levels.len() - 1);
        for i in 0..self.levels.len() {
            dampened_levels.clear();
            dampened_levels.extend_from_slice(&self.levels[..i]);
            dampened_levels.extend_from_slice(&self.levels[i + 1..]);

            if Report::levels_are_safe(&dampened_levels) {
                return true;
            }
        }

        false
    }
}

#[derive(Default, Clone)]
pub struct Day02 {
    reports: Vec<Report>,
}

impl Day02 {
    pub fn count_safe_reports(&self) -> usize {
        self.reports.iter().fold(
            0,
            |acc, report| {
                if report.is_safe() {
                    acc + 1
                } else {
                    acc
                }
            },
        )
    }

    pub fn count_safe_reports2(&self) -> usize {
        self.reports.iter().fold(0, |acc, report| {
            if report.is_safe_dampened() {
                acc + 1
            } else {
                acc
            }
        })
    }
}

impl AoCDay for Day02 {
    fn part1(&mut self) {
        let result = self.count_safe_reports();
        println!("Safe reports: {}", result);
    }

    fn part2(&mut self) {
        let result = self.count_safe_reports2();
        println!("Safe reports: {}", result);
    }

    fn load_input(&mut self, path: &Path) -> anyhow::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        self.reports = reader
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| line.parse().ok())
            .collect();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::LazyLock;

    static EXAMPLE_PATH: LazyLock<PathBuf> =
        LazyLock::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("example_input.txt"));

    #[test]
    fn report_is_safe() {
        let report = "7 6 4 2 1".parse::<Report>().unwrap();
        assert!(report.is_safe());

        let report = "1 3 2 4 5".parse::<Report>().unwrap();
        assert!(!report.is_safe());
    }

    #[test]
    fn count_safe_reports() {
        let mut day = Day02::default();
        day.load_input(&EXAMPLE_PATH).unwrap();

        let expected = 2;
        let actual = day.count_safe_reports();

        assert_eq!(expected, actual);
    }

    #[test]
    fn count_safe_reports2() {
        let mut day = Day02::default();
        day.load_input(&EXAMPLE_PATH).unwrap();

        let expected = 4;
        let actual = day.count_safe_reports2();

        assert_eq!(expected, actual);
    }
}
