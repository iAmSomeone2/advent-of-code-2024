use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Match {
    positions: [(usize, usize); 4],
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

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
