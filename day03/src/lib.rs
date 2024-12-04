use std::{fs, sync::LazyLock};

use aoc_day::AoCDay;
use regex::{Regex, RegexBuilder};

static INSTRUCTION_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    RegexBuilder::new(r"(?:(?P<name>(?:mul|do|don't))\((?P<args>(?:\d+,*)*)\))")
        .case_insensitive(true)
        .build()
        .expect("MUL_REGEX failed to compile")
});

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul(u64, u64),
    Do,
    Dont
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
                    },
                    _ => {},
                }

                ins
            })
            .collect::<Vec<Instruction>>()
    }

    fn execute(&self) -> Option<u64> {
        match self {
            Self::Mul(arg0, arg1) => {
                Some(arg0 * arg1)
            },
            _ => None,
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

mod parse {
    use nom::{branch::alt, bytes::complete::{is_not, tag, take_till, take_while}, character::complete::{alphanumeric1, char}, multi::many1, sequence::{delimited, pair, preceded, terminated}, IResult};

    use crate::Instruction;

    fn mul_instruction(i: &str) -> IResult<&str, Instruction> {
        let parse_arg0 = terminated(alphanumeric1, char(','));
        let parse_pair = pair(parse_arg0, alphanumeric1);

        let (remainder, output) = delimited(tag("mul("), parse_pair, tag(")"))(i)?;

        let arg0: u64 = match output.0.parse() {
            Ok(num) => num,
            Err(err) => {
                panic!("{err}");
            }
        };
        let arg1: u64 = match output.1.parse() {
            Ok(num) => num,
            Err(err) => {
                panic!("{err}");
            }
        };

        Ok((remainder, Instruction::Mul(arg0, arg1)))
    }

    fn do_instruction(i: &str) -> IResult<&str, Instruction> {
        let (remainder, _) = tag("do()")(i)?;

        Ok((remainder, Instruction::Do))
    }

    fn dont_instruction(i: &str) -> IResult<&str, Instruction> {
        let (remainder, _) = tag(r"don't()")(i)?;

        Ok((remainder, Instruction::Dont))
    }

    pub fn instructions(i: &str) -> IResult<&str, Vec<Instruction>> {
        let instruction = alt((mul_instruction, do_instruction, dont_instruction));

        todo!()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn mul_instruction_test() {
            let input = "mul(1,2)q";

            let actual = mul_instruction(input).unwrap();
            let expected = ("q", Instruction::Mul(1, 2));

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
            Instruction::Mul(2, 4),
            Instruction::Dont,
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Do,
            Instruction::Mul(8, 5),
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
