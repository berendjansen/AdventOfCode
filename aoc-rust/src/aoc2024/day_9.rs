use aoc::Solver;

pub struct Solution;

enum BlockType {
    Free,
    File,
}

struct Block {
    block_type: BlockType,
    space: usize,
    start_pos: usize,
}

impl Solution {
    fn convert_disk_map_to_blocks(disk_map: &str) -> Vec<String> {
        let mut out = vec![];
        for (i, b) in disk_map.chars().enumerate() {
            let c = if i % 2 != 0 {
                "."
            } else {
                &(i / 2).to_string()
            };
            for _ in 0..b.to_digit(10).unwrap() {
                out.push(c.to_string());
            }
        }
        out
    }

    fn rearrange_blocks(mut disk_map: Vec<String>) -> Vec<String> {
        let (mut i, mut j) = (0, disk_map.len() - 1);
        while i < j {
            if disk_map[j] == "." {
                j -= 1;
                continue;
            }
            if disk_map[i] == "." {
                disk_map.swap(i, j);
                i += 1;
                j -= 1;
            } else {
                i += 1;
            }
            // println!("");
        }
        disk_map
    }

    fn checksum(blocks: Vec<String>) -> usize {
        blocks
            .into_iter()
            .enumerate()
            .map(|(i, c)| {
                if c == "." {
                    0
                } else {
                    i * str::parse::<usize>(&c).unwrap() as usize
                }
            })
            .sum::<usize>()
    }

    fn rearrange_blocks_per_file(mut disk_map: Vec<String>) -> Vec<String> {
        let (mut _i, mut j) = (0, disk_map.len() - 1);
        while j > 0 {
            if disk_map[j] == "." {
                j -= 1;
            } else {
                let end_of_file = j;
                while j > 0 && disk_map[j] == disk_map[end_of_file] {
                    j -= 1;
                }
                let file_size = end_of_file - j;

                for i in 0..(j + 1) {
                    let start_free_space = i;
                    let mut k = i;
                    while k <= j && disk_map[k] == "." {
                        k += 1;
                    }

                    let space = k - start_free_space;

                    if space >= file_size {
                        for f in 0..file_size {
                            disk_map.swap(start_free_space + f, end_of_file - f)
                        }
                        break;
                    }
                }
            }
        }
        disk_map
    }
}

impl Solver for Solution {
    fn part1(&self, input: &[&str]) -> String {
        let input = input[0];
        let res = Solution::checksum(Solution::rearrange_blocks(
            Solution::convert_disk_map_to_blocks(input),
        ));
        format!("{}", res)
    }

    fn part2(&self, input: &[&str]) -> String {
        let input = input[0];
        let res = Solution::checksum(Solution::rearrange_blocks_per_file(
            Solution::convert_disk_map_to_blocks(input),
        ));
        format!("{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn validate_no_digits_after_period(blocks: Vec<String>) -> bool {
        let mut period_seen = false;
        for s in blocks.iter() {
            if s == &"." {
                period_seen = true;
            };
            if s != &"." && period_seen {
                return false;
            }
        }
        true
    }
    #[test]
    fn test_disk_map_conversion() {
        let disk_map = "12345";
        assert_eq!(
            Solution::convert_disk_map_to_blocks(disk_map),
            "0..111....22222"
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );

        let disk_map = "2333133121414131402";
        assert_eq!(
            Solution::convert_disk_map_to_blocks(disk_map),
            "00...111...2...333.44.5555.6666.777.888899"
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );

        let disk_map = "23331331214141314022503";
        assert_eq!(
            Solution::convert_disk_map_to_blocks(disk_map),
            vec![
                "0", "0", ".", ".", ".", "1", "1", "1", ".", ".", ".", "2", ".", ".", ".", "3",
                "3", "3", ".", "4", "4", ".", "5", "5", "5", "5", ".", "6", "6", "6", "6", ".",
                "7", "7", "7", ".", "8", "8", "8", "8", "9", "9", ".", ".", "10", "10", "10", "10",
                "10", "11", "11", "11",
            ]
        );
    }

    #[test]
    fn test_block_validation() {
        assert!(!validate_no_digits_after_period(
            vec!["5", ".", "3"]
                .into_iter()
                .map(|x| x.to_owned())
                .collect()
        ));

        assert!(validate_no_digits_after_period(
            vec!["5", "3", "."]
                .into_iter()
                .map(|x| x.to_owned())
                .collect()
        ))
    }

    #[test]
    fn test_rearrange_block() {
        let block = "0..111....22222".chars().map(|x| x.to_string()).collect();
        assert_eq!(
            Solution::rearrange_blocks(block),
            "022111222......"
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );

        let block = "0..111....22222".chars().map(|x| x.to_string()).collect();
        assert!(validate_no_digits_after_period(Solution::rearrange_blocks(
            block
        ),))
    }

    #[test]
    fn test_rearrange_block_per_file() {
        let block = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(
            Solution::rearrange_blocks_per_file(block),
            "00992111777.44.333....5555.6666.....8888.."
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_rearrange_block_2() {
        let mut block: Vec<String> = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .map(|x| x.to_string())
            .collect();
        block.push(".".to_string());
        block.push(".".to_string());
        block.push("10".to_string());
        block.push("10".to_string());
        block.push("10".to_string());
        block.push("10".to_string());
        assert_eq!(
            Solution::rearrange_blocks(block),
            vec![
                "0", "0", "10", "10", "10", "1", "1", "1", "10", "9", "9", "2", "8", "8", "8", "3",
                "3", "3", "8", "4", "4", "7", "5", "5", "5", "5", "7", "6", "6", "6", "6", "7",
                ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".", ".",
            ]
        );

        let block = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .map(|x| x.to_string())
            .collect();
        assert!(validate_no_digits_after_period(Solution::rearrange_blocks(
            block
        ),))
    }

    #[test]
    fn test_checksum_block() {
        let block = "0099811188827773336446555566.............."
            .chars()
            .map(|x| x.to_string())
            .collect();
        assert_eq!(Solution::checksum(block), 1928)
    }

    #[test]
    fn test_checksum_block_2() {
        let block = "022111222......".chars().map(|x| x.to_string()).collect();
        assert_eq!(Solution::checksum(block), 60)
    }
}
