use std::collections::{HashMap, HashSet};

use glam::IVec2;

pub fn part_one(input: &str) -> Option<i32> {
    let rounds = 10;
    let mut elves: HashSet<IVec2> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(IVec2::new(x as i32, y as i32))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let directions = vec![
        //N, NE, or NW
        vec![IVec2::new(0, -1), IVec2::new(1, -1), IVec2::new(-1, -1)],
        //S, SE, or SW
        vec![IVec2::new(0, 1), IVec2::new(1, 1), IVec2::new(-1, 1)],
        //W, NW, or SW
        vec![IVec2::new(-1, 0), IVec2::new(-1, 1), IVec2::new(-1, -1)],
        //E, NE, or SE
        vec![IVec2::new(1, 0), IVec2::new(1, 1), IVec2::new(1, -1)],
    ]
    .into_iter()
    .cycle();
    for i in 0..rounds {
        let mut attempted_moves: HashMap<IVec2, Vec<IVec2>> = HashMap::new();
        let current_directions: Vec<Vec<IVec2>> = directions.clone().skip(i).take(4).collect();
        for elf in elves.iter() {
            //Check if elf doesnt have any neighbors
            let no_neighbors = current_directions
                .iter()
                .flat_map(|directions| directions)
                .all(|dir| !elves.contains(&(*dir + *elf)));
            if no_neighbors {
                attempted_moves.entry(*elf).or_insert(vec![*elf]);
                continue; //Elf doesn't do anything
            }

            let possible_move = current_directions.iter().find_map(|directions| {
                directions
                    .iter()
                    .all(|dir| !elves.contains(&(*dir + *elf)))
                    .then_some(directions[0])
            });

            if let Some(pos_move) = possible_move {
                let new_elf = pos_move + *elf;
                attempted_moves
                    .entry(new_elf)
                    .and_modify(|attempts| attempts.push(*elf))
                    .or_insert(vec![new_elf]);
            } else {
                attempted_moves.entry(*elf).and_modify(|attempts| attempts.push(*elf)).or_insert(vec![*elf]);
            }
        }

        elves = attempted_moves
            .iter()
            .flat_map(|(elf, attempts)| {
                if attempts.len() == 1 {
                    vec![*elf]
                } else {
                    attempts.to_vec()
                }
            })
            .collect()
    }
    let min_x = elves.iter().min_by_key(|pos| pos.x).expect("exists").x;
    let max_x = elves.iter().max_by_key(|pos| pos.x).expect("exists").x;
    let min_y = elves.iter().min_by_key(|pos| pos.y).expect("exists").y;
    let max_y = elves.iter().max_by_key(|pos| pos.y).expect("exists").y;
    Some((max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut elves: HashSet<IVec2> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| {
                    if c == '#' {
                        Some(IVec2::new(x as i32, y as i32))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();
    let directions = vec![
        //N, NE, or NW
        vec![IVec2::new(0, -1), IVec2::new(1, -1), IVec2::new(-1, -1)],
        //S, SE, or SW
        vec![IVec2::new(0, 1), IVec2::new(1, 1), IVec2::new(-1, 1)],
        //W, NW, or SW
        vec![IVec2::new(-1, 0), IVec2::new(-1, 1), IVec2::new(-1, -1)],
        //E, NE, or SE
        vec![IVec2::new(1, 0), IVec2::new(1, 1), IVec2::new(1, -1)],
    ]
    .into_iter()
    .cycle();
    let mut result = 0;
    for i in 0.. {
        let mut attempted_moves: HashMap<IVec2, Vec<IVec2>> = HashMap::new();
        let current_directions: Vec<Vec<IVec2>> = directions.clone().skip(i).take(4).collect();
        for elf in elves.iter() {
            //Check if elf doesnt have any neighbors
            let no_neighbors = current_directions
                .iter()
                .flat_map(|directions| directions)
                .all(|dir| !elves.contains(&(*dir + *elf)));
            if no_neighbors {
                attempted_moves.entry(*elf).and_modify(|attempts| attempts.push(*elf)).or_insert(vec![*elf]);
                continue; //Elf doesn't do anything
            }

            let possible_move = current_directions.iter().find_map(|directions| {
                directions
                    .iter()
                    .all(|dir| !elves.contains(&(*dir + *elf)))
                    .then_some(directions[0] + *elf)
            });

            if let Some(new_elf) = possible_move {
                attempted_moves
                    .entry(new_elf)
                    .and_modify(|attempts| attempts.push(*elf))
                    .or_insert(vec![*elf]);
            } else {
                attempted_moves.entry(*elf).and_modify(|attempts| attempts.push(*elf)).or_insert(vec![*elf]);
            }
        }
        
        let new_elves = attempted_moves
            .iter()
            .flat_map(|(elf, attempts)| {
                if attempts.len() == 1 {
                    vec![*elf]
                } else {
                    attempts.to_vec()
                }
            })
            .collect::<HashSet<IVec2>>();
        if elves.iter().all(|pos| new_elves.contains(pos)) && new_elves.iter().all(|pos| elves.contains(pos)) {
            result = i;
            break;
        }
        else {
            elves = new_elves;
        }
    }
    Some(result + 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 23);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_one(&input), Some(110));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 23);
        assert_eq!(part_two(&input), Some(20));
    }
}
