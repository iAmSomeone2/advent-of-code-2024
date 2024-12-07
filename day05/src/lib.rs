use aoc_day::AoCDay;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Default)]
struct OrderRule {
    less_than: HashSet<u32>,
    greater_than: HashSet<u32>,
}

#[derive(Default)]
struct OrderingRules {
    order_map: HashMap<u32, OrderRule>,
}

impl OrderingRules {
    fn new(page_map: HashMap<u32, HashSet<u32>>) -> Self {
        let mut rules: HashMap<u32, OrderRule> = HashMap::new();

        for entry in &page_map {
            let page_num = *entry.0;
            let less_than = entry.1.clone();
            let mut greater_than: HashSet<u32> = HashSet::new();

            for other_entry in &page_map {
                if other_entry.1.contains(&page_num) {
                    greater_than.insert(*other_entry.0);
                }
            }

            rules.insert(
                page_num,
                OrderRule {
                    less_than,
                    greater_than,
                },
            );
        }

        Self { order_map: rules }
    }

    fn order(&self, left: u32, right: u32) -> Ordering {
        match self.order_map.get(&right) {
            Some(rule) => {
                if rule.greater_than.contains(&right) {
                    Ordering::Greater
                } else if rule.less_than.contains(&left) {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            }
            None => Ordering::Equal,
        }
    }
}

#[derive(Default)]
struct Update {
    pages: Vec<u32>,
}

impl Update {
    pub fn is_sorted(&self, rules: &OrderingRules) -> bool {
        self.pages.windows(2).all(|pair| {
            let order = rules.order(pair[0], pair[1]);
            order == Ordering::Less || order == Ordering::Equal
        })
    }

    pub fn middle_num(&self) -> u32 {
        let middle_idx = self.pages.len() / 2;
        self.pages[middle_idx]
    }
}

#[derive(Default)]
pub struct Day05 {
    ordering_rules: OrderingRules,
    updates: Vec<Update>,
}

impl Day05 {
    pub fn sum_middle_numbers(&self) -> u32 {
        self.updates
            .iter()
            .filter(|update| update.is_sorted(&self.ordering_rules))
            .fold(0, |acc, update| acc + update.middle_num())
    }
}

impl AoCDay for Day05 {
    fn part1(&self) {
        todo!()
    }

    fn part2(&self) {
        todo!()
    }

    fn load_input(&mut self, path: &Path) -> anyhow::Result<()> {
        let text = std::fs::read_to_string(path)?;

        let (rules, updates) = parse::rules_and_updates(&text);

        self.ordering_rules = rules;
        self.updates = updates;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../example_input.txt");

    #[test]
    fn part1_test() {
        let (rules, updates) = parse::rules_and_updates(TEST_INPUT);
        let day05 = Day05 {
            ordering_rules: rules,
            updates,
        };

        assert_eq!(day05.sum_middle_numbers(), 143);
    }
}

mod parse {
    use super::{OrderingRules, Update};
    use nom::character::complete;
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{separated_pair, terminated};
    use nom::IResult;
    use std::collections::{HashMap, HashSet};

    fn single_order_rule(input: &str) -> IResult<&str, (u32, u32)> {
        separated_pair(complete::u32, complete::char('|'), complete::u32)(input)
    }

    fn order_rules(input: &str) -> IResult<&str, OrderingRules> {
        let (input, rules) = many1(terminated(single_order_rule, complete::char('\n')))(input)?;

        let mut order_map = HashMap::new();
        for rule in rules {
            let entry = order_map.entry(rule.0).or_insert(HashSet::new());
            entry.insert(rule.1);
        }

        Ok((input, OrderingRules::new(order_map)))
    }

    fn single_update(input: &str) -> IResult<&str, Update> {
        let (input, pages) = separated_list1(complete::char(','), complete::u32)(input)?;

        Ok((input, Update { pages }))
    }

    fn updates(input: &str) -> IResult<&str, Vec<Update>> {
        many1(terminated(single_update, complete::char('\n')))(input)
    }

    pub fn rules_and_updates(input: &str) -> (OrderingRules, Vec<Update>) {
        let (_input, parsed_vals) =
            separated_pair(order_rules, complete::char('\n'), updates)(input).unwrap();

        parsed_vals
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        const TEST_INPUT: &str = include_str!("../example_input.txt");

        #[test]
        fn order_rules_test() {
            let (_, ordering_rules) = order_rules(TEST_INPUT).unwrap();

            assert!(!ordering_rules.order_map.is_empty());
        }

        #[test]
        fn rules_and_updates_test() {
            let (rules, updates) = rules_and_updates(TEST_INPUT);

            assert!(!rules.order_map.is_empty());
            assert!(!updates.is_empty());
        }
    }
}
