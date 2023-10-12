use crate::Direction::{East, North, South, West};
use crate::Rotate::{Left, Right};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{i32 as nom_i32, one_of};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = instructions(&input.trim()).unwrap().1;
    let bunny = instructions
        .iter()
        .fold(Bunny::new(), |bunny, instruction| bunny.jump(instruction));
    Some(distance(bunny.location))
}

/// 257 => too high
pub fn part_two(input: &str) -> Option<u32> {
    let mut bunny = Bunny::new();
    let mut visited: HashSet<Location> = HashSet::from([bunny.location]);

    instructions(input.trim())
        .unwrap()
        .1
        .iter()
        .flat_map(|(rotation, steps)| {
            bunny.rotate(rotation);
            (0..*steps).map(|_| bunny.hop()).collect_vec()
        })
        .find_map(|location| match visited.insert(location) {
            true => None,
            false => Some(distance(location)),
        })
}

fn distance(location: Location) -> u32 {
    (location.0.abs() + location.1.abs()) as u32
}

struct Bunny {
    heading: Direction,
    location: Location,
}

impl Bunny {
    fn new() -> Self {
        Bunny {
            heading: North,
            location: (0, 0),
        }
    }

    fn jump(&self, (rotate, steps): &Instruction) -> Bunny {
        let heading = self.heading.rotate(rotate);
        let (x, y) = self.location;
        let location = match heading {
            North => (x, y + steps),
            South => (x, y - steps),
            East => (x + steps, y),
            West => (x - steps, y),
        };
        Bunny { heading, location }
    }

    fn hop(&mut self) -> Location {
        self.location = match self.heading {
            North => (self.location.0, self.location.1 + 1),
            South => (self.location.0, self.location.1 - 1),
            East => (self.location.0 + 1, self.location.1),
            West => (self.location.0 - 1, self.location.1),
        };
        self.location
    }

    fn rotate(&mut self, rotation: &Rotate) {
        self.heading = self.heading.rotate(rotation);
    }
}
type Instruction = (Rotate, i32);
type Location = (i32, i32);
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate(&self, rotatation: &Rotate) -> Direction {
        match rotatation {
            Right => match self {
                North => East,
                East => South,
                South => West,
                West => North,
            },
            Left => match self {
                North => West,
                East => North,
                South => East,
                West => South,
            },
        }
    }
}
#[derive(Debug)]
enum Rotate {
    Right,
    Left,
}
impl From<char> for Rotate {
    fn from(value: char) -> Self {
        match value {
            'R' => Right,
            'L' => Left,
            _ => unimplemented!("Not a Rotation"),
        }
    }
}
fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(tag(", "), instruction)(input)
}
fn instruction(input: &str) -> IResult<&str, Instruction> {
    tuple((rotate, nom_i32))(input)
}
fn rotate(input: &str) -> IResult<&str, Rotate> {
    map(one_of("RL"), Rotate::from)(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input).unwrap(), 12);
    }

    #[test]
    fn test_part_two() {
        let input = "R8, R4, R4, R8";
        assert_eq!(part_two(input).unwrap(), 4);
    }
}
