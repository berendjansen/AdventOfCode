use aoc::Solver;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    fn parse_ordering_line(s: &str) -> Option<(usize, usize)> {
        s.split("|")
            .map(|s| str::parse::<usize>(s).expect("Cannot parse string to usize."))
            .collect_tuple()
    }

    fn construct_set_of_orderings_from_input(input: &[&str]) -> HashSet<(usize, usize)> {
        input
            .iter()
            .filter(|l| {
                let re = Regex::new(r"\d*\|\d*").expect("Cannot contrust regex.");
                re.is_match(l)
            })
            .map(|l| Solution::parse_ordering_line(l).expect("Cannot parse line to ordering pair."))
            .collect::<HashSet<(usize, usize)>>()
    }

    fn get_middle_element_of_vector<T>(v: &Vec<T>) -> Option<&T> {
        if v.len() % 2 != 1 {
            return None;
        }

        v.get(v.len() / 2)
    }

    fn is_correctly_ordered(v: &Vec<usize>, set_of_orderings: &HashSet<(usize, usize)>) -> bool {
        v.windows(2)
            .map(|w| !set_of_orderings.contains(&(w[1], w[0])))
            .all(|x| x)
    }

    fn fix_ordering<'a>(
        v: &'a mut Vec<usize>,
        set_of_orderings: &HashSet<(usize, usize)>,
    ) -> &'a Vec<usize> {
        while !Solution::is_correctly_ordered(v, set_of_orderings) {
            for i in 0..v.len() - 1 {
                if set_of_orderings.contains(&(v[i + 1], v[i])) {
                    v.swap(i, i + 1)
                }
            }
        }
        v
    }
}

impl Solver for Solution {
    fn part1(&self, input: &[&str]) -> String {
        let set_of_orderings = Solution::construct_set_of_orderings_from_input(input);
        let update_lines = input
            .iter()
            .filter(|l| l.contains(","))
            .map(|l| {
                l.split(",")
                    .map(|d| str::parse::<usize>(d).expect("Cannot parse digits to usize."))
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>();
        let res = update_lines
            .iter()
            .map(|v| {
                if Solution::is_correctly_ordered(v, &set_of_orderings) {
                    Solution::get_middle_element_of_vector(v).expect("Cannot get middle of vector.")
                } else {
                    &(0 as usize)
                }
            })
            .sum::<usize>();

        format!("{}", res)
    }
    fn part2(&self, input: &[&str]) -> String {
        let set_of_orderings = Solution::construct_set_of_orderings_from_input(input);
        let mut update_lines = input
            .iter()
            .filter(|l| l.contains(","))
            .map(|l| {
                l.split(",")
                    .map(|d| str::parse::<usize>(d).expect("Cannot parse digits to usize."))
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>();

        let res = update_lines
            .iter_mut()
            .filter(|v| !Solution::is_correctly_ordered(v, &set_of_orderings))
            .map(|v| {
                let new_v = Solution::fix_ordering(v, &set_of_orderings);
                Solution::get_middle_element_of_vector(new_v).expect("Cannot get middle of vector.")
            })
            .sum::<usize>();

        format!("{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ordering_line() {
        assert_eq!(Solution::parse_ordering_line("5|6"), Some((5, 6)));
        assert_eq!(Solution::parse_ordering_line("2|8"), Some((2, 8)));
    }

    #[test]
    fn test_construct_set_of_orderings_from_input() {
        let input = vec!["5|6", "2|5", "9|3"];
        let res = Solution::construct_set_of_orderings_from_input(&input);
        let mut expected_output = HashSet::new();
        expected_output.insert((5, 6));
        expected_output.insert((2, 5));
        expected_output.insert((9, 3));
        assert_eq!(res, expected_output);
    }

    #[test]
    fn get_middle_of_vector() {
        assert_eq!(
            Solution::get_middle_element_of_vector(&vec![1, 2, 3, 4, 5]),
            Some(&3)
        );
        assert_eq!(
            Solution::get_middle_element_of_vector(&vec![1, 2, 3]),
            Some(&2)
        );
        assert_eq!(
            Solution::get_middle_element_of_vector(&vec![1, 2, 3, 4]),
            None
        );
    }

    #[test]
    fn test_fix_orderings() {
        let mut orderings = HashSet::new();
        orderings.insert((75, 53));
        orderings.insert((75, 61));
        orderings.insert((75, 47));
        orderings.insert((97, 75));
        orderings.insert((97, 47));
        orderings.insert((97, 61));
        orderings.insert((97, 53));
        orderings.insert((47, 61));
        orderings.insert((47, 53));
        orderings.insert((61, 53));

        let mut v: Vec<usize> = vec![75, 97, 47, 61, 53];
        let correct: Vec<usize> = vec![97, 75, 47, 61, 53];
        assert_eq!(Solution::fix_ordering(&mut v, &orderings), &correct)
    }
}
