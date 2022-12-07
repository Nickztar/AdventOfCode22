use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<usize> {
    let ce: Vec<(usize, char)> = input.chars().enumerate().collect();
    for i in 0..ce.len() {
        let start = ce[i];
        let x1 = ce[i+1];
        let x2 = ce[i+2];
        let x3 = ce[i+3];
        let values = [start.1, x1.1, x2.1, x3.1];
        let dedup = values.iter().collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<&char>>(); 
        if dedup.len() == 4 {
            return Some(i + 4);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<usize> {
    let ce: Vec<(usize, char)> = input.chars().enumerate().collect();
    for i in 0..ce.len() {
        let values = ce.get(i..i+14).unwrap();
        let dedup = values.iter().map(|x| x.1).collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<char>>(); 
        if dedup.len() == 14 {
            return Some(i + 14);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }
}
