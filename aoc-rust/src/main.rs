use std::io::{self, Read};

use aoc::Solver;

mod aoc2024;

fn main() -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let lines: Vec<&str> = input.lines().collect();
    let solver = aoc2024::day_3::Solution;

    println!("Part1: {}", solver.part1(&lines));
    println!("Part2: {}", solver.part2(&lines));

    Ok(())
}
