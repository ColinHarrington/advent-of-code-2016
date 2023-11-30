use itertools::Itertools;
use nom::AsChar;
use petgraph::algo::astar;
use petgraph::graphmap::UnGraphMap;
use std::collections::HashMap;
use std::iter::once;

pub fn part_one(input: &str) -> Option<usize> {
    Some(Maze::from(input.trim()).fewest_steps())
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(Maze::from(input.trim()).fewest_steps_and_back())
}

fn manhattan_distance(from: (usize, usize), to: (usize, usize)) -> usize {
    from.0.max(to.0) - from.0.min(to.0) + from.1.max(to.1) - from.1.min(to.1)
}
struct Maze {
    data: Vec<Vec<char>>,
    columns: usize,
    rows: usize,
    locations: HashMap<usize, (usize, usize)>,
}

impl Maze {
    fn from(input: &str) -> Self {
        let data = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        let rows = data.len();
        let columns = data.get(0).unwrap().len();

        let locations = HashMap::from_iter((0..rows).cartesian_product(0..columns).filter_map(
            |(row, column)| match data[row][column] {
                c if c.is_dec_digit() => Some((c.to_digit(32).unwrap() as usize, (row, column))),
                _ => None,
            },
        ));
        Self {
            data,
            columns,
            rows,
            locations,
        }
    }

    fn open_neighbors(&self, row: usize, column: usize) -> Vec<(usize, usize)> {
        vec![
            (row.saturating_sub(1), column),
            (row, column.saturating_sub(1)),
            (row, column + 1),
            (row + 1, column),
        ]
        .into_iter()
        .unique()
        .filter(|(r, c)| !(*r == row && *c == column) && *r < self.rows && *c < self.columns)
        .filter(|(r, c)| self.data[*r][*c] != '#')
        .collect_vec()
    }

    fn space_graph(&self) -> UnGraphMap<(usize, usize), usize> {
        UnGraphMap::from_edges(
            (0..self.rows)
                .cartesian_product(0..self.columns)
                .filter_map(|(row, column)| match self.data[row][column] {
                    '#' => None,
                    _ => Some((row, column)),
                })
                .flat_map(|(row, column)| {
                    self.open_neighbors(row, column)
                        .into_iter()
                        .map(|neighbor| ((row, column), neighbor))
                        .collect_vec()
                }),
        )
    }

    fn shortest_paths(&self) -> HashMap<(usize, usize), usize> {
        let graph = self.space_graph();
        let mut paths: HashMap<(usize, usize), usize> =
            HashMap::from_iter(self.locations.keys().map(|id| ((*id, *id), 0)));

        self.locations
            .iter()
            .tuple_combinations()
            .for_each(|((a, from), (b, to))| {
                let steps = astar(
                    &graph,
                    *from,
                    |node| node == *to,
                    |_| 1,
                    |node| manhattan_distance(*to, node),
                )
                .unwrap()
                .0;
                paths.insert((*a, *b), steps);
                paths.insert((*b, *a), steps);
            });
        paths
    }

    fn destinations(&self) -> Vec<usize> {
        self.locations
            .keys()
            .filter(|k| **k != 0)
            .copied()
            .sorted()
            .collect_vec()
    }

    fn fewest_steps(&self) -> usize {
        let paths = self.shortest_paths();

        self.destinations()
            .iter()
            .permutations(self.locations.len() - 1)
            .map(|scenario| {
                once(&0)
                    .chain(scenario)
                    .tuple_windows()
                    .map(|(a, b)| paths.get(&(*a, *b)).unwrap())
                    .sum::<usize>()
            })
            .min()
            .unwrap_or(0)
    }

    fn fewest_steps_and_back(&self) -> usize {
        let paths = self.shortest_paths();

        self.destinations()
            .iter()
            .permutations(self.locations.len() - 1)
            .map(|scenario| {
                once(&0)
                    .chain(scenario)
                    .chain(once(&0))
                    .tuple_windows()
                    .map(|(a, b)| paths.get(&(*a, *b)).unwrap())
                    .sum::<usize>()
            })
            .min()
            .unwrap_or(0)
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(14));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), None);
    }
}
