use aoc_day::AoCDay;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

#[derive(Default)]
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
    /// Checks if a report is "safe" per the instructions in Day 02 - Part 1
    fn is_safe(&self) -> bool {
        let diffs = self
            .levels
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect::<Vec<_>>();

        let all_decreasing = diffs.iter().all(|val| *val < 0);
        let all_increasing = diffs.iter().all(|val| *val > 0);

        if !all_decreasing && !all_increasing {
            return false;
        }

        diffs
            .iter()
            .map(|val| val.unsigned_abs())
            .all(|val| (1..=3).contains(&val))
    }

    /// Checks if a report is "safe" per the instructions in Day 02 - Part 2
    ///
    /// A single "bad" level may be removed in this implementation
    fn is_safe_dampened(&self) -> bool {
        if self.is_safe() {
            return true;
        }

        for i in 0..self.levels.len() {
            let mut dampened_levels = Vec::with_capacity(self.levels.len() - 1);
            // Clone first slice
            dampened_levels.extend_from_slice(&self.levels[..i]);
            // Clone second slice
            dampened_levels.extend_from_slice(&self.levels[i + 1..]);

            assert_eq!(dampened_levels.len(), self.levels.len() - 1);

            let diffs = dampened_levels
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect::<Vec<_>>();

            let all_decreasing = diffs.iter().all(|val| *val < 0);
            let all_increasing = diffs.iter().all(|val| *val > 0);

            if !all_decreasing && !all_increasing {
                continue;
            }

            let is_safe = diffs
                .iter()
                .map(|val| val.unsigned_abs())
                .all(|val| (1..=3).contains(&val));

            if is_safe {
                return true;
            }
        }

        false
    }
}

#[derive(Default)]
pub struct Day02 {
    reports: Vec<Report>,
}

impl Day02 {
    fn count_safe_reports(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe())
            .count()
    }

    fn count_safe_reports2(&self) -> usize {
        self.reports
            .iter()
            .filter(|report| report.is_safe_dampened())
            .count()
    }
}

impl AoCDay for Day02 {
    fn part1(&self) {
        let result = self.count_safe_reports();
        println!("Safe reports: {}", result);
    }

    fn part2(&self) {
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
