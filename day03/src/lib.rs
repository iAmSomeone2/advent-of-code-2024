use std::{fs, sync::LazyLock};

use aoc_day::AoCDay;
use regex::{Regex, RegexBuilder};

static INSTRUCTION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r"(?P<instruction>mul)\((?P<args>(?:\d+,*)*)\)")
        .case_insensitive(true)
        .build()
        .expect("MUL_REGEX failed to compile")
});

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul(Vec<u64>),
}

impl Instruction {
    fn parse_instructions(input: &str) -> Vec<Instruction> {
        let regex_captures = INSTRUCTION_REGEX.captures_iter(input);

        regex_captures
            .map(|captures| {
                let mut ins = match &captures["instruction"] {
                    "mul" => Instruction::Mul(vec![]),
                    _ => panic!("Unexpected instruction"),
                };

                match ins {
                    Instruction::Mul(ref mut args) => {
                        let mut parsed_args = (&captures["args"])
                            .split(',')
                            .filter_map(|arg| u64::from_str_radix(arg, 10).ok())
                            .collect::<Vec<_>>();

                        args.append(&mut parsed_args);
                    }
                }

                ins
            })
            .collect::<Vec<Instruction>>()
    }

    fn execute(&self) -> Option<u64> {
        match self {
            Self::Mul(args) => {
                let product: u64 = args.iter().fold(1, |acc, elem| acc * (*elem));
                Some(product)
            }
        }
    }
}

#[derive(Default)]
pub struct Day03 {
    instructions: Vec<Instruction>,
}

impl Day03 {
    pub fn sum_mults(&self) -> u64 {
        self.instructions
            .iter()
            .filter(|ins| match ins {
                Instruction::Mul(_) => true,
                _ => false,
            })
            .fold(0, |acc, ins| acc + ins.execute().unwrap_or(0))
    }
}

impl AoCDay for Day03 {
    fn part1(&self) {
        let sum = self.sum_mults();
        println!("Sum of mults: {sum}");
    }

    fn part2(&self) {
        todo!()
    }

    fn load_input(&mut self, path: &std::path::Path) -> anyhow::Result<()> {
        let input = fs::read_to_string(path)?;

        self.instructions = Instruction::parse_instructions(&input);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    static EXAMPLE_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("example_input.txt")
    });
    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

    #[test]
    fn parse_instructions_test() {
        let expected = vec![
            Instruction::Mul(vec![2, 4]),
            Instruction::Mul(vec![5, 5]),
            Instruction::Mul(vec![11, 8]),
            Instruction::Mul(vec![8, 5]),
        ];

        let actual = Instruction::parse_instructions(EXAMPLE_INPUT);

        assert_eq!(actual, expected);
    }

    #[test]
    fn sum_mults_test() {
        let mut day = Day03::default();
        day.load_input(&EXAMPLE_PATH).unwrap();

        let expected = 161;
        let actual = day.sum_mults();

        assert_eq!(actual, expected);
    }
}
