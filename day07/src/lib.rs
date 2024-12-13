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
            permutation_count: 0,
            hold_idx: 0,
            inc_idx: 1,
        }
    }

    fn update_operator_next(&mut self, idx: usize) {
        let updated_operator = self.operators[idx].next();
        self.operators[idx] = updated_operator;
    }

    fn update_operator_prev(&mut self, idx: usize) {
        let updated_operator = self.operators[idx].prev();
        self.operators[idx] = updated_operator;
    }

    fn fully_mutated(&self) -> bool {
        self.permutation_count >= self.permutation_max
    }

    fn get_ops(&self) -> Vec<Operator> {
        self.operators.clone()
    }
}

impl Iterator for OpPermutator {
    type Item = Vec<Operator>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.fully_mutated() {
            return None;
        } else if self.permutation_count == 0 {
            // Return the initial permutation
            self.permutation_count += 1;
            return Some(self.operators.clone());
        }

        let ops_len = self.operators.len();

        if self.hold_idx < ops_len - 1 {
            if self.inc_idx >= ops_len {
                // Decrement the final value and move on to the next set
                self.update_operator_prev(ops_len - 1);
                self.update_operator_prev(self.hold_idx);
                self.hold_idx += 1;
                self.inc_idx = self.hold_idx + 1;
            } else {
                if self.inc_idx > self.hold_idx + 1 {
                    // Increment the value at the current 'inc_idx' and decrement the one behind it
                    self.update_operator_prev(self.inc_idx - 1);
                }
                self.update_operator_next(self.inc_idx);
                self.inc_idx += 1;
            }
        } else if self.hold_idx == ops_len - 1 {
            self.update_operator_next(self.hold_idx);
        } else {
            dbg!(&self.operators);
            return None;
        }

        self.permutation_count += 1;
        Some(self.operators.clone())
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
        let ops = OpPermutator::new(self.inputs.len() - 1);

        let mut result;
        for permutation in ops {
            result = self.inputs[0];
            for (i, input) in self.inputs.iter().skip(1).enumerate() {
                match permutation[i] {
                    Operator::Add => {
                        result += *input;
                    }
                    Operator::Mul => {
                        result *= input;
                    }
                }
            }

            if result == self.expected_result {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn mutator_test() {
        let ops = OpPermutator::new(3);

        assert_eq!(ops.permutation_max, 8);
        let mut mutation_set = HashSet::with_capacity(ops.permutation_max as usize);
        mutation_set.insert(ops.get_ops());

        let print_ops = |ops: &[Operator]| {
            let ops_str = ops.iter().fold(String::new(), |acc, op| {
                if acc.is_empty() {
                    format!("{op}")
                } else {
                    format!("{acc}, {op}")
                }
            });
            println!("[{ops_str}]");
        };

        for permutation in ops {
            print_ops(&permutation);
            mutation_set.insert(permutation);
        }

        // for val in &mutation_set {
        //
        // }

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
