use aoc2024::helpers::helpers::read_lines;
use std::cmp::Ordering;

fn main() {
    let input = match read_lines("input/day2.txt") {
        Ok(lines) => lines,
        Err(e) => panic!("Error reading input file: {}", e),
    };

    let parsed_input = input
        .into_iter()
        .map(|l| parse_line(l.unwrap()))
        .collect::<Vec<Vec<u64>>>();

    let sol1 = parsed_input
        .iter()
        .map(|level| check_level(level) as u64)
        .sum::<u64>();

    println!("Solution part 1: {}", sol1);

    let sol2 = parsed_input
        .iter()
        .map(|level| check_level_with_dampener(level) as u64)
        .sum::<u64>();

    println!("Solution part 2: {}", sol2);
}

fn parse_line(level_string: String) -> Vec<u64> {
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
    if check_level(&level[1..]) {
        return true;
    } else if check_level(&level[..level.len() - 1]) {
        return true;
    }

    if level
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let mut level_clone = vec![0u64; level.len()];
            level_clone.clone_from_slice(level);
            level_clone.remove(i);
            check_level(&level_clone) as u8
        })
        .sum::<u8>()
        > 0
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_level() {
        let level = vec![7, 6, 4, 2, 1];
        assert!(check_level(&level));
    }

    #[test]
    fn test_unsafe_level() {
        let level = vec![1, 2, 7, 8, 9];
        assert!(!check_level(&level));
    }

    #[test]
    fn test_unsafe_level_increasing_decreasing() {
        let level = vec![1, 3, 2, 4, 5];
        assert!(!check_level(&level));
    }

    #[test]
    fn test_safe_level_with_damping() {
        let level = vec![1, 3, 2, 4, 5];
        assert!(check_level_with_dampener(&level));
    }

    #[test]
    fn test_safe_level_with_damping_2() {
        let level = vec![44, 37, 35, 32, 30];
        assert!(check_level_with_dampener(&level));
    }

    #[test]
    fn test_unsafe_level_with_damping() {
        let level = vec![24, 22, 24, 27, 30, 32, 35, 39];
        assert!(!check_level_with_dampener(&level));
    }
}
