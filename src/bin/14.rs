use itertools::Itertools;
use std::collections::{BTreeMap, VecDeque};

pub fn part_one(input: &str) -> Option<usize> {
    let salt = input.trim();
    one_time_pads(salt, 0)
}
pub fn part_two(input: &str) -> Option<usize> {
    let salt = input.trim();
    one_time_pads(salt, 2016)
}

fn one_time_pads(salt: &str, stretch: usize) -> Option<usize> {
    let n = 64usize;
    let mut buffer = VecDeque::new();

    let mut one_time_pads: BTreeMap<usize, (usize, u128, u128)> = BTreeMap::new();
    let mut max_index: Option<usize> = None;
    let mut index = 0usize;

    while max_index.is_none() || index <= max_index.unwrap() {
        let sum = stretch_key(format!("{salt}{index}"), stretch);
        // println!("{index}: {sum:0x}");
        buffer.push_back(sum);

        if let Some(pattern) = five(&sum) {
            threes(pattern & 0xFFF, &buffer)
                .into_iter()
                .filter(|(offset, _)| *offset != 0)
                .map(|(offset, triple)| (index - offset, triple))
                .for_each(|(i3, triple)| {
                    one_time_pads.insert(i3, (index, sum, triple));
                });

            if max_index.is_none() && one_time_pads.len() >= n {
                let max = one_time_pads
                    .keys()
                    .sorted()
                    .nth(n - 1)
                    .map(|idx| *idx + 1000usize)
                    .unwrap();
                max_index = Some(max);
            }
        }

        if buffer.len() > 1000 {
            buffer.pop_front();
        }
        index += 1
    }
    one_time_pads.keys().sorted().nth(n - 1).copied()
}

fn stretch_key(salt: String, stretch: usize) -> u128 {
    u128::from_be_bytes(
        (0usize..stretch)
            .fold(md5::compute(salt), |digest, _| {
                md5::compute(format!("{digest:0x}"))
            })
            .0,
    )
}
fn threes(pattern: u128, buffer: &VecDeque<u128>) -> Vec<(usize, u128)> {
    buffer
        .iter()
        .enumerate()
        .take(1000)
        .filter(|(_, triplet)| three(triplet, pattern))
        .map(|(offset, triplet)| (buffer.len() - offset - 1, *triplet))
        .collect_vec()
}

fn three(data: &u128, pattern: u128) -> bool {
    (0usize..=29)
        .rev()
        .map(|i| i * 4)
        .map(|shift| (data >> shift) & 0xFFF)
        .find(|nibbles| {
            matches!(
                nibbles,
                0x000
                    | 0x111
                    | 0x222
                    | 0x333
                    | 0x444
                    | 0x555
                    | 0x666
                    | 0x777
                    | 0x888
                    | 0x999
                    | 0xaaa
                    | 0xbbb
                    | 0xccc
                    | 0xddd
                    | 0xeee
                    | 0xfff
            )
        })
        == Some(pattern)
}

fn five(data: &u128) -> Option<u128> {
    (0usize..=27)
        .map(|i| i * 4)
        .map(|shift| (data >> shift) & 0xFFFFF)
        .find(|nibbles| {
            matches!(
                nibbles,
                0x00000
                    | 0x11111
                    | 0x22222
                    | 0x33333
                    | 0x44444
                    | 0x55555
                    | 0x66666
                    | 0x77777
                    | 0x88888
                    | 0x99999
                    | 0xaaaaa
                    | 0xbbbbb
                    | 0xccccc
                    | 0xddddd
                    | 0xeeeee
                    | 0xfffff
            )
        })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(22728));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(22551));
    }

    #[test]
    fn test_five() {
        let nope = 0x9ff21004ba41a085cedce94dbe717557u128;
        let has_three = 0x347dac6ee8eeea4652c7476d0f97bee5u128;
        let has_five = 0x3aeeeee1367614f3061d165a5fe3cac3u128;
        assert_eq!(None, five(&nope));
        assert_eq!(None, five(&has_three));
        assert_eq!(Some(0xEEEEE), five(&has_five));
    }

    #[test]
    fn test_22728() {
        let i3 = 22728usize;
        let digest = md5::compute(format!("abc{i3}"));
        let sum = u128::from_be_bytes(digest.0);
        let expected = u128::from_str_radix("26ccc731a8706e0c4f979aeb341871f0", 16).unwrap();
        assert_eq!(expected, sum);
    }
    #[test]
    fn stretch() {
        let expected = u128::from_str_radix("a107ff634856bb300138cac6568c0f24", 16).unwrap();
        assert_eq!(stretch_key("abc0".to_string(), 2016), expected);
    }
}
