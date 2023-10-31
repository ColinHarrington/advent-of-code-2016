use std::str::FromStr;

pub fn part_one(input: &str) -> Option<u32> {
    let elves = u32::from_str(input.trim()).unwrap();
    Some(josephus(elves))
}
/// Binary solution to the [Josephus problem](https://en.wikipedia.org/wiki/Josephus_problem)
/// Next power of two shifted >> 1 is the power of two under count.
/// 2^n + 2l+1 but done in binary
fn josephus(count: u32) -> u32 {
    (count ^ (count.next_power_of_two() >> 1)) << 1 | 1
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves = u32::from_str(input.trim()).unwrap();
    Some(josephus_across(elves))
}

/// Powers of three result in that position winning
/// After the power of three, it goes up by 1 until 2x the previous power of three,
/// Then it goes up by two until the next power of three
fn josephus_across(count: u32) -> u32 {
    match 3u32.pow(count.ilog(3)) {
        three if three == count => count,
        three if count <= 2 * three => count - three,
        three => count - three + count - 2 * three,
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(2));
    }
}
