use itertools::Itertools;
use std::iter::once;
use std::ops::BitAnd;

pub fn part_one(input: &str) -> Option<String> {
    Some(checksum(dragon_curve(
        input.trim().chars().collect_vec(),
        272,
    )))
}

pub fn part_two(input: &str) -> Option<String> {
    Some(checksum(dragon_curve(
        input.trim().chars().collect_vec(),
        35651584,
    )))
}

fn dragon_curve(data: Vec<char>, size: usize) -> Vec<char> {
    if data.len() >= size {
        data.into_iter().take(size).collect_vec()
    } else {
        dragon_curve(expand(data), size)
    }
}

fn expand(data: Vec<char>) -> Vec<char> {
    data.clone()
        .into_iter()
        .chain(once('0'))
        .chain(
            data.iter()
                .map(|c| match c {
                    '0' => '1',
                    _ => '0',
                })
                .rev()
                .collect_vec(),
        )
        .collect_vec()
}

fn checksum(data: Vec<char>) -> String {
    if data.len().bitand(0x1) == 0x1 {
        data.into_iter().collect()
    } else {
        checksum(
            data.into_iter()
                .tuples()
                .map(|(a, b)| match a == b {
                    true => '1',
                    false => '0',
                })
                .collect(),
        )
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code::aoc_cli::check;

    #[test]
    fn test_dragon_curve() {
        let input = advent_of_code::read_file("examples", 16);
        let data = input.chars().collect_vec();
        let curve = dragon_curve(data.clone(), 20);
        let sum = checksum(curve.clone());
        assert_eq!(data.len(), 5);
        assert_eq!(
            curve.into_iter().collect::<String>().as_str(),
            "10000011110010000111"
        );
        assert_eq!(sum.as_str(), "01100");
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some("10111110011110111".to_string()));
    }
}
