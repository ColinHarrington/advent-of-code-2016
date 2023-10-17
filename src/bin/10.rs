use crate::Instruction::{BotPlay, Distribute};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, u32 as nom_u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::collections::{HashMap, VecDeque};

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = instructions(input).unwrap().1;
    balance_bots(instructions, true).map(|(bot, _)| bot)
}

pub fn part_two(input: &str) -> Option<u32> {
    let instructions = instructions(input).unwrap().1;
    balance_bots(instructions, false).map(|(_, outputs)| {
        outputs.get(&0).unwrap() * outputs.get(&1).unwrap() * outputs.get(&2).unwrap()
    })
}

fn balance_bots(
    instructions: Vec<Instruction>,
    early_exit: bool,
) -> Option<(u32, HashMap<u32, u32>)> {
    let mut outputs: HashMap<u32, u32> = HashMap::new();
    let mut values: HashMap<u32, u32> = HashMap::new();
    let plays: HashMap<u32, (Target, Target)> = HashMap::from_iter(instructions.iter().filter_map(
        |instruction| match instruction {
            Distribute(_, _) => None,
            BotPlay(bot, low, high) => Some((bot.number, (low.clone(), high.clone()))),
        },
    ));
    let mut queue = VecDeque::from_iter(
        instructions
            .into_iter()
            .filter(|instruction| matches!(instruction, Distribute(_, _))),
    );
    while let Some(instruction) = queue.pop_front() {
        match instruction {
            Distribute(value, target) => match target {
                Target::Bot(bot) => match values.get(&bot) {
                    Some(&existing) => {
                        let min = value.min(existing);
                        let max = value.max(existing);
                        if early_exit && min == 17 && max == 61 {
                            return Some((bot, outputs));
                        }
                        let (low, high) = plays.get(&bot).unwrap().clone();
                        queue.push_front(Distribute(min, low));
                        queue.push_front(Distribute(max, high));
                    }
                    None => {
                        assert_eq!(None, values.insert(bot, value));
                    }
                },
                Target::Output(output) => {
                    outputs.insert(output, value);
                }
            },
            BotPlay(_, _, _) => panic!("plays shouldn't be present"),
        }
    }
    Some((0, outputs))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}
fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((distribute, bot_play))(input)
}
fn distribute(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag("value "), nom_u32),
            preceded(tag(" goes to "), target),
        )),
        |(value, target)| Instruction::Distribute(value, target),
    )(input)
}
fn bot_play(input: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            bot,
            preceded(tag(" gives low to "), target),
            preceded(tag(" and high to "), target),
        )),
        |(bot, low, high)| Instruction::BotPlay(bot, low, high),
    )(input)
}
fn bot(input: &str) -> IResult<&str, Bot> {
    map(preceded(tag("bot "), nom_u32), |number| Bot { number })(input)
}

fn target(input: &str) -> IResult<&str, Target> {
    alt((target_bot, target_output))(input)
}
fn target_bot(input: &str) -> IResult<&str, Target> {
    map(preceded(tag("bot "), nom_u32), Target::Bot)(input)
}

fn target_output(input: &str) -> IResult<&str, Target> {
    map(preceded(tag("output "), nom_u32), Target::Output)(input)
}
#[derive(Debug, Eq, PartialEq, Clone)]
enum Instruction {
    Distribute(u32, Target),
    BotPlay(Bot, Target, Target),
}
#[derive(Debug, Eq, PartialEq, Clone)]
enum Target {
    Bot(u32),
    Output(u32),
}
#[derive(Debug, Eq, PartialEq, Clone)]
struct Bot {
    number: u32,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(0));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(30));
    }
}
