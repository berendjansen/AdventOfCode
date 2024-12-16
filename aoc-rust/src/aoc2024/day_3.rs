use aoc::Solver;
use regex::Regex;

pub struct Solution;

impl Solution {
    fn extract_pattern<'a>(input: &'a str, pattern: &str) -> Vec<&'a str> {
        let re = Regex::new(pattern).expect("Could not create regex.");
        re.find_iter(input).map(|m| m.as_str()).collect()
    }

    fn parse_mul(input: &str) -> i64 {
        input
            .replace("mul(", "")
            .replace(")", "")
            .split(",")
            .map(|s| s.parse::<i64>().expect("Cannot parse i64 from string"))
            .product()
    }
}

impl Solver for Solution {
    fn part1(&self, input: &[&str]) -> String {
        let res = input
            .iter()
            .flat_map(|l| Solution::extract_pattern(l, r"mul\(\d*,\d*\)"))
            .map(Solution::parse_mul)
            .sum::<i64>();

        format!("{}", res)
    }

    fn part2(&self, input: &[&str]) -> String {
        let mut is_do = true;
        let res = input
            .iter()
            .flat_map(|l| Solution::extract_pattern(l, r"(mul\(\d*,\d*\))|(don't)|(do)"))
            .map(|l| match l {
                "do" => {
                    is_do = true;
                    0
                }
                "don't" => {
                    is_do = false;
                    0
                }
                _ => {
                    if is_do {
                        Solution::parse_mul(l)
                    } else {
                        0
                    }
                }
            })
            .sum::<i64>();

        format!("{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mul() {
        assert_eq!(Solution::parse_mul("mul(4,5)"), 20);
        assert_eq!(Solution::parse_mul("mul(0,2)"), 0);
        assert_eq!(Solution::parse_mul("mul(-5,2)"), -10);
        assert_eq!(Solution::parse_mul("mul(25,45)"), 1125);
    }

    #[test]
    fn regex_extracts_relevant_strings() {
        let test_string = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(
            Solution::extract_pattern(test_string, r"mul\(\d*,\d*\)"),
            vec!["mul(2,4)", "mul(5,5)", "mul(11,8)", "mul(8,5)"]
        )
    }

    #[test]
    fn conditional_regex_extracts_relevant_strings() {
        let test_string =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(
            Solution::extract_pattern(test_string, r"(mul\(\d*,\d*\))|(don't)|(do)"),
            vec![
                "mul(2,4)",
                "don't",
                "mul(5,5)",
                "mul(11,8)",
                "do",
                "mul(8,5)"
            ]
        )
    }

    #[test]
    fn test_example_part1() {
        let solver = Solution;
        let res = solver.part1(&vec![
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        ]);
        assert_eq!(res, String::from("161"));
    }

    #[test]
    fn test_example_part2() {
        let solver = Solution;
        let res = solver.part2(&vec![
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        ]);
        assert_eq!(res, String::from("48"));
    }
}
