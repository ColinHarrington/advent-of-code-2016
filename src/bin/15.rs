use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{i64 as nom_i64, line_ending};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;
use ring_algorithm::chinese_remainder_theorem;
use std::iter::once;
use std::ops::{Add, Rem};

pub fn part_one(input: &str) -> Option<i64> {
    let (u, m): (Vec<i64>, Vec<i64>) = discs(input)
        .unwrap()
        .1
        .into_iter()
        .sorted_by_key(|d1| d1.positions)
        .map(|disc| (disc.remainder_at_zero(), disc.positions))
        .unzip();

    chinese_remainder_theorem::<i64>(&u, &m).map(|a| a.abs())
}

pub fn part_two(input: &str) -> Option<i64> {
    let discs = discs(input).unwrap().1;
    let extra = Disc {
        number: discs.iter().map(|disc| disc.number).max().unwrap() + 1,
        positions: 11,
        start: 0,
    };
    let (u, m): (Vec<i64>, Vec<i64>) = discs
        .into_iter()
        .chain(once(extra))
        .sorted_by_key(|d1| d1.positions)
        .map(|disc| (disc.remainder_at_zero(), disc.positions))
        .unzip();

    chinese_remainder_theorem::<i64>(&u, &m).map(|a| a.abs())
}

fn discs(input: &str) -> IResult<&str, Vec<Disc>> {
    separated_list1(line_ending, disc)(input)
}
fn disc(input: &str) -> IResult<&str, Disc> {
    map(
        tuple((
            preceded(tag("Disc #"), nom_i64),
            delimited(tag(" has "), nom_i64, tag(" positions;")),
            delimited(tag(" at time=0, it is at position "), nom_i64, tag(".")),
        )),
        |(number, positions, start)| Disc {
            number,
            positions,
            start,
        },
    )(input)
}
#[derive(Debug, Eq, PartialEq, Clone)]
struct Disc {
    number: i64,
    positions: i64,
    start: i64,
}

impl Disc {
    fn remainder_at_zero(&self) -> i64 {
        (self.start + self.number + self.positions).rem(self.positions)
    }
    fn position(&self, time: i64) -> i64 {
        (self.start + self.number + time).rem(self.positions)
    }
    fn can_pass(&self, time: i64) -> bool {
        (self.start + self.number + time).rem(self.positions) == 0
        // (time + self.start).rem(self.positions) == (self.number).rem(self.positions)
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), None);
    }
}
