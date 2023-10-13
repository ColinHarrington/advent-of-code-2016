use itertools::Itertools;
use nom::bytes::complete::take;
use nom::character::complete::{alpha1, char as nom_char, line_ending, u32 as nom_u32};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        rooms(input)
            .unwrap()
            .1
            .into_iter()
            .filter(is_real)
            .map(|room| room.sector_id)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        rooms(input)
            .unwrap()
            .1
            .into_iter()
            .filter(is_real)
            .filter(|room| room.decrypt() == "northpole object storage")
            .collect_vec()
            .first()
            .unwrap()
            .sector_id,
    )
}

fn is_real(room: &Room) -> bool {
    let sum: String = room
        .name
        .chars()
        .filter(|&c| c != '-')
        .counts()
        .into_iter()
        .sorted_by(|(_, a), (_, b)| b.cmp(a))
        .group_by(|(_, count)| *count)
        .into_iter()
        .flat_map(|(_, group)| group.into_iter().map(|(chr, _)| chr).sorted().collect_vec())
        .take(5)
        .collect();
    sum == room.checksum
}
fn rooms(input: &str) -> IResult<&str, Vec<Room>> {
    separated_list1(line_ending, room)(input)
}

fn room(input: &str) -> IResult<&str, Room> {
    map(
        tuple((
            separated_list1(nom_char('-'), alpha1),
            preceded(nom_char('-'), nom_u32),
            checksum,
        )),
        |(name_parts, sector_id, checksum)| Room {
            name: name_parts.join("-"),
            sector_id,
            checksum,
        },
    )(input)
}

fn checksum(input: &str) -> IResult<&str, String> {
    map(
        delimited(nom_char('['), take(5usize), nom_char(']')),
        |sum: &str| sum.to_string(),
    )(input)
}

#[derive(Debug, Eq, PartialEq)]
struct Room {
    name: String,
    sector_id: u32,
    checksum: String,
}

impl Room {
    fn decrypt(&self) -> String {
        let offset = (self.sector_id % 26) as u8;
        self.name
            .chars()
            .filter(|c| c.is_ascii())
            .map(|c| c as u8)
            .map(|c| match c {
                0x2D => 0x20,
                chr => ((chr - 0x61 + offset) % 26) + 0x61,
            })
            .map(|b| b as char)
            .collect()
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(1514));
    }

    #[test]
    fn test_part_two() {
        let input = "northpole-object-storage-676[oetra]";
        assert_eq!(part_two(input), Some(676));
    }

    #[test]
    fn parsing() {
        let room1 = Room {
            name: "aaaaa-bbb-z-y-x".to_string(),
            sector_id: 123,
            checksum: "abxyz".to_string(),
        };

        assert_eq!(room1.checksum, checksum("[abxyz]").unwrap().1);

        assert_eq!(room1, room("aaaaa-bbb-z-y-x-123[abxyz]").unwrap().1);
    }
}
