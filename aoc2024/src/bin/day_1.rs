use aoc2024::helpers::helpers::read_lines;
use std::collections::HashMap;

fn main() {
    let input = match read_lines("input/day1.txt") {
        Ok(lines) => lines,
        Err(e) => panic!("Error reading input file: {}", e),
    };

    let mut left: Vec<u64> = Vec::new();
    let mut right: Vec<u64> = Vec::new();

    for l in input {
        if let Ok(s) = l {
            let mut digits = s.split("   ").map(|d| str::parse::<u64>(d).unwrap());
            left.push(digits.next().unwrap());
            right.push(digits.next().unwrap());
        }
    }

    let sol1 = part1(left.clone(), right.clone());
    let sol2 = part2(left, right);

    println!("Solution part 1: {}", sol1);
    println!("Solution part 2: {}", sol2);
}

fn part1(mut left: Vec<u64>, mut right: Vec<u64>) -> u64 {
    left.sort();
    right.sort();

    let output: u64 = left
        .into_iter()
        .zip(right.into_iter())
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    output
}

fn part2(left: Vec<u64>, right: Vec<u64>) -> u64 {
    let right = right.into_iter();
    let mut right_counts = HashMap::new();

    for r in right {
        let entry = right_counts.entry(r).or_insert(1);
        *entry += 1;
    }

    left.iter()
        .map(|l| {
            if let Some(value) = right_counts.get(l) {
                *value * l
            } else {
                0 as u64
            }
        })
        .sum()
}
