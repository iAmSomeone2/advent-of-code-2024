use aoc_day::AoCDay;
use std::collections::HashSet;
use std::fmt::Display;
use std::path::Path;
use std::str::FromStr;

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
enum Direction {
    #[default]
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

#[derive(Default)]
struct Guard {
    current_position: Position,
    direction: Direction,
    traveled_distance: u64,
    distinct_positions: HashSet<Position>,
}

impl Guard {
    fn new(position: Position, direction: Direction) -> Self {
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

    fn move_to(&mut self, obstacle: &Option<Obstacle>, max_width: usize, max_height: usize) {
        let (cur_x, cur_y) = self.current_position;
        let (final_position, visited_cells): (Position, Vec<Position>) = match self.direction {
            Direction::North => {
                let target_pos = if let Some(obstacle) = obstacle {
                    (cur_x, obstacle.position.1 + 1)
                } else {
                    (cur_x, 0)
                };
                let visited_cells = (target_pos.1..self.current_position.1)
                    .map(|y| (cur_x, y))
                    .collect();

                (target_pos, visited_cells)
            }
            Direction::East => {
                let target_pos = if let Some(obstacle) = obstacle {
                    (obstacle.position.0 - 1, cur_y)
                } else {
                    (max_width, cur_y)
                };
                let visited_cells = (self.current_position.0 + 1..=target_pos.0)
                    .map(|x| (x, cur_y))
                    .collect();

                (target_pos, visited_cells)
            }
            Direction::South => {
                let target_pos = if let Some(obstacle) = obstacle {
                    (cur_x, obstacle.position.1 - 1)
                } else {
                    (cur_x, max_height)
                };

                let visited_cells = (self.current_position.1 + 1..=target_pos.1)
                    .map(|y| (cur_x, y))
                    .collect();

                (target_pos, visited_cells)
            }
            Direction::West => {
                let target_pos = if let Some(obstacle) = obstacle {
                    (obstacle.position.0 + 1, cur_y)
                } else {
                    (0, cur_y)
                };
                let visited_cells = (target_pos.0..self.current_position.0)
                    .map(|x| (x, cur_y))
                    .collect();

                (target_pos, visited_cells)
            }
        };

        let distance_traveled = visited_cells.len();
        for cell in visited_cells {
            self.distinct_positions.insert(cell);
        }
        self.current_position = final_position;
        self.traveled_distance += distance_traveled as u64;
    }

    /// Casts a "ray" from the [Guard]'s current position and [Direction].
    ///
    /// # Return
    ///
    /// If the "ray" hits an [Obstacle] before exiting the [PatrolArea], then the position of the
    /// hit is returned.
    fn cast_ray(&self, obstacles: &[Obstacle]) -> Option<Obstacle> {
        let (cur_x, cur_y) = self.current_position;
        let obstacle_iter = obstacles.iter();

        let hit = match self.direction {
            Direction::North => obstacle_iter
                .filter(|obs| obs.position.1 < cur_y && obs.position.0 == cur_x)
                .max_by_key(|obs| obs.position.1),
            Direction::East => obstacle_iter
                .filter(|obs| obs.position.0 > cur_x && obs.position.1 == cur_y)
                .min_by_key(|obs| obs.position.0),
            Direction::South => obstacle_iter
                .filter(|obs| obs.position.1 > cur_y && obs.position.0 == cur_x)
                .min_by_key(|obs| obs.position.1),
            Direction::West => obstacle_iter
                .filter(|obs| obs.position.0 < cur_x && obs.position.1 == cur_y)
                .max_by_key(|obs| obs.position.0),
        };

        hit.cloned()
    }

    /// Patrols forward until encountering an obstacle or exiting the patrol area
    ///
    /// # Return
    ///
    /// `true` is returned if the [Guard] encountered an [Obstacle] and is still in the grid at the
    /// end of this move; `false`, otherwise.
    fn patrol(&mut self, obstacles: &[Obstacle], max_width: usize, max_height: usize) -> bool {
        let obstacle = self.cast_ray(obstacles);
        self.move_to(&obstacle, max_width, max_height);

        if obstacle.is_some() {
            self.turn_right();
            true
        } else {
            false
        }
    }
}

/*
   NOTE:
   The patrol area is mostly empty, and we don't care about empty spaces. Checking them is a waste
   of time. Instead, only the guard and obstacles need to be tracked.

   When using raycasting for collision detection, filter the list of obstacles down to only what is
   directly in front of the guard. Then, find the closest obstacle for the given direction
   using `min_by_key` and `max_by_key`.
*/

/// An object in the [PatrolArea] which the [Guard] may collide with
#[derive(Copy, Clone)]
struct Obstacle {
    position: Position,
}

impl Obstacle {
    fn new(x: usize, y: usize) -> Self {
        Self { position: (x, y) }
    }
}

#[derive(Default)]
pub struct PatrolArea {
    guard: Guard,
    obstacles: Vec<Obstacle>,
    width: usize,
    height: usize,
}

impl PatrolArea {
    /// Steps the patrol one time; returning `true` if the [Guard] remains in the [PatrolArea]
    pub fn step_patrol(&mut self) -> bool {
        self.guard.patrol(&self.obstacles, self.width, self.height)
    }
}

impl FromStr for PatrolArea {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut guard_dir: Option<Direction> = None;
        let mut guard_pos: Option<Position> = None;
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

#[derive(Default, Debug, Copy, Clone)]
enum CellContents {
    #[default]
    Empty,
    Obstacle,
    Guard,
    Visited,
}

impl Display for PatrolArea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cell_contents = vec![vec![CellContents::default(); self.width]; self.height];

