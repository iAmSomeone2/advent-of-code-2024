use std::hash::Hash;
use std::str::FromStr;

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Direction {
    #[default]
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Hash)]
struct CrosswordMatch {
    start_position: (usize, usize),
    direction: Direction,
}

// impl Hash for CrosswordMatch {
//     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
//         self.positions
//             .iter()
//             .for_each(|pos| state.write_usize(pos.0 ^ pos.1));
//     }
// }

struct Crossword {
    cells: Vec<char>,
    width: usize,
    height: usize,
}

impl FromStr for Crossword {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let cells = s
            .lines()
            .flat_map(|line| {
                if width == 0 {
                    width = line.len();
                }
                height += 1;
                line.chars()
            })
            .collect();

        Ok(Crossword::new(cells, width, height))
    }
}

impl Crossword {
    const XMAS: &'static str = "XMAS";
    const SAMX: &'static str = "SAMX";

    fn new(cells: Vec<char>, width: usize, height: usize) -> Self {
        Self {
            cells,
            width,
            height,
        }
    }

    fn read_at(&self, x: usize, y: usize) -> Option<char> {
        self.cells.get(x + (y * self.width)).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::{DefaultHasher, Hasher};

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

    #[test]
    fn hash_match() {
        let cross_match = CrosswordMatch::default();
        let mut hasher = DefaultHasher::new();
        cross_match.hash(&mut hasher);

        let hash = hasher.finish();
        assert_ne!(hash, 0);
    }
}
