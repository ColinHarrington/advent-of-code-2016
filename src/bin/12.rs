use crate::Instruction::{CopyRegister, CopyValue, Decrement, Increment, JumpNotZero};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{
    anychar, char as nom_char, i32 as nom_i32, line_ending, u32 as nom_u32,
};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;

pub fn part_one(input: &str) -> Option<u32> {
    let mut computer = Computer::new();
    computer.execute_program(instructions(input).unwrap().1);
    Some(computer.registers.a)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut computer = Computer {
        registers: Registers {
            a: 0,
            b: 0,
            c: 1,
            d: 0,
        },
    };
    computer.execute_program(instructions(input).unwrap().1);
    Some(computer.registers.a)
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}
fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        copy_value,
        copy_register,
        increment,
        decrement,
        jump_not_zero,
    ))(input)
}
fn copy_register(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(tag("cpy "), separated_pair(anychar, nom_char(' '), anychar)),
        |(from, to)| CopyRegister(from, to),
    )(input)
}
fn copy_value(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(tag("cpy "), separated_pair(nom_u32, nom_char(' '), anychar)),
        |(value, register)| CopyValue(value, register),
    )(input)
}
fn increment(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("inc "), anychar), Increment)(input)
}
fn decrement(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("dec "), anychar), Decrement)(input)
}
fn jump_not_zero(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(tag("jnz "), separated_pair(anychar, nom_char(' '), nom_i32)),
        |(register, offset)| JumpNotZero(register, offset),
    )(input)
}

#[derive(Debug)]
enum Instruction {
    CopyRegister(char, char),
    CopyValue(u32, char),
    Increment(char),
    Decrement(char),
    JumpNotZero(char, i32),
}

struct Computer {
    registers: Registers,
}

impl Computer {
    fn new() -> Self {
        Computer {
            registers: Registers {
                a: 0,
                b: 0,
                c: 0,
                d: 0,
            },
        }
    }

    fn execute_program(&mut self, instructions: Vec<Instruction>) {
        let mut instruction_pointer: i32 = 0;
        while let Some(instruction) = instructions.get(instruction_pointer as usize) {
            match instruction {
                CopyRegister(from, to) => self.registers.copy_register(*from, *to),
                CopyValue(value, register) => self.registers.copy_value(*value, *register),
                Increment(register) => self.registers.increment(*register),
                Decrement(register) => self.registers.decrement(*register),
                JumpNotZero(target, offset) => {
                    let value = if target.is_ascii_digit() {
                        u32::from(*target)
                    } else {
                        self.registers.value(*target)
                    };
                    instruction_pointer += if value != 0 { *offset } else { 1 }
                }
            }
            if !matches!(instruction, JumpNotZero(_, _)) {
                instruction_pointer += 1
            }
        }
    }
}

struct Registers {
    a: u32,
    b: u32,
    c: u32,
    d: u32,
}

impl Registers {
    fn increment(&mut self, register: char) {
        match register {
            'a' => self.a += 1,
            'b' => self.b += 1,
            'c' => self.c += 1,
            'd' => self.d += 1,
            _ => panic!("invalid register"),
        }
    }

    fn decrement(&mut self, register: char) {
        match register {
            'a' => self.a -= 1,
            'b' => self.b -= 1,
            'c' => self.c -= 1,
            'd' => self.d -= 1,
            _ => panic!("invalid register"),
        }
    }

    fn copy_value(&mut self, value: u32, register: char) {
        match register {
            'a' => self.a = value,
            'b' => self.b = value,
            'c' => self.c = value,
            'd' => self.d = value,
            _ => panic!("invalid register"),
        }
    }

    fn copy_register(&mut self, from: char, to: char) {
        match to {
            'a' => self.a = self.value(from),
            'b' => self.b = self.value(from),
            'c' => self.c = self.value(from),
            'd' => self.d = self.value(from),
            _ => panic!("invalid register"),
        }
    }

    fn value(&self, register: char) -> u32 {
        match register {
            'a' => self.a,
            'b' => self.b,
            'c' => self.c,
            'd' => self.d,
            _ => panic!("invalid register"),
        }
    }
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(42));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(42));
    }
}
