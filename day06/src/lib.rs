use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::North),
            '>' => Ok(Direction::East),
            'v' => Ok(Direction::South),
            '<' => Ok(Direction::West),
            _ => Err(format!("{value} is not a valid direction")),
        }
    }
}

struct Guard {
    current_position: (usize, usize),
    direction: Direction,
    traveled_distance: u64,
    distinct_positions: HashSet<(usize, usize)>,
}

impl Guard {
    fn new(position: (usize, usize), direction: Direction) -> Self {
        Self {
            current_position: position,
            direction,
            traveled_distance: 0,
            distinct_positions: HashSet::new(),
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }

    fn move_to(&mut self, obstacle: &Obstacle) {}

    /// Casts a "ray" from the [Guard]'s current position and [Direction].
    ///
    /// # Return
    ///
    /// If the "ray" hits an [Obstacle] before exiting the [PatrolArea], then the position of the
    /// hit is returned.
    fn cast_ray(&self, obstacles: &[Obstacle]) -> Option<Obstacle> {
        let (cur_x, cur_y) = self.current_position;

        match self.direction {
            Direction::North => obstacles
                .iter()
                .filter(|obs| obs.position.1 < cur_y)
                .max_by_key(|obs| obs.position.1)
                .cloned(),
            Direction::East => obstacles
                .iter()
                .filter(|obs| obs.position.0 > cur_x)
                .min_by_key(|obs| obs.position.0)
                .cloned(),
            Direction::South => obstacles
                .iter()
                .filter(|obs| obs.position.1 > cur_y)
                .min_by_key(|obs| obs.position.1)
                .cloned(),
            Direction::West => obstacles
                .iter()
                .filter(|obs| obs.position.0 < cur_x)
                .max_by_key(|obs| obs.position.0)
                .cloned(),
        }
    }

    /// Patrols forward until encountering an obstacle or exiting the patrol area
    ///
    /// # Return
    ///
    /// `true` is returned if the [Guard] encountered an [Obstacle] and is still in the grid at the
    /// end of this move; `false`, otherwise.
    fn patrol(&mut self, obstacles: &[Obstacle]) -> bool {
        todo!()
    }
}

/*
   NOTE:
   The patrol area is mostly empty, and we don't care about empty spaces. Checking them is a waste
   of time. Instead, only the guard and obstacles need to be tracked.

   When using raycasting for collision detection, filter the list of obstacles down to only what is
   directly in front of the guard. Then, find the closest obstacle for the given direction possibly
   using something like `min_by`.
*/

/// An object in the [PatrolArea] which the [Guard] may collide with
#[derive(Copy, Clone)]
struct Obstacle {
    position: (usize, usize),
}

impl Obstacle {
    fn new(x: usize, y: usize) -> Self {
        Self { position: (x, y) }
    }
}

struct PatrolArea {
    guard: Guard,
    obstacles: Vec<Obstacle>,
    width: usize,
    height: usize,
}

impl FromStr for PatrolArea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guard_dir: Option<Direction> = None;
        let mut guard_pos: Option<(usize, usize)> = None;
        let mut width = 0;
        let mut height = 0;
        let mut obstacles = Vec::new();

        for (y, line) in s.trim().lines().enumerate() {
            height += 1;
            if width == 0 {
                width = line.len();
            }

            for (x, c) in line.chars().enumerate() {
                if guard_pos.is_none() && guard_dir.is_none() {
                    if let Ok(direction) = Direction::try_from(c) {
                        guard_dir = Some(direction);
                        guard_pos = Some((x, y));
                    }
                }

                if c == '#' {
                    obstacles.push(Obstacle::new(x, y));
                }
            }
        }

        if guard_dir.is_none() || guard_pos.is_none() {
            return Err("Could not locate guard".to_string());
        }

        let guard = Guard::new(guard_pos.unwrap(), guard_dir.unwrap());

        Ok(Self {
            guard,
            obstacles,
            width,
            height,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../example_input.txt");

    #[test]
    fn parse_patrol_area() {
        let patrol_area = PatrolArea::from_str(INPUT);

        assert!(patrol_area.is_ok());
    }
}
