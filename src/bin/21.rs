use crate::Instruction::{
    MovePosition, Reverse, RotateLeft, RotateLetter, RotateRight, SwapLetters, SwapPositions,
};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, line_ending, u32 as nom_u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;
use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<String> {
    Some(scramble("abcdefgh", instructions(input.trim()).unwrap().1))
}
fn scramble(seed: &str, instructions: Vec<Instruction>) -> String {
    let mut password = VecDeque::from_iter(seed.chars());
    for instruction in instructions {
        match instruction {
            SwapPositions(a, b) => password.swap(a, b),
            SwapLetters(a, b) => password.swap(
                password.iter().position(|&c| c == a).unwrap(),
                password.iter().position(|&c| c == b).unwrap(),
            ),
            RotateRight(steps) => password.rotate_right(steps),
            RotateLeft(steps) => password.rotate_left(steps),
            RotateLetter(letter) => {
                let position = password.iter().position(|&c| c == letter).unwrap();
                password.rotate_right(position + 1);
                if position >= 4 {
                    password.rotate_right(1);
                }
            }
            Reverse(start, end) => {
                let section = password
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i >= start && *i <= end)
                    .map(|(_, c)| *c)
                    .rev()
                    .collect_vec();
                for (i, c) in section.into_iter().enumerate() {
                    password[start + i] = c;
                }
            }
            MovePosition(from, to) => {
                let letter = password.remove(from).unwrap();
                password.insert(to, letter)
            }
        }
    }
    password.iter().collect()
}

pub fn part_two(input: &str) -> Option<String> {
    Some(unscramble(
        "fbgdceah",
        instructions(input.trim()).unwrap().1,
    ))
}

fn unscramble(seed: &str, instructions: Vec<Instruction>) -> String {
    let mut password = VecDeque::from_iter(seed.chars());
    for instruction in instructions.into_iter().rev() {
        match instruction {
            SwapPositions(a, b) => password.swap(a, b),
            SwapLetters(a, b) => password.swap(
                password.iter().position(|&c| c == a).unwrap(),
                password.iter().position(|&c| c == b).unwrap(),
            ),
            RotateRight(steps) => password.rotate_left(steps),
            RotateLeft(steps) => password.rotate_right(steps),
            RotateLetter(letter) => {
                password.rotate_left(
                    match password.iter().position(|&c| c == letter).unwrap() {
                        0 => 9,
                        1 => 1,
                        2 => 6,
                        3 => 2,
                        4 => 7,
                        5 => 3,
                        6 => 8,
                        7 => 4,
                        _ => unimplemented!("password size not handled"),
                    } % password.len(),
                );
            }
            Reverse(start, end) => {
                let section = password
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i >= start && *i <= end)
                    .map(|(_, c)| *c)
                    .rev()
                    .collect_vec();
                for (i, c) in section.into_iter().enumerate() {
                    password[start + i] = c;
                }
            }
            MovePosition(from, to) => {
                let letter = password.remove(to).unwrap();
                password.insert(from, letter)
            }
        }
    }
    password.iter().collect()
}

#[derive(Debug)]
enum Instruction {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    RotateRight(usize),
    RotateLeft(usize),
    RotateLetter(char),
    Reverse(usize, usize),
    MovePosition(usize, usize),
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}
fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        swap_positions,
        swap_letters,
        rotate_right,
        rotate_left,
        rotate_letter,
        reverse,
        move_position,
    ))(input)
}
fn swap_positions(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("swap position "),
            separated_pair(nom_u32, tag(" with position "), nom_u32),
        ),
        |(a, b)| SwapPositions(a as usize, b as usize),
    )(input)
}

fn swap_letters(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("swap letter "),
            separated_pair(anychar, tag(" with letter "), anychar),
        ),
        |(a, b)| SwapLetters(a, b),
    )(input)
}

fn rotate_right(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(tag("rotate right "), terminated(nom_u32, steps)),
        |step| RotateRight(step as usize),
    )(input)
}
fn steps(input: &str) -> IResult<&str, &str> {
    alt((tag(" steps"), tag(" step")))(input)
}
fn rotate_left(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(tag("rotate left "), terminated(nom_u32, steps)),
        |step| RotateLeft(step as usize),
    )(input)
}
fn rotate_letter(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(tag("rotate based on position of letter "), anychar),
        RotateLetter,
    )(input)
}
fn reverse(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("reverse positions "),
            separated_pair(nom_u32, tag(" through "), nom_u32),
        ),
        |(a, b)| Reverse(a as usize, b as usize),
    )(input)
}
fn move_position(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("move position "),
            separated_pair(nom_u32, tag(" to position "), nom_u32),
        ),
        |(a, b)| MovePosition(a as usize, b as usize),
    )(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        let (tail, instructions) = instructions(input.trim()).unwrap();
        assert_eq!("", tail);
        assert_eq!("decab".to_string(), scramble("abcde", instructions));
        assert_eq!(part_one(&input), Some("fbdecgha".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some("efghdabc".to_string()));
    }
}
