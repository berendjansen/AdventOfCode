use aoc::{matrix_from_input, Solver};
use ndarray::Array2;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn next_direction(&self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }
}

pub struct Solution;

impl Solution {
    fn find_start_position(grid: &Array2<char>) -> Option<(usize, usize)> {
        for ((i, j), c) in grid.indexed_iter() {
            if c == &'^' {
                return Some((i, j));
            }
        }
        None
    }

    fn next_position_in_direction(
        current_position: (usize, usize),
        direction: &Direction,
    ) -> (Option<usize>, Option<usize>) {
        let (i, j) = current_position;
        match direction {
            Direction::Left => (Some(i), j.checked_sub(1)),
            Direction::Right => (Some(i), j.checked_add(1)),
            Direction::Up => (i.checked_sub(1), Some(j)),
            Direction::Down => (i.checked_add(1), Some(j)),
        }
    }
    fn make_step(
        grid: &Array2<char>,
        current_position: (usize, usize),
        direction: Direction,
    ) -> (Option<(usize, usize)>, Direction) {
        let potential_next_position =
            Solution::next_position_in_direction(current_position, &direction);

        let (i, j) = match potential_next_position {
            (Some(i), Some(j)) => (i, j),
            _ => return (None, direction),
        };

        let next_char = match grid.get((i, j)) {
            Some(c) => c,
            None => return (None, direction),
        };

        if next_char != &'#' {
            (Some((i, j)), direction)
        } else {
            Solution::make_step(grid, current_position, direction.next_direction())
        }
    }

    fn let_guard_walk(grid: &Array2<char>) -> Option<HashSet<(usize, usize, Direction)>> {
        let mut visited_positions: HashSet<(usize, usize, Direction)> = HashSet::new();
        let mut direction = Direction::Up;
        let mut current_position =
            Solution::find_start_position(grid).expect("Cannot find starting position.");
        visited_positions.insert((current_position.0, current_position.1, direction.clone()));

        while let (Some((i, j)), next_direction) =
            Solution::make_step(grid, current_position, direction)
        {
            if !visited_positions.insert((i, j, next_direction.clone())) {
                return None;
            };
            current_position = (i, j);
            direction = next_direction;
        }
        Some(visited_positions)
    }
}

impl Solver for Solution {
    fn part1(&self, input: &[&str]) -> String {
        let grid = matrix_from_input(input);
        let mut current_position =
            Solution::find_start_position(&grid).expect("Cannot find starting position.");
        let mut visited_positions: HashSet<(usize, usize)> = HashSet::new();
        let mut direction = Direction::Up;

        while let (Some((i, j)), next_direction) =
            Solution::make_step(&grid, current_position, direction)
        {
            visited_positions.insert((i, j));
            current_position = (i, j);
            direction = next_direction;
        }

        format!("{}", visited_positions.len())
    }

    fn part2(&self, input: &[&str]) -> String {
        let mut res = 0;
        let grid = matrix_from_input(input);
        let visited_positions = Solution::let_guard_walk(&grid)
            .expect("Encountered loop.")
            .into_iter()
            .map(|(a, b, _)| (a, b))
            .collect::<HashSet<(usize, usize)>>()
            .into_iter()
            .collect::<Vec<(usize, usize)>>();
        let start = Solution::find_start_position(&grid).expect("Cannot find starting position.");

        for (i, j) in visited_positions {
            // for ((i, j), c) in grid.indexed_iter() {
            if (i, j) != start {
                let mut modified_grid = grid.clone();
                *modified_grid.get_mut((i, j)).unwrap() = '#';
                match Solution::let_guard_walk(&modified_grid) {
                    Some(_) => continue,
                    None => res += 1,
                }
                println!("{}", res);
            }
        }

        format!("{}", res)
    }
}

#[cfg(test)]
mod tests {}
