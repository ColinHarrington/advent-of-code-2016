use nom::character::complete::{char as nom_char, u32 as nom_u32};
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

/// 120766 => too high
pub fn part_one(input: &str) -> Option<usize> {
    Some(decompress(input.trim()))
}

fn decompress(input: &str) -> usize {
    let mut tail = input;
    let mut decompressed = 0;
    while !tail.is_empty() {
        decompressed += match marker(tail) {
            Ok((data, marker)) => {
                (_, tail) = data.split_at(marker.characters);
                marker.characters * marker.repetitions
            }
            Err(_) => {
                (_, tail) = tail.split_at(1);
                1
            }
        }
    }
    decompressed
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(decompress_v2(input.trim()))
}

fn decompress_v2(input: &str) -> usize {
    let mut tail = input;
    let mut decompressed = 0;
    while !tail.is_empty() {
        decompressed += match marker(tail) {
            Ok((data, marker)) => {
                let (head, remainder) = data.split_at(marker.characters);
                tail = remainder;
                marker.repetitions * decompress_v2(head)
            }
            Err(_) => {
                tail = tail.split_at(1).1;
                1
            }
        }
    }
    decompressed
}

fn marker(input: &str) -> IResult<&str, Marker> {
    map(
        separated_pair(
            preceded(nom_char('('), nom_u32),
            nom_char('x'),
            terminated(nom_u32, nom_char(')')),
        ),
        |(c, r)| Marker {
            characters: c as usize,
            repetitions: r as usize,
        },
    )(input)
}
struct Marker {
    characters: usize,
    repetitions: usize,
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(57));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_example("09b");
        assert_eq!(part_two(&input), Some(242394));
    }

    #[test]
    fn decompression() {
        assert_eq!(6, decompress("ADVENT"));
        assert_eq!(7, decompress("A(1x5)BC"));
        assert_eq!(9, decompress("(3x3)XYZ"));
        assert_eq!(11, decompress("A(2x2)BCD(2x2)EFG"));
        assert_eq!(6, decompress("(6x1)(1x3)A"));
        assert_eq!(18, decompress("X(8x2)(3x3)ABCY"));
    }

    #[test]
    fn decompression2() {
        assert_eq!(9, decompress_v2("(3x3)XYZ"));
        assert_eq!(20, decompress_v2("X(8x2)(3x3)ABCY"));
        assert_eq!(241920, decompress_v2("(27x12)(20x12)(13x14)(7x10)(1x12)A"));
        assert_eq!(
            445,
            decompress_v2("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
        );
    }
}
