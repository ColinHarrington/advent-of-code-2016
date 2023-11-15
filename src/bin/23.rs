use crate::parse::instructions;
use crate::Argument::{Register, Value};
use crate::Arguments::{Binary, Unary};
use crate::Instruction::*;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

pub fn part_one(input: &str) -> Option<i32> {
    let instructions = instructions(input).unwrap().1;

    Some(Computer::new(instructions, 7).run())
}

pub fn part_two(input: &str) -> Option<i32> {
    let instructions = instructions(input).unwrap().1;

    Some(Computer::new(instructions, 12).run())
}

#[derive(Debug, Clone)]
pub enum Arguments {
    Unary(Argument),
    Binary(Argument, Argument),
}

#[derive(Debug, Clone)]
pub enum Argument {
    Value(i32),
    Register(char),
}

#[derive(Debug)]
struct Computer {
    instructions: VecDeque<Instruction>,
    registers: HashMap<char, i32>,
}
impl Computer {
    fn new(instructions: Vec<Instruction>, a: i32) -> Self {
        Self {
            instructions: VecDeque::from_iter(instructions),
            registers: HashMap::from([('a', a)]),
        }
    }

    fn run(&mut self) -> i32 {
        self.optimize();

        let mut ip = 0;

        while let Some(instruction) = self.instructions.get(ip) {
            match instruction {
                JumpNotZero(Binary(arg1, arg2)) => {
                    let value = match arg1 {
                        Value(val) => *val,
                        Register(register) => self.get(*register),
                    };
                    ip = match value {
                        0 => ip + 1,
                        _ => ip_at(
                            ip,
                            match arg2 {
                                Value(value) => *value,
                                Register(register) => self.get(*register),
                            },
                        ),
                    };
                }
                instruction => {
                    match instruction {
                        Copy(Binary(arg1, Register(register))) => self.set(
                            *register,
                            match arg1 {
                                Value(val) => *val,
                                Register(register) => self.get(*register),
                            },
                        ),
                        Increment(Unary(Register(register))) => {
                            self.set(*register, self.get(*register) + 1)
                        }
                        Decrement(Unary(Register(register))) => {
                            self.set(*register, self.get(*register) - 1)
                        }
                        Mul(Binary(Register(r1), Register(r2))) => {
                            self.set('a', self.get(*r1) * self.get(*r2))
                        }
                        Toggle(Unary(Register(register))) => {
                            let target_idx = ip_at(ip, self.get(*register));
                            if let Some(target) = self.instructions.remove(target_idx) {
                                self.instructions.insert(
                                    target_idx,
                                    match target {
                                        Copy(args) => JumpNotZero(args.clone()),
                                        JumpNotZero(args) => Copy(args.clone()),
                                        Increment(args) => Decrement(args.clone()),
                                        Decrement(args) => Increment(args.clone()),
                                        Toggle(args) => Increment(args.clone()),
                                        i => i,
                                    },
                                );
                                self.optimize();
                            }
                        }
                        _ => {}
                    }
                    ip += 1;
                }
            }
        }
        self.get('a')
    }
    fn get(&self, register: char) -> i32 {
        *self.registers.get(&register).unwrap_or(&0i32)
    }
    fn set(&mut self, register: char, value: i32) {
        self.registers.insert(register, value);
    }

    /// Multiplication via manual addition.
    /// The following is the equivalent of `a += (b * c)`
    ///
    /// ```
    /// cpy b c
    /// inc a
    /// dec c
    /// jnz c -2
    /// dec d
    /// jnz d -5
    ///```
    fn optimize(&mut self) {
        let instructions = self.instructions.iter().collect_vec().clone();

        let multiplications = instructions
            .windows(6)
            .enumerate()
            .filter_map(|(idx, window)| {
                match (
                    window[0], window[1], window[2], window[3], window[4], window[5],
                ) {
                    (
                        Copy(Binary(Register(x1), Register(t1))),
                        Increment(Unary(Register('a'))),
                        Decrement(Unary(Register(t2))),
                        JumpNotZero(Binary(Register(t3), Value(-2))),
                        Decrement(Unary(Register(z1))),
                        JumpNotZero(Binary(Register(z2), Value(-5))),
                    ) if t1 == t2 && t2 == t3 && z1 == z2 => Some((idx, *x1, *t1, *z1)),
                    _ => None,
                }
            })
            .collect_vec();
        for (idx, x, t, z) in multiplications {
            self.instructions[idx] = Mul(Binary(Register(x), Register(z)));
            self.instructions[idx + 1] = Copy(Binary(Value(0), Register(t)));
            self.instructions[idx + 2] = Copy(Binary(Value(0), Register(z)));
            self.instructions[idx + 3] = Nop;
            self.instructions[idx + 4] = Nop;
            self.instructions[idx + 5] = Nop;
        }
    }
}

fn ip_at(ip: usize, offset: i32) -> usize {
    if offset < 0 {
        ip.saturating_sub(offset.unsigned_abs() as usize)
    } else {
        ip + (offset as usize)
    }
}
#[derive(Debug, Clone)]
pub enum Instruction {
    Copy(Arguments),
    Increment(Arguments),
    Decrement(Arguments),
    JumpNotZero(Arguments),
    Toggle(Arguments),
    Mul(Arguments),
    Nop,
}

mod parse {
    use crate::Argument::{Register, Value};
    use crate::Arguments::Unary;
    use crate::Instruction::{Copy, Decrement, Increment, JumpNotZero, Toggle};
    use crate::{Argument, Arguments, Instruction};
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{anychar, char as nom_char, i32 as nom_i32, line_ending};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair};
    use nom::IResult;
    use Arguments::Binary;

    pub fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
        separated_list1(line_ending, instruction)(input)
    }
    fn instruction(input: &str) -> IResult<&str, Instruction> {
        alt((copy, increment, decrement, jump_not_zero, toggle))(input)
    }
    fn copy(input: &str) -> IResult<&str, Instruction> {
        map(preceded(tag("cpy "), args), Copy)(input)
    }
    fn increment(input: &str) -> IResult<&str, Instruction> {
        map(preceded(tag("inc "), args), Increment)(input)
    }
    fn decrement(input: &str) -> IResult<&str, Instruction> {
        map(preceded(tag("dec "), args), Decrement)(input)
    }
    fn jump_not_zero(input: &str) -> IResult<&str, Instruction> {
        map(preceded(tag("jnz "), args), JumpNotZero)(input)
    }
    fn toggle(input: &str) -> IResult<&str, Instruction> {
        map(preceded(tag("tgl "), args), Toggle)(input)
    }

    fn args(input: &str) -> IResult<&str, Arguments> {
        alt((two_args, single_args))(input)
    }
    fn single_args(input: &str) -> IResult<&str, Arguments> {
        map(argument, Unary)(input)
    }
    fn two_args(input: &str) -> IResult<&str, Arguments> {
        map(
            separated_pair(argument, nom_char(' '), argument),
            |(arg1, arg2)| Binary(arg1, arg2),
        )(input)
    }
    fn argument(input: &str) -> IResult<&str, Argument> {
        alt((value, register))(input)
    }
    fn value(input: &str) -> IResult<&str, Argument> {
        map(nom_i32, Value)(input)
    }
    fn register(input: &str) -> IResult<&str, Argument> {
        map(anychar, Register)(input)
    }
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(3));
    }
}
