use crate::parse::nodes;
use itertools::Itertools;
use petgraph::algo::astar;
use petgraph::prelude::UnGraphMap;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
    let nodes = nodes(input.trim()).unwrap().1;
    Some(
        nodes
            .clone()
            .into_iter()
            .cartesian_product(nodes)
            .filter(|(a, b)| a.used > 0 && a != b && a.used <= b.available())
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let nodes = nodes(input.trim()).unwrap().1;
    let empty = *nodes.iter().find(|node| node.used == 0).unwrap();
    let node_map: HashMap<(u32, u32), &Node> =
        HashMap::from_iter(nodes.iter().map(|node| ((node.x, node.y), node)));

    let target = *node_map
        .get(
            node_map
                .keys()
                .filter(|(_, y)| *y == 0)
                .sorted_by_key(|(x, _)| x)
                .next_back()
                .unwrap(),
        )
        .unwrap();

    let graph: UnGraphMap<Node, usize> = UnGraphMap::from_edges(
        nodes
            .iter()
            .filter(|node| node.used < 100)
            .flat_map(|node| {
                node.neighbors()
                    .into_iter()
                    .filter_map(|neighbor| node_map.get(&neighbor))
                    .filter(|&other| other.size < 100 && other.size > node.used)
                    .map(|&other| (*node, *other))
            }),
    );
    let goal = *node_map.get(&(target.x - 1, 0)).unwrap();
    let steps = astar(&graph, empty, |n| n == *goal, |_| 1, |_| 0)
        .unwrap()
        .0;

    Some(goal.x * 5 + 1 + steps)
}

type Position = (u32, u32);
#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct Node {
    x: u32,
    y: u32,
    size: u32,
    used: u32,
}
impl Node {
    fn available(&self) -> u32 {
        self.size - self.used
    }
    fn neighbors(&self) -> Vec<Position> {
        vec![
            (self.x, self.y.saturating_sub(1)),
            (self.x.saturating_sub(1), self.y),
            (self.x + 1, self.y),
            (self.x, self.y + 1),
        ]
        .into_iter()
        .unique()
        .filter(|&position| position != (self.x, self.y))
        .collect_vec()
    }
}
mod parse {
    use crate::Node;
    use nom::bytes::complete::tag;
    use nom::character::complete::{line_ending, multispace1, space1, u32};
    use nom::combinator::map;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, separated_pair, terminated, tuple};
    use nom::IResult;

    pub fn nodes(input: &str) -> IResult<&str, Vec<Node>> {
        preceded(
            separated_pair(
                tag(r"root@ebhq-gridcenter# df -h"),
                line_ending,
                separated_pair(tag("Filesystem"), space1, tag("Size  Used  Avail  Use%\n")),
            ),
            separated_list1(line_ending, node),
        )(input)
    }
    fn node(input: &str) -> IResult<&str, Node> {
        map(
            tuple((
                preceded(tag("/dev/grid/node-x"), u32),
                preceded(tag("-y"), u32),
                preceded(multispace1, terminated(u32, tag("T"))),
                preceded(multispace1, terminated(u32, tag("T"))),
                preceded(multispace1, terminated(u32, tag("T"))),
                preceded(multispace1, terminated(u32, tag("%"))),
            )),
            |(x, y, size, used, _, _)| Node { x, y, size, used },
        )(input)
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), Some(7));
    }
}
