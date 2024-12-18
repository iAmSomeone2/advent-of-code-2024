use aoc_day::AoCDay;
use std::collections::VecDeque;
use std::fmt::Display;
use std::io::Write;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::LazyLock;

#[derive(Default, Debug, Eq, PartialEq, Copy, Clone)]
enum Block {
    /// A block which contains data for the file with the matching ID
    File(u64),
    /// A free block
    #[default]
    Free,
}

#[derive(Debug, Default)]
struct FileSystem {
    blocks: VecDeque<Block>,
    last_file_block: usize,
    free_blocks: VecDeque<usize>,
}

impl Display for FileSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let blocks = self.blocks.iter().fold(String::new(), |acc, b| match b {
            Block::File(id) => format!("{acc}{}", id),
            Block::Free => format!("{acc}."),
        });

        write!(f, "{}", blocks)
    }
}

impl FromStr for FileSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut file_id = 0;
        let mut current_idx = 0;
        let mut blocks = VecDeque::new();
        let mut is_file = true;
        let mut last_file_block = 0;

        s.chars().filter_map(|c| c.to_digit(10)).for_each(|val| {
            let end_idx = current_idx + val as usize;
            let blocks_range = current_idx..end_idx;
            current_idx = end_idx;

            if is_file {
                blocks.extend(blocks_range.map(|_| Block::File(file_id)));
                file_id += 1;
                is_file = false;
                last_file_block = current_idx - 1;
            } else {
                blocks.extend(blocks_range.map(|_| Block::Free));
                is_file = true;
            }
        });

        let free_blocks = blocks
            .iter()
            .enumerate()
            .filter(|(_, b)| **b == Block::Free)
            .map(|(i, _)| i)
            .collect();

        Ok(FileSystem {
            blocks,
            free_blocks,
            last_file_block,
        })
    }
}

impl FileSystem {
    fn fragment_last_file_block(&mut self) {
        if let Some(first_free_block_idx) = self.free_blocks.pop_front() {
            self.blocks[first_free_block_idx] = self.blocks[self.last_file_block];
            self.blocks[self.last_file_block] = Block::Free;
            self.free_blocks.push_back(self.last_file_block);

            let max_idx = self.blocks.len() - 1;
            self.last_file_block = max_idx
                - self
                    .blocks
                    .iter()
                    .rev()
                    .position(|b| *b != Block::Free)
                    .unwrap();
        }
    }

    fn is_fragmented(&self) -> bool {
        let free_range = self.last_file_block + 1..self.blocks.len();
        self.free_blocks.iter().all(|b| free_range.contains(b))
    }

    fn fragment(&mut self) {
        while !self.is_fragmented() {
            self.fragment_last_file_block();
        }
    }

    fn checksum(&self) -> u64 {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, b)| match b {
                Block::Free => 0,
                Block::File(id) => (i as u64) * *id,
            })
            .sum()
    }
}

#[derive(Default)]
pub struct Day09 {
    fs: FileSystem,
}

impl AoCDay for Day09 {
    fn part1(&mut self) {
        self.fs.fragment();
        self.save_output().unwrap();
        let checksum = self.fs.checksum();
        println!("FS checksum: {}", checksum);
    }

    fn part2(&mut self) {
        todo!()
    }

    fn load_input(&mut self, input_path: &std::path::Path) -> anyhow::Result<()> {
        let input = std::fs::read_to_string(input_path)?;
        self.fs = FileSystem::from_str(&input).unwrap();
        Ok(())
    }
}

static OUTPUT_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| PathBuf::from("outputs").join("day09.txt"));

impl Day09 {
    pub fn save_output(&self) -> anyhow::Result<()> {
        println!("Saving output to {}", OUTPUT_PATH.as_path().display());
        let file = std::fs::File::create(OUTPUT_PATH.as_path())?;
        let mut writer = std::io::BufWriter::new(file);
        write!(writer, "{}", self.fs)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_parse_example_input() {
        let fs = FileSystem::from_str(EXAMPLE_INPUT).unwrap();
        assert_eq!(fs.to_string(), "00...111...2...333.44.5555.6666.777.888899");
    }

    #[test]
    fn test_defrag_last_file_block() {
        let mut fs = FileSystem::from_str(EXAMPLE_INPUT).unwrap();
        fs.fragment_last_file_block();

        assert_eq!(fs.to_string(), "009..111...2...333.44.5555.6666.777.88889.");
    }

    #[test]
    fn test_defrag_example_input() {
        let mut fs = FileSystem::from_str(EXAMPLE_INPUT).unwrap();
        fs.fragment();

        assert_eq!(fs.to_string(), "0099811188827773336446555566..............");
        assert_eq!(fs.checksum(), 1928);
    }
}
