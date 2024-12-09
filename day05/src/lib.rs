use aoc_day::AoCDay;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::rc::Rc;

#[derive(Default, Clone)]
pub struct Page {
    id: u32,
    order_rule: Option<Rc<RefCell<OrderRule>>>,
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Page {}

impl PartialOrd<Self> for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        let other_id = other.id;
        if let Some(self_order_rule) = &self.order_rule {
            let self_order_rule = self_order_rule.borrow();
            if self_order_rule.less_than.contains(&other_id) {
                Ordering::Less
            } else if self_order_rule.greater_than.contains(&other_id) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        } else if let Some(other_order_rule) = &other.order_rule {
            let other_order_rule = other_order_rule.borrow();
            if other_order_rule.less_than.contains(&self.id) {
                Ordering::Greater
            } else if other_order_rule.greater_than.contains(&self.id) {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        } else {
            Ordering::Equal
        }
    }
}

#[derive(Default)]
pub struct OrderRule {
    less_than: HashSet<u32>,
    greater_than: HashSet<u32>,
}

#[derive(Default)]
struct OrderingRules {
    order_map: HashMap<u32, Rc<RefCell<OrderRule>>>,
}

impl OrderingRules {
    fn new(page_map: HashMap<u32, HashSet<u32>>) -> Self {
        let mut rules: HashMap<u32, Rc<RefCell<OrderRule>>> = HashMap::new();

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
                Rc::new(RefCell::new(OrderRule {
                    less_than,
                    greater_than,
                })),
            );
        }

        Self { order_map: rules }
    }
}

#[derive(Default, Clone)]
pub struct Update {
    pages: Vec<Page>,
}

impl Update {
    pub fn is_sorted(&self) -> bool {
        self.pages.is_sorted()
    }

    pub fn middle_num(&self) -> u32 {
        let middle_idx = self.pages.len() / 2;
        self.pages[middle_idx].id
    }

    pub fn sort(&mut self) {
        self.pages.sort_unstable()
    }
}

#[derive(Default)]
pub struct Day05 {
    updates: Vec<Update>,
}

impl Day05 {
    pub fn sum_middle_numbers_sorted(&self) -> u32 {
        self.updates
            .iter()
            .filter(|update| update.is_sorted())
            .fold(0, |acc, update| acc + update.middle_num())
    }

    pub fn sum_middle_numbers_unsorted(&self) -> u32 {
        let unsorted_updates = self
            .updates
            .iter()
            .filter(|update| !update.is_sorted())
            .cloned();

        unsorted_updates.fold(0, |acc, mut update| {
            update.sort();
            acc + update.middle_num()
        })
    }
}

impl AoCDay for Day05 {
    fn part1(&self) {
        let sum = self.sum_middle_numbers_sorted();
        println!("Sum of middle numbers (sorted): {}", sum);
    }

    fn part2(&self) {
        let sum = self.sum_middle_numbers_unsorted();
        println!("Sum of middle numbers (unsorted): {}", sum);
    }

    fn load_input(&mut self, path: &Path) -> anyhow::Result<()> {
        let text = std::fs::read_to_string(path)?;

        self.updates = parse::input_to_updates(&text);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../example_input.txt");

    #[test]
    fn update_is_sorted() {
        let updates = parse::input_to_updates(TEST_INPUT);

        let expected_sort = [true, true, true, false, false, false];

        for (idx, update) in updates.iter().enumerate() {
            let actual_sort = update.is_sorted();
            assert_eq!(expected_sort[idx], actual_sort, "Failed idx: {}", idx);
        }
    }

    #[test]
    fn part1_test() {
        let updates = parse::input_to_updates(TEST_INPUT);
        let day05 = Day05 { updates };

        assert_eq!(day05.sum_middle_numbers_sorted(), 143);
    }

    #[test]
    fn part2_test() {
        let updates = parse::input_to_updates(TEST_INPUT);
        let day05 = Day05 { updates };

        assert_eq!(day05.sum_middle_numbers_unsorted(), 123);
    }
}

pub mod parse {
    use super::{OrderingRules, Page, Update};
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

    fn update_pages(input: &str) -> IResult<&str, Vec<u32>> {
        let (input, pages) = separated_list1(complete::char(','), complete::u32)(input)?;

        Ok((input, pages))
    }

    fn updates(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
        many1(terminated(update_pages, complete::char('\n')))(input)
    }

    pub fn input_to_updates(input: &str) -> Vec<Update> {
        let (_input, parsed_vals) =
            separated_pair(order_rules, complete::char('\n'), updates)(input).unwrap();

        let updates = parsed_vals
            .1
            .iter()
            .map(|page_nums| {
                let pages: Vec<Page> = page_nums
                    .iter()
                    .map(|page_num| {
                        let order_rule = parsed_vals.0.order_map.get(page_num).cloned();

                        Page {
                            id: *page_num,
                            order_rule,
                        }
                    })
                    .collect();

                Update { pages }
            })
            .collect::<Vec<Update>>();

        updates
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
            let updates = input_to_updates(TEST_INPUT);

            assert!(!updates.is_empty());
        }
    }
}
