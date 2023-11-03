use itertools::Itertools;
use petgraph::algo::{astar, dijkstra};
use petgraph::matrix_graph::Zero;
use petgraph::prelude::UnGraphMap;
use std::hash::Hash;
use std::str::FromStr;

pub fn part_one(input: &str) -> Option<usize> {
    let favorite = u64::from_str(input.trim()).unwrap();
    let from = Coordinate { x: 1, y: 1 };
    let to = Coordinate { x: 31, y: 39 };

    Some(shortest_path(favorite, from, to))
}

fn shortest_path(favorite: u64, from: Coordinate, to: Coordinate) -> usize {
    astar(
        &graph_maze(favorite),
        from,
        |finish| finish == to,
        |_| 1usize,
        |_| 0,
    )
    .unwrap()
    .0
}

/// Build an Undirected Graph of open spaces that is the maze we are navigating
/// Mapped out a 50x50 grid of this maze
fn graph_maze(favorite: u64) -> UnGraphMap<Coordinate, usize> {
    UnGraphMap::from_edges(
        (0u64..=50)
            .cartesian_product(0u64..=50)
            .map(|(x, y)| Coordinate { x, y })
            .filter(|coordinate| coordinate.is_space(favorite))
            .flat_map(|node| {
                node.neighbors()
                    .into_iter()
                    .filter(|&other| other.is_space(favorite))
                    .map(move |other| (node, other, 1usize))
            }),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let favorite = u64::from_str(input.trim()).unwrap();
    let start = Coordinate { x: 1, y: 1 };

    Some(reachable(favorite, 50, start))
}

fn reachable(favorite: u64, steps: usize, start: Coordinate) -> usize {
    dijkstra(&graph_maze(favorite), start, None, |_| 1)
        .into_iter()
        .filter(|(_, distance)| *distance <= steps)
        .count()
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Ord, PartialOrd)]
struct Coordinate {
    x: u64,
    y: u64,
}
impl Coordinate {
    /// Find `x*x + 3*x + 2*x*y + y + y*y`.
    /// - Add the office designer's favorite number (your puzzle input).
    /// - Find the binary representation of that sum; count the number of bits that are 1.
    ///   - If the number of bits that are `1` is even, it's an open space.
    ///   - If the number of bits that are `1` is odd, it's a wall.
    fn is_space(&self, favorite: u64) -> bool {
        (((self.x * self.x)
            + (3 * self.x)
            + (2 * self.x * self.y)
            + self.y
            + (self.y * self.y)
            + favorite)
            .count_ones()
            & 1)
        .is_zero()
    }

    fn neighbors(&self) -> Vec<Coordinate> {
        vec![
            Coordinate {
                x: self.x,
                y: self.y.saturating_sub(1),
            },
            Coordinate {
                x: self.x.saturating_sub(1),
                y: self.y,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y + 1,
            },
        ]
        .into_iter()
        .unique()
        .filter(|c| c != self)
        .collect_vec()
    }
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        let favorite = u64::from_str(input.trim()).unwrap();
        let from = Coordinate { x: 1, y: 1 };
        let to = Coordinate { x: 7, y: 4 };

        assert_eq!(shortest_path(favorite, from, to), 11);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);

        let favorite = u64::from_str(input.trim()).unwrap();
        let start = Coordinate { x: 1, y: 1 };
        assert_eq!(reachable(favorite, 10, start), 18);
    }
}
