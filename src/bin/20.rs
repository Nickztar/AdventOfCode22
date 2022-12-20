use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<i64> {
    Some(decrypt(input, 1, 1))
}

fn decrypt(input: &str, key: i64, rounds: u32) -> i64 {
    let encrypted_numbers: Vec<i64> = input.lines().map(|line| line.parse().unwrap()).collect();
    let mut indices = (0..encrypted_numbers.len()).collect::<VecDeque<_>>();

    for _ in 0..rounds {
        for i in 0..encrypted_numbers.len() {
            let pos = indices.iter().position(|&p| p == i).unwrap();
    
            let num = encrypted_numbers[i] * key;
    
            let ir = indices.remove(pos);
            assert_eq!(ir, Some(i));
    
            let new_index = (pos as i64 + num).rem_euclid(encrypted_numbers.len() as i64 - 1);
            // dbg!(new_index, num);
            indices.insert(new_index as usize, i);
    
            // dbg!(indices
            //     .iter()
            //     .map(|&i| original_numbers[i])
            //     .collect::<Vec<_>>());
        }
    }
    let numbers = indices
        .iter()
        .map(|&i| encrypted_numbers[i])
        .collect::<Vec<_>>();

    let zero = numbers.iter().position(|&n| n == 0).unwrap();
    let indices = [1000, 2000, 3000];

    indices
        .iter()
        .map(|offset| numbers[(zero + offset) % numbers.len()] * key)
        .sum()
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(decrypt(input, 811589153, 10))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}
