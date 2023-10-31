use itertools::Itertools;
use std::iter::once;

pub fn part_one(input: &str) -> Option<usize> {
    Some(space_count(input.trim(), 40))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(space_count(input.trim(), 400000))
}

fn space_count(line: &str, rows: usize) -> usize {
    let mut prev = line.chars().map(|c| c == '^').collect_vec();
    let mut count = prev.iter().filter(|&t| !(*t)).count();
    for _ in 1..rows {
        prev = once(prev[1])
            .chain(prev.windows(3).map(|window| {
                matches!(
                    window,
                    [true, true, false]
                        | [false, true, true]
                        | [true, false, false]
                        | [false, false, true]
                )
            }))
            .chain(once(prev[prev.len() - 2]))
            .collect_vec();
        count += prev.iter().filter(|&trap| !(*trap)).count()
    }
    count
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(space_count(&input.trim(), 10), 38);
        assert_eq!(part_one(&input), Some(185));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(1935478));
    }
}
