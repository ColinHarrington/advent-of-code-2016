use itertools::Itertools;
use nom::character::complete::{char, line_ending, u32 as nom_u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::BTreeSet;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        condense_ranges(ip_ranges(input).unwrap().1)
            .first()
            .unwrap()
            .end
            .saturating_add(1),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        condense_ranges(ip_ranges(input).unwrap().1)
            .iter()
            .tuple_windows()
            .map(|(a, b)| b.start.sub(a.end.add(1)))
            .sum(),
    )
}

/// Merges contiguous ranges until we have exhausted supplied ranges
/// Returns sorted distinct non-contiguous BlockRanges as a BTreeSet
fn condense_ranges(ranges: Vec<BlockRange>) -> BTreeSet<BlockRange> {
    let mut condensed = BTreeSet::new();

    for range in ranges {
        let expanded = condensed
            .iter()
            .filter(|other| range.is_contiguous(other))
            .fold(range.clone(), |acc, next| acc.merge(next));

        condensed.retain(|other| !expanded.is_contiguous(other));
        condensed.insert(expanded);
    }

    condensed
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Clone)]
struct BlockRange {
    start: u32,
    end: u32,
}
impl Display for BlockRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.start, self.end)
    }
}

impl BlockRange {
    fn new((start, end): (u32, u32)) -> Self {
        Self { start, end }
    }

    fn is_contiguous(&self, other: &BlockRange) -> bool {
        other.start <= self.end.saturating_add(1) && other.start >= self.start
            || self.start <= other.end.saturating_add(1) && self.start >= other.start
    }

    fn merge(&self, other: &BlockRange) -> BlockRange {
        BlockRange {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }
}

fn ip_ranges(input: &str) -> IResult<&str, Vec<BlockRange>> {
    separated_list1(line_ending, ip_range)(input)
}

fn ip_range(input: &str) -> IResult<&str, BlockRange> {
    map(separated_pair(nom_u32, char('-'), nom_u32), BlockRange::new)(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1));
    }
}
