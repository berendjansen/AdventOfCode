use aoc::Solver;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Operation {
    Plus,
    Mul,
    Concat,
}

impl Operation {
    fn apply(&self, a: i64, b: i64) -> i64 {
        match self {
            Operation::Plus => a + b,
            Operation::Mul => a * b,
            Operation::Concat => str::parse::<i64>(&format!("{}{}", a, b)).unwrap(),
        }
    }
}

pub struct Solution;

impl Solution {
    fn parse_input(input: &[&str]) -> Vec<(i64, Vec<i64>)> {
        input
            .iter()
            .map(|s| {
                let split_res: Vec<&str> = s.split(": ").collect();
                let result = str::parse::<i64>(split_res[0]).expect("Cannot parse result to i64.");
                let v = split_res[1]
                    .split(" ")
                    .map(|x| str::parse::<i64>(x).expect("Cannot parse to i64."))
                    .collect();
                (result, v)
            })
            .collect()
    }

    fn fix_equation(res: i64, digits: &[i64], allowed_operators: Vec<Operation>) -> Option<i64> {
        let sets_of_operations = (1..digits.len())
            .map(|_| allowed_operators.clone())
            .multi_cartesian_product();

        for set_of_operations in sets_of_operations {
            let mut operations_iterator = set_of_operations.iter();
            let result: i64 = digits
                .to_owned()
                .clone()
                .into_iter()
                .reduce(|a, b| match operations_iterator.next() {
                    Some(op) => op.apply(a, b),
                    None => panic!("Reached end of iterators before end of digits."),
                })
                .unwrap();

            if result == res {
                return Some(result);
            }
        }
        None
    }
}

impl Solver for Solution {
    fn part1(&self, input: &[&str]) -> String {
        let parsed_input = Solution::parse_input(input);
        let res = parsed_input
            .into_iter()
            .map(|(r, v)| {
                Solution::fix_equation(r, &v, vec![Operation::Plus, Operation::Mul]).unwrap_or(0)
            })
            .sum::<i64>();
        format!("{}", res)
    }
    fn part2(&self, input: &[&str]) -> String {
        let parsed_input = Solution::parse_input(input);
        let res = parsed_input
            .into_iter()
            .map(|(r, v)| {
                Solution::fix_equation(
                    r,
                    &v,
                    vec![Operation::Plus, Operation::Mul, Operation::Concat],
                )
                .unwrap_or(0)
            })
            .sum::<i64>();
        format!("{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_correct_digit() {
        assert_eq!(
            Solution::fix_equation(10, &vec![10], vec![Operation::Plus, Operation::Mul])
                .expect("Cannot find solution"),
            10
        )
    }
}
