#![feature(iter_array_chunks)]

use std::collections::VecDeque;
pub fn part_one(input: &str) -> Option<String> {
    let mut stack_length = 0;
    let stacks = input
        .lines()
        .take_while(|line| !line.is_empty())
        .flat_map(|line| {
            [line, " "]
                .join("")
                .chars()
                .array_chunks::<4>()
                .filter_map(|chunk| match chunk.get(1) {
                    Some(c) => {
                        if c.is_numeric() {
                            stack_length += 1;
                            None
                        } else {
                            Some(*c)
                        }
                    }
                    None => None,
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<char>>();
    let mut vecs: Vec<VecDeque<char>> = Vec::new();
    for i in 0..stack_length {
        let mut vec: VecDeque<char> = VecDeque::new();
        for k in 0..stack_length {
            let idx = i + (k * (stack_length));
            let c = *stacks.get(idx).unwrap_or(&' ');
            if !c.is_whitespace() {
                vec.push_front(c);
            }
        }
        vecs.push(vec);
    }
    let moves = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(" ")
                .filter_map(|parts| {
                    let c = parts.parse::<usize>();
                    match c {
                        Ok(size) => Some(size),
                        Err(_) => None,
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    // stacks.reverse();
    for (_idx, action) in moves.iter().enumerate() {
        let (count, from, to) = (action[0], action[1], action[2]);
        let mut popped_chars: Vec<char> = Vec::new();
        println!("{count} {from} {to}");
        for _i in 0..count {
            let popped = vecs[from - 1].pop_back().unwrap();
            popped_chars.push(popped);
        }
        for c in popped_chars {
            vecs[to - 1].push_back(c);
        }
    }
    Some(String::from_iter(
        vecs.iter()
            .map(|v| v.to_owned().pop_back().unwrap())
            .collect::<Vec<char>>(),
    ))
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stack_length = 0;
    let stacks = input
        .lines()
        .take_while(|line| !line.is_empty())
        .flat_map(|line| {
            [line, " "]
                .join("")
                .chars()
                .array_chunks::<4>()
                .filter_map(|chunk| match chunk.get(1) {
                    Some(c) => {
                        if c.is_numeric() {
                            stack_length += 1;
                            None
                        } else {
                            Some(*c)
                        }
                    }
                    None => None,
                })
                .collect::<Vec<char>>()
        })
        .collect::<Vec<char>>();
    let mut vecs: Vec<VecDeque<char>> = Vec::new();
    for i in 0..stack_length {
        let mut vec: VecDeque<char> = VecDeque::new();
        for k in 0..stack_length {
            let idx = i + (k * (stack_length));
            let c = *stacks.get(idx).unwrap_or(&' ');
            if !c.is_whitespace() {
                vec.push_front(c);
            }
        }
        vecs.push(vec);
    }
    let moves = input
        .lines()
        .skip_while(|line| !line.is_empty())
        .skip(1)
        .map(|line| {
            line.split(" ")
                .filter_map(|parts| {
                    let c = parts.parse::<usize>();
                    match c {
                        Ok(size) => Some(size),
                        Err(_) => None,
                    }
                })
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    // stacks.reverse();
    for (_idx, action) in moves.iter().enumerate() {
        let (count, from, to) = (action[0], action[1], action[2]);
        let mut popped_chars: Vec<char> = Vec::new();
        for _i in 0..count {
            let popped = vecs[from - 1].pop_back().unwrap();
            popped_chars.push(popped);
        }
        popped_chars.reverse();
        for c in popped_chars {
            vecs[to - 1].push_back(c);
        }
    }
    Some(String::from_iter(
        vecs.iter()
            .map(|v| v.to_owned().pop_back().unwrap())
            .collect::<Vec<char>>(),
    ))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_string()));
    }
}
