use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<String> {
    let door_id = input.trim();
    let password: String = (0u64..u64::MAX)
        .into_iter()
        .filter_map(|i| match md5::compute(format!("{door_id}{i}")).0 {
            [0, 0, third, ..] if third <= 15 => Some(third),
            _ => None,
        })
        .map(|b| format!("{b:x}"))
        .take(8)
        .collect();

    Some(password)
}

pub fn part_two(input: &str) -> Option<String> {
    let door_id = input.trim();

    let password: String = (0u64..u64::MAX)
        .into_iter()
        .filter_map(|i| match md5::compute(format!("{door_id}{i}")).0 {
            [0, 0, third, fourth, ..] if third <= 7 => Some((third, fourth >> 4)),
            _ => None,
        })
        .fold_while([None; 8], |mut password, (i, b)| {
            let idx = i as usize;
            if password[idx].is_none() {
                password[idx] = Some(format!("{b:x}").chars().next().unwrap())
            }
            if password.iter().any(|position| position.is_none()) {
                Continue(password)
            } else {
                Done(password)
            }
        })
        .into_inner()
        .into_iter()
        .filter_map(|c| c)
        .collect();

    Some(password)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("18f47a30".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("05ace8e3".to_string()));
    }
}
