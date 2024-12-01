fn parse_input_line(line: &str) -> (i64, i64) {
    let ids: Vec<i64> = line.split_whitespace()
        .filter(|s| !s.is_empty())
        .take(2)
        .filter_map(|s| s.parse().ok())
        .collect();
    (ids[0], ids[1])
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    input.lines().map(parse_input_line).collect()
}

fn total_distance(input: &str) -> i64 {
    let (mut left_list, mut right_list) = parse_input(input);
    left_list.sort_unstable();
    right_list.sort_unstable();

    left_list.iter().enumerate().fold(0, |acc, (i, left_id)| {
        let right_id = right_list[i];
        acc + (left_id - right_id).abs()
    })
}

/// Day 01 - Part 1
pub fn part1(input: &str) {
    let result = total_distance(input);
    println!("Result:\t{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example_input.txt");

    #[test]
    fn parse_input_line_test() {
        let line = EXAMPLE_INPUT.lines().next().unwrap();
        let expected = (3, 4);
        let actual = parse_input_line(line);

        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_input_test() {
        let expected = (vec![3,4,2,1,3,3], vec![4,3,5,3,9,3]);
        let actual = parse_input(EXAMPLE_INPUT);

        assert_eq!(expected, actual);
    }

    #[test]
    fn total_distance_test() {
        let expected = 11;
        let actual = total_distance(EXAMPLE_INPUT);

        assert_eq!(expected, actual);
    }
}
