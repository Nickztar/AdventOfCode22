pub fn part_one(input: &str) -> Option<u32> {
    let mut v = vec![0];
    let mut totals = vec![0];
    for line in input.lines().into_iter() {
        match line.parse::<u32>() {
            Ok(integer) => v.push(integer),
            Err(_) => {
                totals.push(v.iter().sum());
                v.clear();
            }
        }
    }
    Some(totals.iter().max().unwrap().to_owned())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut v = vec![0];
    let mut totals = vec![0];
    for line in input.lines().into_iter() {
        match line.parse::<u32>() {
            Ok(integer) => v.push(integer),
            Err(_) => {
                totals.push(v.iter().sum());
                v.clear();
            }
        }
    }
    if !v.is_empty() {
        totals.push(v.iter().sum());
    }
    totals.sort();
    Some(totals.iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
