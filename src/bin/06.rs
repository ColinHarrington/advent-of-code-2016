use itertools::Itertools;

pub fn part_one(input: &str) -> Option<String> {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let columns = lines.get(0).unwrap().len();
    let eccd: String = (0..columns)
        .map(|i| {
            lines
                .clone()
                .into_iter()
                .map(|line| *line.get(i).unwrap())
                .counts()
                .into_iter()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap()
                .0
        })
        .collect();
    Some(eccd)
}

pub fn part_two(input: &str) -> Option<String> {
    let lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let columns = lines.get(0).unwrap().len();
    let eccd: String = (0..columns)
        .map(|i| {
            lines
                .clone()
                .into_iter()
                .map(|line| *line.get(i).unwrap())
                .counts()
                .into_iter()
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap()
                .0
        })
        .collect();
    Some(eccd)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some("easter".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some("advent".to_string()));
    }
}
