use nom::character::complete;
use nom::character::complete::anychar;
use nom::multi::many_till;
use nom::sequence::separated_pair;
use nom::{branch::alt, bytes::complete::tag, multi::many1, sequence::delimited, IResult, Parser};
use std::{fs, sync::LazyLock};

use aoc_day::AoCDay;
use regex::{Regex, RegexBuilder};

// const INPUT: &str = include_str!("../../inputs/day03.txt");

static INSTRUCTION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r"(?:(?P<name>(?:mul|do|don't))\((?P<args>(?:\d+,*)*)\))")
        .case_insensitive(true)
        .build()
        .expect("INSTRUCTION_REGEX failed to compile")
});

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum InstructionResult {
    Product(u64),
    Do,
    Dont,
}

impl Instruction {
    fn parse_instructions(input: &str) -> Vec<Instruction> {
        let regex_captures = INSTRUCTION_REGEX.captures_iter(input);

        regex_captures
            .map(|captures| {
                let ins = &captures["name"];
                let mut ins = match ins {
                    "mul" => Instruction::Mul(0, 0),
                    "do" => Instruction::Do,
                    "don't" => Instruction::Dont,
                    _ => panic!("Unexpected instruction"),
                };

                match ins {
                    Instruction::Mul(ref mut arg0, ref mut arg1) => {
                        let parsed_args = (&captures["args"])
                            .split(',')
                            .filter_map(|arg| u64::from_str_radix(arg, 10).ok())
                            .collect::<Vec<_>>();

                        *arg0 = parsed_args[0];
                        *arg1 = parsed_args[1];
                    }
                    _ => {}
                }

                ins
            })
            .collect::<Vec<Instruction>>()
    }

    fn execute(&self) -> InstructionResult {
        match self {
            Self::Mul(arg0, arg1) => InstructionResult::Product(arg0 * arg1),
            Self::Do => InstructionResult::Do,
            Self::Dont => InstructionResult::Dont,
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
                Instruction::Mul(..) => true,
                _ => false,
            })
            .fold(0, |acc, ins| match ins.execute() {
                InstructionResult::Product(res) => acc + res,
                _ => acc,
            })
    }

    pub fn sum_mults2(&self) -> u64 {
        let mut result = 0;
        let mut should_execute = true;
        for ins in self.instructions.iter() {
            match ins.execute() {
                InstructionResult::Product(res) => {
                    if should_execute {
                        result += res;
                    }
                }
                InstructionResult::Do => should_execute = true,
                InstructionResult::Dont => should_execute = false,
            }
        }

        result
    }
}

impl AoCDay for Day03 {
    fn part1(&self) {
        let sum = self.sum_mults();
        println!("Sum of mults: {sum}");
    }

    fn part2(&self) {
        let sum = self.sum_mults2();
        println!("Sum of mults: {sum}");
    }

    fn load_input(&mut self, path: &std::path::Path) -> anyhow::Result<()> {
        let input = fs::read_to_string(path)?;

        let (_rem, instructions) = parse_instructions(&input)?;

        self.instructions = instructions;

        Ok(())
    }
}

fn mul_instruction(i: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(i)?;
    let (input, pair): (&str, (u64, u64)) = delimited(
        tag("("),
        separated_pair(complete::u64, complete::char(','), complete::u64),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn do_instruction(i: &str) -> IResult<&str, Instruction> {
    let (remainder, _) = tag("do()")(i)?;

    Ok((remainder, Instruction::Do))
}

fn dont_instruction(i: &str) -> IResult<&str, Instruction> {
    let (remainder, _) = tag(r"don't()")(i)?;

    Ok((remainder, Instruction::Dont))
}

fn parse_instructions(i: &str) -> IResult<(), Vec<Instruction>> {
    let instruction = alt((mul_instruction, do_instruction, dont_instruction));

    let (_, instructions) =
        many1(many_till(anychar, instruction).map(|(_discard, ins)| ins))(i).unwrap();

    Ok(((), instructions))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    static EXAMPLE_PATH: LazyLock<PathBuf> =
        LazyLock::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("example_input.txt"));

    static EXAMPLE_PATH2: LazyLock<PathBuf> =
        LazyLock::new(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("example_input2.txt"));

    const EXAMPLE1: &str = include_str!("../example_input.txt");
    const EXAMPLE2: &str = include_str!("../example_input2.txt");

    #[test]
    fn mul_instruction_test() {
        let input = "mul(1,2)q";

        let actual = mul_instruction(input).unwrap();
        let expected = ("q", Instruction::Mul(1, 2));

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_all_muls_test() {
        let expected = [
            Instruction::Mul(2, 4),
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Mul(8, 5),
        ];
        let (_, actual) = parse_instructions(EXAMPLE1).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn do_instruction_test() {
        let input = "do()124";

        let actual = do_instruction(input).unwrap();
        let expected = ("124", Instruction::Do);

        assert_eq!(actual, expected);
    }

    #[test]
    fn dont_instruction_test() {
        let input = "don't()124";

        let actual = dont_instruction(input).unwrap();
        let expected = ("124", Instruction::Dont);

        assert_eq!(actual, expected);
    }

    #[test]
    fn parse_all_instructions_test() {
        let expected = [
            Instruction::Mul(2, 4),
            Instruction::Dont,
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Do,
            Instruction::Mul(8, 5),
        ];

        let (_, actual) = parse_instructions(EXAMPLE2).unwrap();
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

    #[test]
    fn sum_mults2_test() {
        let mut day = Day03::default();
        day.load_input(&EXAMPLE_PATH2).unwrap();

        let expected = 48;
        let actual = day.sum_mults2();

        assert_eq!(actual, expected);
    }
}
