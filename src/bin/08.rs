use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char as nom_char, line_ending, u32 as nom_u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::fmt::{Display, Formatter};

pub fn part_one(input: &str) -> Option<usize> {
    let mut screen = Screen::new(8, 50);
    for instruction in instructions(input).unwrap().1 {
        screen.execute(instruction);
    }
    Some(screen.lit_pixels())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut screen = Screen::new(6, 50);
    for instruction in instructions(input).unwrap().1 {
        screen.execute(instruction);
    }
    Some(format!("{screen}"))
}

fn instructions(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, instruction)(input)
}
fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((rect, rotate_row, rotate_column))(input)
}

fn rect(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("rect "),
            separated_pair(nom_u32, nom_char('x'), nom_u32),
        ),
        |(w, h)| Instruction::Rect(w as usize, h as usize),
    )(input)
}

fn rotate_row(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("rotate row y="),
            separated_pair(nom_u32, tag(" by "), nom_u32),
        ),
        |(w, h)| Instruction::RotateRow(w as usize, h as usize),
    )(input)
}

fn rotate_column(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("rotate column x="),
            separated_pair(nom_u32, tag(" by "), nom_u32),
        ),
        |(w, h)| Instruction::RotateColumn(w as usize, h as usize),
    )(input)
}

#[derive(Debug, Eq, PartialEq)]
enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}
impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Rect(w, h) => write!(f, "rect {w}x{h}"),
            Instruction::RotateRow(row, amount) => write!(f, "rotate row y={row} by {amount}"),
            Instruction::RotateColumn(column, amount) => {
                write!(f, "rotate column x={column} by {amount}")
            }
        }
    }
}
struct Screen {
    data: Vec<Vec<bool>>,
}

impl Screen {
    fn new(rows: usize, columns: usize) -> Self {
        Self {
            data: (0..rows)
                .map(|_| (0..columns).map(|_| false).collect_vec())
                .collect_vec(),
        }
    }

    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Rect(width, height) => self.draw_rectangle(width, height),
            Instruction::RotateRow(row, amount) => self.rotate_row(row, amount),
            Instruction::RotateColumn(column, amount) => self.rotate_column(column, amount),
        }
    }

    fn draw_rectangle(&mut self, width: usize, height: usize) {
        for row in 0..height {
            for col in 0..width {
                self.data[row][col] = true;
            }
        }
    }

    fn rotate_row(&mut self, row: usize, amount: usize) {
        self.data[row].rotate_right(amount)
    }

    fn rotate_column(&mut self, column: usize, amount: usize) {
        let mut data = self.data.iter().map(|row| row[column]).collect_vec();
        data.rotate_right(amount);
        for (row, value) in data.into_iter().enumerate() {
            self.data[row][column] = value
        }
    }

    fn lit_pixels(&self) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|&v| *v).count())
            .sum()
    }
}

impl Display for Screen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let lines: Vec<String> = self
            .data
            .iter()
            .enumerate()
            .map(|(_, row)| {
                row.iter()
                    .map(|pixel| if *pixel { '█' } else { ' ' })
                    .collect()
            })
            .collect_vec();
        writeln!(f, "{}", lines.join("\n"))
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Instruction::{Rect, RotateColumn, RotateRow};

    #[test]
    fn parsing() {
        assert_eq!(Rect(1, 8), instruction("rect 1x8").unwrap().1);
        assert_eq!(
            RotateRow(0, 4),
            instruction("rotate row y=0 by 4").unwrap().1
        );
        assert_eq!(
            RotateColumn(13, 1),
            instruction("rotate column x=13 by 1").unwrap().1
        );
    }
}
