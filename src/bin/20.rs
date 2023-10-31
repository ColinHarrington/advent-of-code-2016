use itertools::Itertools;
use nom::character::complete::{char, line_ending, u32 as nom_u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::ops::RangeInclusive;

pub fn part_one(input: &str) -> Option<u32> {
    let ranges = ip_ranges(input).unwrap().1;
    let mut min = 0u32;
    while let Some(range) = ranges.iter().find(|range| range.contains(&min)) {
        min = range.end() + 1;
    }
    Some(min)
}

pub fn part_two(input: &str) -> Option<u32> {
    let ranges = ip_ranges(input).unwrap().1;
    let mut block: Vec<IpRange> = vec![];
    //Find the start
    // let start = ranges
    //     .iter()
    //     .find(|range| range.contains(0))
    //     .unwrap()
    //     .clone();
    let r = ranges.iter().fold((0u32..=0u32), |acc, range| {
        match range.start() <= &(acc.end() + 1) {
            true => *acc.start()..=*range.end(),
            false => acc,
        }
    });
    // -> find all overlapping
    // loop until no contiguous
    // ranges.iter().reduce()
    None
}

type IpRange = RangeInclusive<u32>;

fn ip_ranges(input: &str) -> IResult<&str, Vec<IpRange>> {
    separated_list1(line_ending, ip_range)(input)
}

fn ip_range(input: &str) -> IResult<&str, IpRange> {
    map(separated_pair(nom_u32, char('-'), nom_u32), |(from, to)| {
        from..=to
    })(input)
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
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), None);
    }
}
