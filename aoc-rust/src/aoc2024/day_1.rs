use std::collections::HashMap;

use aoc::Solver;

pub struct Solution;

impl Solution {
    fn split_columns(input: &[&str]) -> (Vec<u64>, Vec<u64>) {
        let mut left: Vec<u64> = Vec::new();
        let mut right: Vec<u64> = Vec::new();

        for l in input {
            let mut digits = l.split("   ").map(|d| str::parse::<u64>(d).unwrap());
            left.push(digits.next().unwrap());
            right.push(digits.next().unwrap());
        }
        (left, right)
    }
}

impl Solver for Solution {
    fn part1(&self, input: &[&str]) -> String {
        let (mut left, mut right) = Solution::split_columns(input);
        left.sort();
        right.sort();

        let output: u64 = left
            .into_iter()
            .zip(right)
            .map(|(l, r)| l.abs_diff(r))
            .sum();

        format!("{}", output)
    }

    fn part2(&self, input: &[&str]) -> String {
        let (left, right) = Solution::split_columns(input);
        let right = right.into_iter();
        let mut right_counts = HashMap::new();

        for r in right {
            let entry = right_counts.entry(r).or_insert(1);
            *entry += 1;
        }

        let output: u64 = left
            .iter()
            .map(|l| {
                if let Some(value) = right_counts.get(l) {
                    *value * l
                } else {
                    0_u64
                }
            })
            .sum();

        format!("{}", output)
    }
}
