use std::path::Path;

pub trait AoCDay {
    /// Part 1 implementation
    ///
    /// Result must be printed to the console.
    fn part1(&self);

    /// Part 2 implementation
    ///
    /// Result must be printed to the console.
    fn part2(&self);

    /// Load the Day's input from the given path
    fn load_input(&mut self, path: &Path) -> anyhow::Result<()>;
}
