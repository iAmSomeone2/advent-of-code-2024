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
}

impl AoCDay for Day02 {
    fn part1(&self) {
        let result = self.count_safe_reports();
        println!("Safe reports: {}", result);
    }

    fn part2(&self) {
        todo!()
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
}
