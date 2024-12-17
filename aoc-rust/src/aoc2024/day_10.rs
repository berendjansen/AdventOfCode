use aoc::set;
use aoc::Solver;
use std::collections::{HashMap, HashSet};

pub struct Solution;

fn neighbours(pos: &(usize, usize), h: usize, w: usize) -> HashSet<(usize, usize)> {
    let (i, j) = pos;
    let mut neighbours = set![];
    if i < &(h - 1) {
        neighbours.insert((i + 1, *j));
    }

    if j < &(w - 1) {
        neighbours.insert((*i, j + 1));
    }

    if i > &0 {
        neighbours.insert((i - 1, *j));
    }

    if j > &0 {
        neighbours.insert((*i, j - 1));
    }
    neighbours
}

fn process_input(
    input: &[&str],
) -> (
    usize,
    usize,
    HashMap<(usize, usize), i64>,
    Vec<(usize, usize)>,
    Vec<(usize, usize)>,
) {
    let (h, w) = (input.len(), input[0].len());
    let mut height_map = HashMap::new();
    let mut top_positions = vec![];
    let mut start_positions = vec![];
    for i in 0..input.len() {
        for (j, c) in input[i].chars().enumerate() {
            let d = match c {
                '.' => 99 as i64,
                _ => c.to_digit(10).unwrap() as i64,
            };
            height_map.insert((i, j), d);
            match d {
                9 => top_positions.push((i, j)),
                0 => start_positions.push((i, j)),
                _ => continue,
            }
        }
    }
    (h, w, height_map, top_positions, start_positions)
}

impl Solver for Solution {
    fn part1(&self, input: &[&str]) -> String {
        let mut trailhead_map = HashMap::new();
        let (h, w, height_map, top_positions, start_positions) = process_input(input);

        for top_position in top_positions {
            let mut positions_to_visit: HashSet<((usize, usize), usize)> = set![];
            for x in neighbours(&top_position, h, w) {
                if height_map.get(&top_position).unwrap() - *height_map.get(&x).unwrap() == 1 {
                    positions_to_visit.insert((x, 1));
                }
            }

            while !positions_to_visit.is_empty() {
                // while let Some((p, steps)) = positions_to_visit.iter().next() {
                let (p, steps) = positions_to_visit.iter().next().unwrap().clone();
                positions_to_visit.remove(&(p, steps));

                trailhead_map
                    .entry(p)
                    .or_insert(set![])
                    .insert((top_position, steps));

                for x in neighbours(&p, h, w) {
                    if height_map.get(&p).unwrap() - *height_map.get(&x).unwrap() == 1 {
                        positions_to_visit.insert((x, steps + 1));
                    }
                }
            }
        }

        let res = start_positions
            .iter()
            .map(|x| trailhead_map.get(&x).map_or(0, |s| s.len()))
            .sum::<usize>();

        format!("{}", res)
    }

    fn part2(&self, input: &[&str]) -> String {
        let (h, w, height_map, top_positions, start_positions) = process_input(input);
        let mut trailhead_map = HashMap::new();

        for top_position in top_positions {
            let mut positions_to_visit: HashSet<((usize, usize), Vec<(usize, usize)>)> = set![];
            for x in neighbours(&top_position, h, w) {
                if height_map.get(&top_position).unwrap() - *height_map.get(&x).unwrap() == 1 {
                    positions_to_visit.insert(((x), vec![top_position, x]));
                }
            }

            while !positions_to_visit.is_empty() {
                let (p, steps) = positions_to_visit.iter().next().unwrap().clone();
                positions_to_visit.remove(&((p), steps.clone()));

                let trailheads_in_this_position = trailhead_map.entry(p).or_insert(set![]);
                trailheads_in_this_position.insert((top_position, steps.clone()));

                for x in neighbours(&p, h, w) {
                    if height_map.get(&p).unwrap() - *height_map.get(&x).unwrap() == 1 {
                        let mut new_steps = steps.clone();
                        new_steps.push(x);
                        positions_to_visit.insert(((x), new_steps));
                    }
                }
            }
        }

        let res = start_positions
            .iter()
            .map(|x| trailhead_map.get(&x).map_or(0, |s| s.len()))
            .sum::<usize>();

        format!("{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbours_all_directions() {
        let n = neighbours(&(2, 3), 5, 5);
        assert_eq!(n, set![(1, 3), (3, 3), (2, 2), (2, 4)])
    }

    #[test]
    fn test_neighbours_edge() {
        let n = neighbours(&(5, 3), 5, 5);
        assert_eq!(n, set![(4, 3), (5, 2), (5, 4)])
    }
}
