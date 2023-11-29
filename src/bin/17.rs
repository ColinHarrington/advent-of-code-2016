use itertools::Itertools;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

pub fn part_one(input: &str) -> Option<String> {
    shortest_path(input.trim())
}
fn shortest_path(seed: &str) -> Option<String> {
    let mut queue = VecDeque::from([State::new()]);

    let mut shortest = None;
    while let Some(state) = queue.pop_front() {
        if state.row == 3 && state.column == 3 {
            shortest = Some(state.path);
            break;
        }
        options(format!("{seed}{}", state.path))
            .into_iter()
            .filter_map(|dir| state.step(dir))
            .for_each(|next| queue.push_back(next));
    }

    shortest
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(longest_path(input.trim()))
}

fn longest_path(seed: &str) -> usize {
    let mut queue = VecDeque::from([State::new()]);

    let mut longest = 0usize;
    while let Some(state) = queue.pop_front() {
        if state.row == 3 && state.column == 3 {
            if state.path.len() > longest {
                longest = state.path.len();
            }
            continue;
        }
        options(format!("{seed}{}", state.path))
            .into_iter()
            .filter_map(|dir| state.step(dir))
            .for_each(|next| queue.push_back(next));
    }

    longest
}

#[derive(Debug, Clone)]
struct State {
    path: String,
    row: i8,
    column: i8,
}
impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}) {}", self.row, self.column, self.path)
    }
}

impl State {
    fn new() -> Self {
        Self {
            path: "".to_string(),
            row: 0,
            column: 0,
        }
    }
    fn step(&self, direction: char) -> Option<State> {
        match direction {
            'U' if self.row != 0 => Some((self.row - 1, self.column)),
            'D' if self.row < 3 => Some((self.row + 1, self.column)),
            'L' if self.column != 0 => Some((self.row, self.column - 1)),
            'R' if self.column < 3 => Some((self.row, self.column + 1)),
            _ => None,
        }
        .map(|(row, column)| State {
            path: format!("{}{direction}", self.path),
            row,
            column,
        })
    }
}

fn options(seed: String) -> Vec<char> {
    format!("{:x}", md5::compute(seed.as_bytes()))
        .chars()
        .take(4)
        .enumerate()
        .filter_map(|(i, c)| match c {
            'b' | 'c' | 'd' | 'e' | 'f' => match i {
                0 => Some('U'),
                1 => Some('D'),
                2 => Some('L'),
                3 => Some('R'),
                _ => unreachable!(),
            },
            _ => None,
        })
        .collect_vec()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one("ihgpwlah"), Some("DDRRRD".to_string()));
        assert_eq!(part_one("kglvqrro"), Some("DDUDRLRRUDRD".to_string()));
        assert_eq!(
            part_one("ulqzkmiv"),
            Some("DRURDRUDDLLDLUURRDULRLDUUDDDRR".to_string())
        );
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two("ihgpwlah"), Some(370));
        assert_eq!(part_two("kglvqrro"), Some(492));
        assert_eq!(part_two("ulqzkmiv"), Some(830));
    }
}
