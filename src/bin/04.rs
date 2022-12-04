use std::{ops::Range, collections::{HashSet, hash_set}};

pub fn part_one(input: &str) -> Option<usize> {
    let overlapping_jobs = input.lines().map(|line| line.split_once(",").unwrap()).map(|l| [l.0, l.1].iter().map(|section| {
        let parts = section.split_once("-").unwrap();
        let start = parts.0.parse::<u32>().unwrap();
        let end = parts.1.parse::<u32>().unwrap();
        (start, end)
    }).collect::<Vec<(u32, u32)>>()).filter(|r| {
        let left = r[0];
        let right = r[1];
        (right.0 >= left.0 && right.1 <= left.1) || (left.0 >= right.0 && left.1 <= right.1)
    }).count();
    Some(overlapping_jobs)
}

pub fn part_two(input: &str) -> Option<usize> {
    let overlapping_jobs = input.lines().map(|line| line.split_once(",").unwrap()).map(|l| [l.0, l.1].iter().map(|section| {
        let parts = section.split_once("-").unwrap();
        let start = parts.0.parse::<u32>().unwrap();
        let end = parts.1.parse::<u32>().unwrap();
        (start..(end + 1)).collect::<HashSet<u32>>()
    }).collect::<Vec<HashSet<u32>>>()).filter(|r| {
        let left = &r[0];
        let right = &r[1];
        left.intersection(right).count() > 0
    }).count();
    Some(overlapping_jobs)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2))
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
