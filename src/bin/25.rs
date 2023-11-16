use crate::parse::instructions;
use crate::Arg::{Register, Value};
use crate::Operation::{Copy, Decrement, Increment, JumpNotZero, Out};
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let instructions = instructions(input.trim()).unwrap().1;
    let expected = "0101010101010101";
    (0u32..).find(|seed| compute(&instructions, *seed as i32, 16) == *expected)
}

fn compute(instructions: &[Instruction], seed: i32, length: usize) -> String {
    let mut output: Vec<char> = vec![];
    Computer::new(seed).run(instructions, &mut output, length);
    output.iter().collect()
}

/// No actual Part2 today, but I took the chance to optimize the problem based on my input
pub fn part_one_fast(input: &str) -> Option<u32> {
    let instructions = instructions(input.trim()).unwrap().1;
    Some(fast_track(&instructions))
}

fn fast_track(instructions: &[Instruction]) -> u32 {
    let embedded = extract_embedded_seed(instructions);
    let embedded_binary = format!("{embedded:0b}");
    let max = embedded_binary.len() + (embedded_binary.len() & 0b1); //Must be even to have a valid repetition
    let smallest_in_binary: String = (0..=max)
        .map(|i| match i % 2 {
            0 => '0',
            _ => '1',
        })
        .collect();
    u32::from_str_radix(&smallest_in_binary, 2).unwrap() - embedded
}

/// Instruction 1 and 2 are copying values into registers and then multiplying them together
fn extract_embedded_seed(instructions: &[Instruction]) -> u32 {
    match (
        instructions.iter().nth(1).unwrap(),
        instructions.iter().nth(2).unwrap(),
    ) {
        ((Copy, Args(Value(a), _)), (Copy, Args(Value(b), _))) => (a * b) as u32,
        _ => unimplemented!(),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 25);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(1, part_one_fast, input);
}

struct Computer {
    data: HashMap<char, i32>,
}
impl Computer {
    fn new(seed: i32) -> Self {
        Self {
            data: HashMap::from([('a', seed)]),
        }
    }
    fn run(&mut self, instructions: &[Instruction], output: &mut Vec<char>, output_length: usize) {
        let mut ip = 0;

        while let Some(instruction) = instructions.get(ip) {
            match instruction {
                (Copy, Args(Register(from), Some(Register(target)))) => {
                    self.set(*target, self.get(*from));
                }
                (Copy, Args(Value(value), Some(Register(target)))) => {
                    self.set(*target, *value);
                }
                (Increment, Args(Register(register), None)) => {
                    self.inc(*register);
                }
                (Decrement, Args(Register(register), None)) => {
                    self.dec(*register);
                }
                (JumpNotZero, Args(arg1, Some(Value(offset)))) => {
                    let test = match arg1 {
                        Register(register) => self.get(*register),
                        Value(value) => *value,
                    };
                    ip = match (test, *offset) {
                        (0, _) => ip + 1,
                        (_, 0) => ip + 1,
                        (_, steps) if steps < 0 => ip.saturating_sub(steps.unsigned_abs() as usize),
                        (_, steps) => ip + (steps as usize),
                    };
                }
                (Out, Args(Register(register), None)) => match self.get(*register) {
                    0 => output.push('0'),
                    1 => output.push('1'),
                    _ => unimplemented!(),
                },
                _ => {}
            }
            if !matches!(instruction, (JumpNotZero, _)) {
                ip += 1;
            }
            if output.len() >= output_length {
                break;
            }
        }
    }
    fn get(&self, register: char) -> i32 {
        *self.data.get(&register).unwrap_or(&0)
    }
    fn set(&mut self, register: char, value: i32) -> Option<i32> {
        self.data.insert(register, value)
    }
    fn inc(&mut self, register: char) -> Option<i32> {
        self.data.insert(register, self.get(register) + 1)
    }
    fn dec(&mut self, register: char) -> Option<i32> {
        self.data.insert(register, self.get(register) - 1)
    }
}
type Instruction = (Operation, Args);
#[derive(Debug, Clone)]
pub enum Operation {
    Copy,
    Increment,
    Decrement,
    JumpNotZero,
    Out,
}
#[derive(Debug, Clone, Copy)]
pub struct Args(Arg, Option<Arg>);
#[derive(Debug, Clone, Copy)]
enum Arg {
    Register(char),
    Value(i32),
}
mod parse {
    use crate::Arg::{Register, Value};
    use crate::Operation::{Copy, Decrement, Increment, JumpNotZero, Out};
    use crate::{Arg, Args, Instruction, Operation};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{char, i32, line_ending, one_of};
    use nom::combinator::{map, opt};
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, tuple};
    use nom::IResult;

    pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(line_ending, instruction)(input)
    }

    fn instruction(input: &str) -> IResult<&str, Instruction> {
        tuple((operation, preceded(char(' '), args)))(input)
    }
    fn operation(input: &str) -> IResult<&str, Operation> {
        map(
            alt((tag("cpy"), tag("inc"), tag("dec"), tag("jnz"), tag("out"))),
            |operation| match operation {
                "cpy" => Copy,
                "inc" => Increment,
                "dec" => Decrement,
                "jnz" => JumpNotZero,
                "out" => Out,
                _ => unimplemented!(),
            },
        )(input)
    }
    fn args(input: &str) -> IResult<&str, Args> {
        map(
            tuple((arg, opt(preceded(char(' '), arg)))),
            |(arg1, arg2)| Args(arg1, arg2),
        )(input)
    }
    fn arg(input: &str) -> IResult<&str, Arg> {
        alt((arg_value, arg_register))(input)
    }
    fn arg_value(input: &str) -> IResult<&str, Arg> {
        map(i32, Value)(input)
    }
    fn arg_register(input: &str) -> IResult<&str, Arg> {
        map(one_of("abcd"), Register)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one(&input), Some(38));
    }

    #[test]
    fn test_part_one_faster() {
        let input = advent_of_code::read_file("examples", 25);
        assert_eq!(part_one_fast(&input), Some(38));
    }
}
