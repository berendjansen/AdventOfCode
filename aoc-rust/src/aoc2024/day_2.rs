use aoc::Solver;
use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    fn parse_line(level_string: &str) -> Vec<u64> {
        level_string
            .split(char::is_whitespace)
            .map(|s| str::parse::<u64>(s).expect("Cannot parse string to u64."))
            .collect()
    }

    fn check_level(level: &[u64]) -> bool {
        let mut increasing = false;
        let mut decreasing = false;
        let mut level_it = level.into_iter();
        let mut current_value = level_it.next().expect("empty level");
        while let Some(next_value) = level_it.next() {
            match next_value.cmp(current_value) {
                Ordering::Less => {
                    decreasing = true;
                }
                Ordering::Greater => {
                    increasing = true;
                }
                Ordering::Equal => return false,
            }

            if increasing && decreasing {
                return false;
            }

            let diff = next_value.abs_diff(*current_value);
            if diff < 1 || diff > 3 {
                return false;
            }

            current_value = next_value;
        }
        true
    }

    fn check_level_with_dampener(level: &[u64]) -> bool {
        if Solution::check_level(&level[1..]) || Solution::check_level(&level[..level.len() - 1]) {
            return true;
        }

        level.iter().enumerate().any(|(i, _)| {
            let mut level_clone = vec![0u64; level.len()];
            level_clone.clone_from_slice(level);
            level_clone.remove(i);
            Solution::check_level(&level_clone)
        })
    }
}

impl Solver for Solution {
    fn part1(&self, input: &[&str]) -> String {
        let output = input
            .iter()
            .map(|l| {
                let parsed_line = Solution::parse_line(l);
                Solution::check_level(&parsed_line) as u64
            })
            .sum::<u64>();

        format!("{}", output)
    }
    fn part2(&self, input: &[&str]) -> String {
        let output = input
            .iter()
            .map(|l| {
                let parsed_line = Solution::parse_line(l);
                Solution::check_level_with_dampener(&parsed_line) as u64
            })
            .sum::<u64>();

        format!("{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_level() {
        let level = vec![7, 6, 4, 2, 1];
        assert!(Solution::check_level(&level));
    }

    #[test]
    fn test_unsafe_level() {
        let level = vec![1, 2, 7, 8, 9];
        assert!(!Solution::check_level(&level));
    }

    #[test]
    fn test_unsafe_level_increasing_decreasing() {
        let level = vec![1, 3, 2, 4, 5];
        assert!(!Solution::check_level(&level));
    }

    #[test]
    fn test_safe_level_with_damping() {
        let level = vec![1, 3, 2, 4, 5];
        assert!(Solution::check_level_with_dampener(&level));
    }

    #[test]
    fn test_safe_level_with_damping_2() {
        let level = vec![44, 37, 35, 32, 30];
        assert!(Solution::check_level_with_dampener(&level));
    }

    #[test]
    fn test_unsafe_level_with_damping() {
        let level = vec![24, 22, 24, 27, 30, 32, 35, 39];
        assert!(!Solution::check_level_with_dampener(&level));
    }
}
