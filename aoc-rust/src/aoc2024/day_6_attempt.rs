use aoc::{matrix_from_input, Solver};
use ndarray::Array2;
use std::collections::{HashMap, HashSet};

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

    fn to_char(&self) -> char {
        match self {
            Direction::Left => '-',
            Direction::Up => '|',
            Direction::Right => '-',
            Direction::Down => '|',
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
        let (i, j) = current_position;

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

    fn get_obstruction_locations(grid: &Array2<char>) -> HashSet<(usize, usize)> {
        let mut positions = HashSet::new();
        for ((i, j), c) in grid.indexed_iter() {
            if c == &'#' {
                positions.insert((i, j));
            }
        }
        positions
    }

    fn positions_to_the_right(
        grid: &Array2<char>,
        direction: Direction,
        current_position: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let (i, j) = current_position;
        match direction.next_direction() {
            Direction::Up => (0..i).rev().map(|x| (x, j)).collect(),
            Direction::Down => (i..grid.shape()[0]).map(|x| (x, j)).collect(),
            Direction::Right => (j..grid.shape()[1]).map(|x| (i, x)).collect(),
            Direction::Left => (0..j).rev().map(|x| (i, x)).collect(),
        }
    }

    fn position_before_first_obstacle(
        positions: Vec<(usize, usize)>,
        obstacles: &HashSet<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        for w in positions.windows(2) {
            if obstacles.contains(&w[1]) {
                println!("Found obstacle at {:?}, returning {:?}", w[1], w[0]);
                return Some(w[0]);
            }
        }
        None
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
        let grid = matrix_from_input(input);
        let mut current_position =
            Solution::find_start_position(&grid).expect("Cannot find starting position.");
        let mut direction = Direction::Up;
        let obstacles = Solution::get_obstruction_locations(&grid);

        let mut visited_positions: HashMap<(usize, usize), Direction> = HashMap::new();
        visited_positions.insert(current_position, direction.clone());

        let mut path: HashMap<(usize, usize), char> = HashMap::new();
        path.insert(current_position, direction.to_char());

        let mut potential_loop_count: usize = 0;
        let mut potential_loop_positions: Vec<(usize, usize)> = vec![];

        let mut diagram: Array2<char> = Array2::from_elem(grid.raw_dim(), '.');

        while let (Some((i, j)), next_direction) =
            Solution::make_step(&grid, current_position, direction)
        {
            visited_positions.insert((i, j), next_direction.clone());
            let lane_to_the_right =
                Solution::positions_to_the_right(&grid, next_direction.clone(), (i, j));
            println!("Current pos: {}, {}", i, j);
            // println!("Lane to the right: {:?}", lane_to_the_right);

            if let (Some((i_before_ob, j_before_ob)), (Some(to_place_i), Some(to_place_j))) = (
                Solution::position_before_first_obstacle(lane_to_the_right, &obstacles),
                Solution::next_position_in_direction((i, j), &next_direction),
            ) {
                if visited_positions.contains_key(&(i_before_ob, j_before_ob)) {
                    potential_loop_positions.push((to_place_i, to_place_j));
                    println!(
                        "Added {}, {} as block by looking forward because I visited {}, {}",
                        to_place_i, to_place_j, i_before_ob, j_before_ob
                    );
                    potential_loop_count += 1;
                    *diagram.get_mut((to_place_i, to_place_j)).unwrap() = '0';
                }
            }

            // check positions behind, if obstacle behind me, and there is an obstacle right front of that obstacle, and there is an obstacle in front of me, a block can be placed to the front right of the obstacle in front of me
            // let lane_behind =

            current_position = (i, j);
            direction = next_direction;
            path.insert(current_position, direction.to_char());

            for ((i, j), _) in grid.indexed_iter() {
                let d_entry = diagram.get_mut((i, j)).unwrap();
                *d_entry = if let Some(c) = path.get(&(i, j)) {
                    *c
                } else if d_entry == &'0' {
                    continue;
                } else {
                    '.'
                };
            }

            for (i, j) in obstacles.clone().into_iter() {
                let x = diagram
                    .get_mut((i, j))
                    .expect("Cannot get element from diagram.");
                *x = '#';
            }

            for r in diagram.rows() {
                println!("{:?}", r.into_iter().collect::<String>());
            }
            println!("\n");
        }

        let block_count = potential_loop_positions.len();

        for (i, j) in potential_loop_positions.into_iter() {
            let x = diagram
                .get_mut((i, j))
                .expect("Cannot get element from diagram.");
            *x = '0';
        }

        println!("{:?}", block_count);
        format!("{}", potential_loop_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_last_before_obstruction() {
        let mut obstacles = HashSet::new();
        obstacles.insert((4, 5));

        let positions = vec![(1, 5), (2, 5), (3, 5), (4, 5), (5, 5)];
        assert_eq!(
            Solution::position_before_first_obstacle(positions, &obstacles),
            Some((3, 5))
        );
    }

    #[test]
    fn test_get_last_before_obstruction_no_obstacles() {
        let mut obstacles = HashSet::new();
        obstacles.insert((4, 6));

        let positions = vec![(1, 5), (2, 5), (3, 5), (4, 5), (5, 5)];
        assert_eq!(
            Solution::position_before_first_obstacle(positions, &obstacles),
            None
        );
    }
}
