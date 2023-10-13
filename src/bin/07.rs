use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete::{alpha1, char as nom_char, line_ending};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use nom::sequence::delimited;
use nom::IResult;

pub fn part_one(input: &str) -> Option<usize> {
    Some(
        parse_ips(input)
            .unwrap()
            .1
            .iter()
            .filter(|ip| ip.supports_tls())
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(
        parse_ips(input)
            .unwrap()
            .1
            .iter()
            .filter(|ip| ip.supports_ssl())
            .count(),
    )
}

fn can_abba(data: &str) -> bool {
    data.len() >= 4
        && data.as_bytes().windows(4).any(|window| {
            window.len() == 4
                && window[0] != window[1]
                && window[0] == window[3]
                && window[1] == window[2]
        })
}

fn aba_sequences(data: &str) -> Vec<[u8; 3]> {
    data.as_bytes()
        .windows(3)
        .filter(|window| window.len() == 3 && window[0] == window[2] && window[0] != window[1])
        .map(|window| [window[0], window[1], window[2]])
        .collect_vec()
}

fn has_bab(data: &str, bab: [u8; 3]) -> bool {
    data.as_bytes().windows(3).any(|window| window == bab)
}

fn parse_ips(input: &str) -> IResult<&str, Vec<IP7>> {
    separated_list1(line_ending, parse_ip7)(input)
}
fn parse_ip7(input: &str) -> IResult<&str, IP7> {
    map(many1(segment), IP7::new)(input)
}

fn segment(input: &str) -> IResult<&str, Segment> {
    alt((segmented_unbracketed, segmented_bracket))(input)
}

fn segmented_unbracketed(input: &str) -> IResult<&str, Segment> {
    map(alpha1, Segment::unbracketed)(input)
}

fn segmented_bracket(input: &str) -> IResult<&str, Segment> {
    map(
        delimited(nom_char('['), alpha1, nom_char(']')),
        Segment::bracketed,
    )(input)
}

struct Segment {
    bracketed: bool,
    data: String,
}

impl Segment {
    fn bracketed(data: &str) -> Self {
        Self {
            bracketed: true,
            data: data.to_string(),
        }
    }
    fn unbracketed(data: &str) -> Self {
        Self {
            bracketed: false,
            data: data.to_string(),
        }
    }
}
struct IP7 {
    segments: Vec<Segment>,
}

impl IP7 {
    fn new(segments: Vec<Segment>) -> Self {
        Self { segments }
    }

    fn supports_tls(&self) -> bool {
        self.hypernets().iter().all(|hypernet| !can_abba(hypernet))
            && self.segments.iter().any(|segment| can_abba(&segment.data))
    }

    fn supernets(&self) -> Vec<String> {
        self.segments
            .iter()
            .filter(|segment| !segment.bracketed)
            .map(|segment| segment.data.clone())
            .collect_vec()
    }

    fn hypernets(&self) -> Vec<String> {
        self.segments
            .iter()
            .filter(|segment| segment.bracketed)
            .map(|segment| segment.data.clone())
            .collect_vec()
    }

    fn supports_ssl(&self) -> bool {
        let hypernets = self.hypernets();
        self.supernets()
            .iter()
            .flat_map(|supernet| {
                aba_sequences(supernet)
                    .into_iter()
                    .map(|aba| [aba[1], aba[0], aba[1]])
                    .collect_vec()
            })
            .any(|bab| hypernets.iter().any(|hypernet| has_bab(hypernet, bab)))
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_example("07b");
        assert_eq!(part_two(&input), Some(3));
    }
}