        let is_in_range =
            |x, y| -> bool { (0..self.width).contains(&x) && (0..self.height).contains(&y) };

        for obs_pos in self.obstacles.iter().map(|obs| obs.position) {
            cell_contents[obs_pos.1][obs_pos.0] = CellContents::Obstacle;
        }
        for visited in &self.guard.distinct_positions {
            if is_in_range(visited.0, visited.1) {
                cell_contents[visited.1][visited.0] = CellContents::Visited;
            }
        }
        let guard_pos = self.guard.current_position;
        if is_in_range(guard_pos.0, guard_pos.1) {
            cell_contents[guard_pos.1][guard_pos.0] = CellContents::Guard;
        }

        let mut display_buf = String::new();
        for row in &cell_contents {
            for cell in row {
                let c = match cell {
                    CellContents::Empty => '░',
                    CellContents::Obstacle => '█',
                    CellContents::Guard => match self.guard.direction {
                        Direction::North => '▲',
                        Direction::East => '▶',
                        Direction::South => '▼',
                        Direction::West => '◀',
                    },
                    CellContents::Visited => '◈',
                };
                display_buf.push(c);
            }
            display_buf.push('\n');
        }

        write!(f, "{display_buf}")
    }
}

#[derive(Default)]
pub struct Day06 {
    patrol_area: PatrolArea,
}

impl Display for Day06 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let patrol_area = format!("{}", self.patrol_area);
        let guard_info = format!(
            "Guard at ({}, {})",
            self.patrol_area.guard.current_position.0, self.patrol_area.guard.current_position.1
        );

        write!(f, "{}{}", patrol_area, guard_info)
    }
}

impl Day06 {
    fn distinct_patrol_position_count(&mut self) -> usize {
        while self.patrol_area.step_patrol() {}
        self.patrol_area.guard.distinct_positions.len() - 1
    }
}

const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

impl AoCDay for Day06 {
    fn part1(&mut self) {
        // while self.patrol_area.step_patrol() {
        //     println!("{}", self);
        //     thread::sleep(Duration::from_millis(125));
        // }
        // println!("{}", self);
        // let count = self.patrol_area.guard.distinct_positions.len();
        let count = self.distinct_patrol_position_count();
        println!("\nDistinct patrol positions: {}", count);
    }

    fn part2(&mut self) {
        todo!()
    }

    fn load_input(&mut self, path: &Path) -> anyhow::Result<()> {
        let input = std::fs::read_to_string(path)?;
        let patrol_area = PatrolArea::from_str(&input).unwrap();

        self.patrol_area = patrol_area;
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
    fn parse_patrol_area() {
        let patrol_area = PatrolArea::from_str(EXAMPLE_INPUT);

        assert!(patrol_area.is_ok());
    }

    #[test]
    fn part1() {
        let mut day = Day06::default();
        day.load_input(&EXAMPLE_PATH).unwrap();

        let expected = 41;
        let actual = day.distinct_patrol_position_count();

        assert_eq!(expected, actual);
    }
}
