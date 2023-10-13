use itertools::Itertools;
use nom::character::complete::{line_ending, multispace0, u32 as u32_nom};
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::IResult;

pub fn part_one(input: &str) -> Option<usize> {
    let triples = triples(input).unwrap().1;
    Some(triples.iter().filter(|triple| can_triangle(triple)).count())
}

fn can_triangle((a, b, c): &Triple) -> bool {
    let ordered: [u32; 3] = vec![a, b, c]
        .into_iter()
        .copied()
        .sorted()
        .collect_vec()
        .try_into()
        .unwrap();

    (ordered[0] + ordered[1]) > ordered[2]
}

pub fn part_two(input: &str) -> Option<usize> {
    let triples: Vec<Triple> = triples(input)
        .unwrap()
        .1
        .chunks(3)
        .flat_map(|chunk| {
            let [a, b, c] = chunk else {
                panic!("Uneven chunks of three")
            };
            vec![(a.0, b.0, c.0), (a.1, b.1, c.1), (a.2, b.2, c.2)]
        })
        .collect_vec();
    Some(triples.iter().filter(|triple| can_triangle(triple)).count())
}

type Triple = (u32, u32, u32);
fn triples(input: &str) -> IResult<&str, Vec<Triple>> {
    separated_list1(line_ending, triple)(input)
}
fn triple(input: &str) -> IResult<&str, Triple> {
    tuple((triple_entry, triple_entry, triple_entry))(input)
}
fn triple_entry(input: &str) -> IResult<&str, u32> {
    preceded(multispace0, u32_nom)(input)
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_example("03b");
        assert_eq!(part_two(&input), Some(6));
    }
}
