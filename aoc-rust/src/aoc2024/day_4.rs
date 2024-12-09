use aoc::matrix_from_input;
use aoc::Solver;
use fancy_regex::Regex;
use ndarray::Array2;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug)]
enum Increment {
    Add,
    Subtract,
}

impl Increment {
    fn apply(&self, x: usize) -> Option<usize> {
        match self {
            Increment::Add => x.checked_add(1),
            Increment::Subtract => x.checked_sub(1),
        }
    }
}

#[derive(Debug)]
enum Dimension {
    X,
    Y,
}

impl Dimension {
    fn to_index(&self) -> usize {
        match &self {
            Dimension::X => 1,
            Dimension::Y => 0,
        }
    }
}

pub struct Solution;

impl Solution {
    fn get_diagonal_in_dimension(
        mat: &Array2<char>,
        dimension: &Dimension,
        skip_first: bool,
        direction: &Direction,
    ) -> Vec<String> {
        let mut output = Vec::new();
        let start = if skip_first { 1 } else { 0 };
        let (row_increment, col_increment) = match direction {
            Direction::Down => (Increment::Add, Increment::Add),
            Direction::Up => (Increment::Subtract, Increment::Add),
        };

        let outer_index = if let (Dimension::X, Direction::Up) = (dimension, direction) {
            mat.shape()[dimension.to_index()] - 1
        } else {
            0
        };

        // println!(
        //     "With dimension: {:?} and direction {:?}, outer_index: {}",
        //     dimension, direction, outer_index
        // );

        for i in start..mat.shape()[dimension.to_index()] {
            let mut row = match dimension {
                Dimension::X => Some(outer_index),
                Dimension::Y => Some(i),
            };
            let mut col = match dimension {
                Dimension::X => Some(i),
                Dimension::Y => Some(outer_index),
            };
            let mut diag = Vec::new();
            while let Some(value) = mat.get((row.unwrap(), col.unwrap())) {
                diag.push(*value);
                row = row_increment.apply(row.unwrap());
                col = col_increment.apply(col.unwrap());
                if let (Some(_), Some(_)) = (row, col) {
                    continue;
                } else {
                    break;
                }
            }
            if diag.len() > 3 {
                output.push(diag.iter().collect::<String>());
            }
        }
        output
    }

    fn get_diagonals(mat: &Array2<char>) -> Vec<String> {
        let mut output =
            Solution::get_diagonal_in_dimension(&mat, &Dimension::Y, false, &Direction::Down);
        output.append(&mut Solution::get_diagonal_in_dimension(
            &mat,
            &Dimension::X,
            true,
            &Direction::Down,
        ));
        output.append(&mut Solution::get_diagonal_in_dimension(
            &mat,
            &Dimension::Y,
            true,
            &Direction::Up,
        ));
        output.append(&mut Solution::get_diagonal_in_dimension(
            &mat,
            &Dimension::X,
            true,
            &Direction::Up,
        ));
        output
    }

    fn get_flat_dimension(mat: &Array2<char>, dimension: usize) -> Vec<String> {
        let lanes = if dimension == 0 {
            mat.columns()
        } else {
            mat.rows()
        };

        lanes
            .into_iter()
            .map(|r| r.into_iter().collect::<String>())
            .collect::<Vec<String>>()
    }

    fn count_xmax_from_vector_of_strings(v: &Vec<String>) -> usize {
        let re = Regex::new(r"(?=(XMAS))|(?=(SAMX))").expect("Cannot create regex.");
        v.iter()
            .map(|s| {
                let c = re.find_iter(s).count();
                // println!("In string {}: {} matches", s, c);
                c
            })
            .sum()
    }

    fn check_given_positions(
        mat: &Array2<char>,
        positions_and_chars: Vec<(isize, isize, char)>,
        i: usize,
        j: usize,
    ) -> Option<bool> {
        // println!(
        //     "checking position ({}, {}) for {:?}",
        //     i, j, positions_and_chars
        // );
        for (i_inc, j_inc, letter) in positions_and_chars {
            let new_i = if i_inc == 1 {
                i.checked_add(1)?
            } else {
                i.checked_sub(1)?
            };
            let new_j = if j_inc == 1 {
                j.checked_add(1)?
            } else {
                j.checked_sub(1)?
            };
            let c = mat.get((new_i, new_j))?;
            // println!("target: {}, char: {}", letter, c);
            if c != &letter {
                return Some(false);
            }
        }
        // println!("Returning true");
        Some(true)
    }

    fn check_cross_position(mat: &Array2<char>, i: usize, j: usize) -> Option<bool> {
        let positions_and_chars = vec![
            vec![(-1, -1, 'M'), (-1, 1, 'S'), (1, -1, 'M'), (1, 1, 'S')],
            vec![(-1, -1, 'S'), (-1, 1, 'S'), (1, -1, 'M'), (1, 1, 'M')],
            vec![(-1, -1, 'M'), (-1, 1, 'M'), (1, -1, 'S'), (1, 1, 'S')],
            vec![(-1, -1, 'S'), (-1, 1, 'M'), (1, -1, 'S'), (1, 1, 'M')],
        ];

        let is_match = positions_and_chars
            .into_iter()
            .map(|pos| Solution::check_given_positions(mat, pos, i, j))
            .any(|x| x == Some(true));

        // println!("Found match on {}, {}; {}", i, j, is_match);
        Some(is_match)
    }
}

impl Solver for Solution {
    fn part1(&self, input: &[&str]) -> String {
        let mat = matrix_from_input(input);

        let diags = Solution::get_diagonals(&mat);
        let horizontals = Solution::get_flat_dimension(&mat, 1);
        let verticals = Solution::get_flat_dimension(&mat, 0);

        let res: usize = vec![diags, horizontals, verticals]
            .iter()
            .map(|e| Solution::count_xmax_from_vector_of_strings(e))
            .sum();

        format!("{}", res)
    }

    fn part2(&self, input: &[&str]) -> String {
        let mat = matrix_from_input(input);
        let mut res = 0;
        for i in 0..mat.shape()[0] {
            for j in 0..mat.shape()[1] {
                if let Some('A') = mat.get((i, j)) {
                    match Solution::check_cross_position(&mat, i, j) {
                        Some(true) => res += 1,
                        _ => continue,
                    }
                }
            }
        }
        format!("{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let input = input.split("\n").into_iter().collect::<Vec<&str>>();
        let solver = Solution;
        assert_eq!(solver.part1(&input), "18")
    }
}
