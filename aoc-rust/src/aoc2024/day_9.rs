use std::cmp::Reverse;
use std::collections::BinaryHeap;

use aoc::Solver;

pub struct Solution;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BlockType {
    Free,
    File,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Block {
    block_type: BlockType,
    size: usize,
    pos: usize,
}

impl Block {
    fn new(block_type: BlockType, size: usize, pos: usize) -> Block {
        Block {
            block_type,
            size,
            pos,
        }
    }
}

impl Solution {
    fn expand_disk_map(disk_map: &str) -> Vec<String> {
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
                    i * str::parse::<usize>(&c).unwrap()
                }
            })
            .sum::<usize>()
    }

    fn rearrange_blocks_per_file(
        disk_map: Vec<Block>,
        mut free_spaces: [BinaryHeap<Reverse<usize>>; 10],
    ) -> usize {
        let mut j = disk_map.len() - 1;
        let mut out = 0;

        while j > 0 {
            let mut first_pos = usize::MAX;
            let mut heap_popped = usize::MAX;
            for (i, spaces) in free_spaces.iter().skip(disk_map[j].size).enumerate() {
                if let Some(Reverse(free_block_position)) = spaces.peek() {
                    if free_block_position < &first_pos {
                        first_pos = *free_block_position;
                        heap_popped = i + disk_map[j].size;
                    }
                }
            }

            // if we have found a space we move
            let pos = if heap_popped < usize::MAX && first_pos < disk_map[j].pos {
                let file_block = disk_map[j];
                let space_size = heap_popped;

                free_spaces[heap_popped].pop(); // remove the previous space
                if file_block.size < space_size {
                    free_spaces[space_size - file_block.size]
                        .push(Reverse(first_pos + file_block.size));
                }
                first_pos
            } else {
                disk_map[j].pos
            };

            for k in pos..(pos + disk_map[j].size) {
                out += k * j / 2;
            }
            j -= 2;
        }
        out
    }
}

impl Solver for Solution {
    fn part1(&self, input: &[&str]) -> String {
        let input = input[0];
        let res = Solution::checksum(Solution::rearrange_blocks(Solution::expand_disk_map(input)));
        format!("{}", res)
    }

    fn part2(&self, input: &[&str]) -> String {
        let input = input[0].trim();
        let mut disk_map = vec![];
        let mut pos = 0;
        let mut free_spaces: [BinaryHeap<Reverse<usize>>; 10] = [const { BinaryHeap::new() }; 10];
        for (i, b) in input.chars().enumerate() {
            let block_type = if i % 2 != 0 {
                BlockType::Free
            } else {
                BlockType::File
            };

            let block_size = b.to_digit(10).unwrap() as usize;

            disk_map.push(Block::new(block_type, block_size, pos));

            if let BlockType::Free = block_type {
                free_spaces[block_size].push(Reverse(pos))
            }
            pos += block_size;
        }

        let res = Solution::rearrange_blocks_per_file(disk_map, free_spaces);
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
            Solution::expand_disk_map(disk_map),
            "0..111....22222"
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );

        let disk_map = "2333133121414131402";
        assert_eq!(
            Solution::expand_disk_map(disk_map),
            "00...111...2...333.44.5555.6666.777.888899"
                .chars()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
        );

        let disk_map = "23331331214141314022503";
        assert_eq!(
            Solution::expand_disk_map(disk_map),
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
