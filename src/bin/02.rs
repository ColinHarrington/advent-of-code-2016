use std::ops::{Add, Sub};
use std::str::Chars;

pub fn part_one(input: &str) -> Option<String> {
    let mut keypad = SquareKeypad::new();
    let combo: String = input
        .lines()
        .map(|line| keypad.execute_steps(line.chars()))
        .collect();
    Some(combo)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut keypad = DiamondKeypad::new();
    let combo: String = input
        .lines()
        .map(|line| keypad.execute_steps(line.chars()))
        .collect();
    Some(combo)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

type Key = (u8, u8);

struct SquareKeypad {
    key: Key,
}

impl SquareKeypad {
    fn new() -> Self {
        Self { key: (1, 1) }
    }

    fn execute_steps(&mut self, chars: Chars) -> char {
        for direction in chars {
            self.translate(direction);
        }
        self.value()
    }

    fn translate(&mut self, direction: char) {
        let (row, col) = self.key;
        self.key = match direction {
            'U' => (row.saturating_sub(1), col),
            'D' => (row.add(1).min(2), col),
            'L' => (row, col.saturating_sub(1)),
            'R' => (row, col.add(1).min(2)),
            _ => unimplemented!(),
        }
    }

    fn value(&self) -> char {
        match self.key {
            (0, 0) => '1',
            (0, 1) => '2',
            (0, 2) => '3',
            (1, 0) => '4',
            (1, 1) => '5',
            (1, 2) => '6',
            (2, 0) => '7',
            (2, 1) => '8',
            (2, 2) => '9',
            _ => panic!("Invalid Key"),
        }
    }
}

#[derive(Debug)]
struct DiamondKeypad {
    key: Key,
}

impl DiamondKeypad {
    fn new() -> Self {
        Self { key: (2, 0) }
    }

    fn execute_steps(&mut self, chars: Chars) -> char {
        for direction in chars {
            self.translate(direction);
        }
        self.value()
    }

    fn translate(&mut self, direction: char) {
        self.key = match direction {
            'U' => match self.key {
                (2, 0) | (1, 1) | (0, 2) | (1, 3) | (2, 4) => self.key,
                _ => (self.key.0.sub(1), self.key.1),
            },
            'D' => match self.key {
                (2, 0) | (3, 1) | (4, 2) | (3, 3) | (2, 4) => self.key,
                _ => (self.key.0.add(1), self.key.1),
            },
            'L' => match self.key {
                (0, 2) | (1, 1) | (2, 0) | (3, 1) | (4, 2) => self.key,
                _ => (self.key.0, self.key.1.sub(1)),
            },
            'R' => match self.key {
                (0, 2) | (1, 3) | (2, 4) | (3, 3) | (4, 2) => self.key,
                _ => (self.key.0, self.key.1.add(1)),
            },
            _ => panic!("invalid key"),
        };
    }

    /// ```
    ///     1
    ///   2 3 4
    /// 5 6 7 8 9
    ///   A B C
    ///     D
    /// ```
    fn value(&mut self) -> char {
        match self.key {
            (0, 2) => '1',
            (1, 1) => '2',
            (1, 2) => '3',
            (1, 3) => '4',
            (2, 0) => '5',
            (2, 1) => '6',
            (2, 2) => '7',
            (2, 3) => '8',
            (2, 4) => '9',
            (3, 1) => 'A',
            (3, 2) => 'B',
            (3, 3) => 'C',
            (4, 2) => 'D',
            _ => panic!("Invalid Key"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some("1985".to_string()));
    }

    #[test]
    fn translation() {
        let scenarios = [
            ((0, 0), (0, 0), 'U'),
            ((0, 0), (1, 0), 'D'),
            ((0, 0), (0, 0), 'L'),
            ((0, 0), (0, 1), 'R'),
            ((2, 2), (1, 2), 'U'),
            ((2, 2), (2, 2), 'D'),
            ((2, 2), (2, 1), 'L'),
            ((2, 2), (2, 2), 'R'),
        ];

        for (key, expected, direction) in scenarios {
            let mut keypad = SquareKeypad { key };
            keypad.translate(direction);
            assert_eq!(expected, keypad.key);
        }
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some("5DB3".to_string()));
    }
}
