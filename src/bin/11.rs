use crate::parse::floors;
use crate::PartType::{Generator, Microchip};
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Add;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    let floors = floors(input).unwrap().1;

    let mut heap = BinaryHeap::from([State::new(floors)]);
    while let Some(state) = heap.pop() {
        if state.finished() {
            //best steps
            break;
        }
        let floor_parts = state.floors[state.floor].clone();
        let possible_moves = floor_parts
            .iter()
            .combinations(2)
            .chain(floor_parts.iter().combinations(1));

        for moving_parts in possible_moves {
            let elevator_moves = vec![state.floor.saturating_sub(1), state.floor.add(1).min(3)]
                .into_iter()
                .filter(|&floor| floor != state.floor)
                .unique()
                .collect_vec();

            for target_floor in elevator_moves {
                // println!("{target_floor}");
                let floors = state
                    .floors
                    .clone()
                    .into_iter()
                    .enumerate()
                    .map(|(floor, parts)| {
                        if floor == state.floor {
                            parts
                                .into_iter()
                                .filter(|part| !moving_parts.contains(&part))
                                .collect_vec()
                        } else if floor == target_floor {
                            parts
                                .into_iter()
                                .chain(moving_parts.iter().map(|&part| *part))
                                .collect_vec()
                        } else {
                            parts.into_iter().collect_vec()
                        }
                    })
                    .collect_vec();
                if (tenable(floors.get(state.floor).unwrap())
                    && tenable(floors.get(target_floor).unwrap()))
                {
                    heap.push(State {
                        priority: floors
                            .iter()
                            .enumerate()
                            .map(|(floor, parts)| floor * parts.len())
                            .sum(),
                        moves: state.moves + 1,
                        floor: target_floor,
                        floors,
                    });
                }
            }
        }
    }
    None
}

fn tenable(parts: &Vec<Part>) -> bool {
    let mut rtgs: Vec<&Part> = vec![];
    let mut chips: Vec<&Part> = vec![];
    for part in parts.iter() {
        match part {
            (element, Generator) => {
                if let Some(chip) = chips.iter().position(|&&(other, _)| other == *element) {
                    chips.remove(chip);
                } else {
                    rtgs.push(part);
                }
            }
            (element, Microchip) => {
                if let Some(rtg) = rtgs.iter().position(|&&(other, _)| other == *element) {
                    rtgs.remove(rtg);
                } else {
                    chips.push(part);
                }
            }
        }
    }
    //all chips have matching generators, or if not, then there are no unmatched generators
    chips.is_empty() || rtgs.is_empty()
}
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug, Eq, PartialEq)]
struct Pair(Element, u8, u8);
#[derive(Debug, Eq, PartialEq)]
struct State {
    priority: usize,
    moves: usize,
    floor: usize,
    floors: Vec<Vec<Part>>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl State {
    fn new(floors: Vec<Vec<Part>>) -> Self {
        Self {
            priority: 0,
            moves: 0,
            floor: 0,
            floors,
        }
    }
    /// Finished when all pairs are at the top floor => 4
    /// Floors => 1..4
    fn finished(&self) -> bool {
        self.floors
            .iter()
            .enumerate()
            .filter(|(floor, _)| *floor != 3)
            .all(|(_, floor)| floor.is_empty())
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Element {
    Hydrogen,
    Lithium,
    Thulium,
    Plutonium,
    Strontium,
    Promethium,
    Ruthenium,
    Elerium,
    Dilithium,
}
impl FromStr for Element {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "hydrogen" => Ok(Element::Hydrogen),
            "lithium" => Ok(Element::Lithium),
            "thulium" => Ok(Element::Thulium),
            "plutonium" => Ok(Element::Plutonium),
            "strontium" => Ok(Element::Strontium),
            "promethium" => Ok(Element::Promethium),
            "ruthenium" => Ok(Element::Ruthenium),
            "elerium" => Ok(Element::Elerium),
            "dilithium" => Ok(Element::Dilithium),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum PartType {
    Generator,
    Microchip,
}

type Part = (Element, PartType);
fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
mod parse {
    use crate::PartType::{Generator, Microchip};
    use crate::{Element, Part};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{alpha1, line_ending};
    use nom::combinator::{map, map_res};
    use nom::multi::separated_list1;
    use nom::sequence::{delimited, preceded, terminated};
    use nom::IResult;
    use std::str::FromStr;

    pub fn floors(input: &str) -> IResult<&str, Vec<Vec<Part>>> {
        separated_list1(line_ending, floor)(input)
    }

    fn floor(input: &str) -> IResult<&str, Vec<Part>> {
        delimited(
            delimited(tag("The "), alpha1, tag(" floor contains ")),
            alt((parts, no_parts)),
            tag("."),
        )(input)
    }
    fn parts(input: &str) -> IResult<&str, Vec<Part>> {
        separated_list1(
            alt((tag(", and "), tag(", "), tag(" and "))),
            preceded(tag("a "), part),
        )(input)
    }
    fn no_parts(input: &str) -> IResult<&str, Vec<Part>> {
        map(tag("nothing relevant"), |_| vec![])(input)
    }
    fn part(input: &str) -> IResult<&str, Part> {
        alt((generator, microchip))(input)
    }
    fn generator(input: &str) -> IResult<&str, Part> {
        map(terminated(element, tag(" generator")), |el| (el, Generator))(input)
    }
    fn microchip(input: &str) -> IResult<&str, Part> {
        map(terminated(element, tag("-compatible microchip")), |el| {
            (el, Microchip)
        })(input)
    }
    fn element(input: &str) -> IResult<&str, Element> {
        map_res(alpha1, Element::from_str)(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
