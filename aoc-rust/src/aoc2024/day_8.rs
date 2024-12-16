use aoc::matrix_from_input;
use aoc::{set, Solver};
use itertools::Itertools;
use ndarray::Array2;
use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    fn get_char_positions(grid: Array2<char>) -> HashMap<char, Vec<(usize, usize)>> {
        let mut position_map = HashMap::new();
        for ((i, j), c) in grid.indexed_iter() {
            if c != &'.' {
                position_map.entry(*c).or_insert(Vec::new()).push((i, j))
            }
        }
        position_map
    }

    fn antinode_positions(
        pos_a: (usize, usize),
        pos_b: (usize, usize),
        height: usize,
        width: usize,
    ) -> HashSet<(usize, usize)> {
        let pos_a = (pos_a.0 as isize, pos_a.1 as isize);
        let pos_b = (pos_b.0 as isize, pos_b.1 as isize);
        let diff_y = pos_a.0 - pos_b.0;
        let diff_x = pos_a.1 - pos_b.1;

        let antinode_1 = ((pos_a.0 + diff_y), (pos_a.1 + diff_x));
        let antinode_2 = ((pos_b.0 - diff_y), (pos_b.1 - diff_x));

        convert_tuples_to_isize_and_filter(&[antinode_1, antinode_2])
            .into_iter()
            .filter(|(i, j)| i < &height && j < &width)
            .collect()
    }

    fn t_freq_antinode_positions(
        pos_a: (usize, usize),
        pos_b: (usize, usize),
        height: usize,
        width: usize,
    ) -> HashSet<(usize, usize)> {
        let pos_a = (pos_a.0 as isize, pos_a.1 as isize);
        let pos_b = (pos_b.0 as isize, pos_b.1 as isize);
        let diff_y = pos_a.0 - pos_b.0;
        let diff_x = pos_a.1 - pos_b.1;

        let antinodes: Vec<(isize, isize)> = (0..height)
            .flat_map(|i| {
                [
                    (
                        (pos_a.0 + i as isize * diff_y),
                        (pos_a.1 + i as isize * diff_x),
                    ),
                    (
                        (pos_b.0 - i as isize * diff_y),
                        (pos_b.1 - i as isize * diff_x),
                    ),
                ]
            })
            .collect();

        convert_tuples_to_isize_and_filter(&antinodes)
            .into_iter()
            .filter(|(i, j)| i < &height && j < &width)
            .collect()
    }
}

fn convert_tuples_to_isize_and_filter(input: &[(isize, isize)]) -> Vec<(usize, usize)> {
    input
        .iter()
        .filter_map(|&(x, y)| match (x.try_into(), y.try_into()) {
            (Ok(converted_x), Ok(converted_y)) => Some((converted_x, converted_y)),
            _ => None,
        })
        .collect()
}
impl Solver for Solution {
    fn part1(&self, input: &[&str]) -> String {
        let (height, width) = (input.len(), input[0].len());
        let grid = matrix_from_input(input);
        let char_positions = Solution::get_char_positions(grid);
        let mut antinode_positions: HashSet<(usize, usize)> = set![];

        for (_, positions) in char_positions.into_iter() {
            for c in positions.into_iter().combinations(2) {
                for an in Solution::antinode_positions(c[0], c[1], height, width) {
                    antinode_positions.insert(an);
                }
            }
        }

        format!("{}", antinode_positions.len())
    }
    fn part2(&self, input: &[&str]) -> String {
        let (height, width) = (input.len(), input[0].len());
        let grid = matrix_from_input(input);
        let char_positions = Solution::get_char_positions(grid);
        let mut antinode_positions: HashSet<(usize, usize)> = set![];

        for (_, positions) in char_positions.into_iter() {
            for c in positions.into_iter().combinations(2) {
                for an in Solution::t_freq_antinode_positions(c[0], c[1], height, width) {
                    antinode_positions.insert(an);
                }
            }
        }

        format!("{}", antinode_positions.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_antinode_positions_from_two_points() {
        let point_a = (4, 2);
        let point_b = (5, 3);
        let antinodes = set![(3, 1), (6, 4)];
        assert_eq!(
            Solution::antinode_positions(point_a, point_b, 10, 10),
            antinodes
        )
    }

    #[test]
    fn test_antinode_positions_from_two_points_rev() {
        let point_b = (4, 2);
        let point_a = (5, 3);
        let antinodes = set![(3, 1), (6, 4)];
        assert_eq!(
            Solution::antinode_positions(point_a, point_b, 10, 10),
            antinodes
        )
    }

    #[test]
    fn test_antinode_positions_from_two_points_one_oob() {
        let point_a = (4, 2);
        let point_b = (5, 3);
        let antinodes = set![(3, 1)];
        assert_eq!(
            Solution::antinode_positions(point_a, point_b, 5, 10),
            antinodes
        )
    }

    #[test]
    fn test_t_freq_antinode_positions_from_two_points() {
        let point_a = (4, 2);
        let point_b = (5, 3);
        let antinodes = set![
            (2, 0),
            (3, 1),
            (6, 4),
            (7, 5),
            (8, 6),
            (9, 7),
            (4, 2),
            (5, 3)
        ];
        assert_eq!(
            Solution::t_freq_antinode_positions(point_a, point_b, 10, 10),
            antinodes
        )
    }

    #[test]
    fn test_t_freq_antinode_positions_from_two_points_rev() {
        let point_b = (4, 2);
        let point_a = (5, 3);
        let antinodes = set![
            (2, 0),
            (3, 1),
            (6, 4),
            (7, 5),
            (8, 6),
            (9, 7),
            (4, 2),
            (5, 3)
        ];
        assert_eq!(
            Solution::t_freq_antinode_positions(point_a, point_b, 10, 10),
            antinodes
        )
    }

    #[test]
    fn test_t_freq_antinode_positions_from_two_points_2() {
        let point_b = (5, 2);
        let point_a = (4, 3);
        let antinodes = set![
            (3, 4),
            (2, 5),
            (1, 6),
            (0, 7),
            (6, 1),
            (7, 0),
            (5, 2),
            (4, 3)
        ];
        assert_eq!(
            Solution::t_freq_antinode_positions(point_a, point_b, 10, 10),
            antinodes
        )
    }
}
