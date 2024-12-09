use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
struct Match {
    positions: [(usize, usize); 4],
}

impl Hash for Match {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.positions
            .iter()
            .for_each(|pos| state.write_usize(pos.0 ^ pos.1));
    }
}

struct Crossword {
    cells: Vec<Vec<char>>,
}

impl FromStr for Crossword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s.lines().map(|line| line.chars().collect()).collect();

        Ok(Crossword { cells })
    }
}

impl Crossword {
    const XMAS: &'static str = "XMAS";
    const SAMX: &'static str = "SAMX";
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::DefaultHasher;

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

    #[test]
    fn hash_match() {
        let mut cross_match = Match::default();
        let mut hasher = DefaultHasher::new();
        cross_match.hash(&mut hasher);

        let hash = hasher.finish();
        assert_ne!(hash, 0);
    }
}
