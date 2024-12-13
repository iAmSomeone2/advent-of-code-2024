use std::fmt::{write, Formatter};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Operator {
    Add,
    Mul,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Operator {
    const COUNT: u64 = 2;

    fn next(&self) -> Self {
        match self {
            Self::Add => Self::Mul,
            Self::Mul => Self::Add,
        }
    }

    fn prev(&self) -> Self {
        match self {
            Self::Add => Self::Mul,
            Self::Mul => Self::Add,
        }
    }
}

struct OpPermutator {
    operators: Vec<Operator>,
    /// Maximum number of mutations
    permutation_max: u64,
    /// Number of times a mutation has occurred.
    permutation_count: u64,
    /// Current index being held in-place
    hold_idx: usize,
    /// Current index to increment
    inc_idx: usize,
}

impl std::fmt::Display for OpPermutator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ops_str: String = self.operators.iter().fold(String::new(), |acc, op| {
            if acc.is_empty() {
                format!("{op}")
            } else {
                format!("{acc}, {op}")
            }
        });

        write!(f, "[{ops_str}]")
    }
}

/*
   000
   010
   001
   100
   110
   101
   110
   111
*/

impl OpPermutator {
    fn new(size: usize) -> Self {
        Self {
            operators: vec![Operator::Add; size],
            permutation_max: Operator::COUNT.pow(size as u32),
            permutation_count: 1,
            hold_idx: 0,
            inc_idx: 1,
        }
    }

    fn next_permutation(&mut self) -> Vec<Operator> {
        let inc_op = self.operators[self.inc_idx].next();
        self.operators[self.inc_idx] = inc_op;

        if self.inc_idx > self.hold_idx + 1 {
            let dec_idx = self.inc_idx - 1;
            let dec_op = self.operators[dec_idx].prev();
            self.operators[dec_idx] = dec_op;
        }

        self.permutation_count += 1;
        self.inc_idx += 1;
        if self.inc_idx >= self.operators.len() {
            let inc_op = self.operators[self.hold_idx].next();
            self.operators[self.hold_idx] = inc_op;

            self.hold_idx += 1;
            if self.hold_idx >= self.operators.len() - 1 {
                self.hold_idx = 0;
            }
            self.inc_idx = self.hold_idx + 1;
        }

        self.get_ops()
    }

    fn fully_mutated(&self) -> bool {
        self.permutation_count >= self.permutation_max
    }

    fn get_ops(&self) -> Vec<Operator> {
        self.operators.clone()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Equation {
    expected_result: u64,
    inputs: Vec<u64>,
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, eq) = parse::equation(s).unwrap();

        Ok(eq)
    }
}

impl Equation {
    fn is_possible(&self) -> bool {
        let mut ops = OpPermutator::new(self.inputs.len() - 1);
        let mut is_possible = false;

        let mut result;
        let mut op_permutation = ops.get_ops();
        while !ops.fully_mutated() && !is_possible {
            result = self.inputs[0];
            for (i, input) in self.inputs.iter().skip(1).enumerate() {
                match op_permutation[i] {
                    Operator::Add => {
                        result += *input;
                    }
                    Operator::Mul => {
                        result *= input;
                    }
                }
            }

            is_possible = result == self.expected_result;
            op_permutation = ops.next_permutation();
        }

        is_possible
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn mutator_test() {
        let mut ops = OpPermutator::new(3);
        // println!("{ops}");

        assert_eq!(ops.permutation_max, 8);
        let mut mutation_set = HashSet::with_capacity(ops.permutation_max as usize);
        mutation_set.insert(ops.get_ops());

        for _ in 0..9 {
            let mutation = ops.next_permutation();
            // println!("{ops}");
            mutation_set.insert(mutation);
        }

        for val in &mutation_set {
            let ops_str: String = val.iter().fold(String::new(), |acc, op| {
                if acc.is_empty() {
                    format!("{op}")
                } else {
                    format!("{acc}, {op}")
                }
            });
            println!("[{ops_str}]");
        }

        assert_eq!(mutation_set.len(), 8)
    }
}

mod parse {
    use super::Equation;
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete;
    use nom::combinator::eof;
    use nom::multi::{separated_list0, separated_list1};
    use nom::sequence::{terminated, tuple};
    use nom::IResult;

    pub fn equation(input: &str) -> IResult<&str, Equation> {
        let get_expected_result = terminated(complete::u64, tag(": "));
        let get_inputs = separated_list1(complete::multispace1, complete::u64);

        let (input, (expected_result, inputs)) = tuple((get_expected_result, get_inputs))(input)?;

        let equation = Equation {
            expected_result,
            inputs,
        };

        Ok((input, equation))
    }

    pub fn all_equations(input: &str) -> IResult<&str, Vec<Equation>> {
        let ending_options = alt((complete::line_ending, eof));
        separated_list0(ending_options, equation)(input)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_INPUT: &str = include_str!("../example.txt");

        #[test]
        fn parse_equation() {
            let input = "7290: 6 8 6 15";

            let expected = Equation {
                expected_result: 7290,
                inputs: vec![6, 8, 6, 15],
            };
            let (remainder, actual) = equation(input).unwrap();

            assert!(remainder.is_empty());
            assert_eq!(expected, actual);
        }

        #[test]
        fn parse_all_equations() {
            let expected = vec![
                Equation {
                    expected_result: 7290,
                    inputs: vec![6, 8, 6, 15],
                },
                Equation {
                    expected_result: 190,
                    inputs: vec![10, 19],
                },
            ];
            let (remainder, actual) = all_equations(TEST_INPUT).unwrap();

            assert!(remainder.is_empty());
            assert_eq!(expected, actual);
        }
    }
}
