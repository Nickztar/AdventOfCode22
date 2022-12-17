use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let mut rock_structures: HashSet<(u32, u32)> = input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|rock_struct| {
                    let (x, y) = rock_struct.split_once(",").unwrap();
                    (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
                })
                .collect::<Vec<(u32, u32)>>()
                .windows(2)
                .flat_map(|window| {
                    let from = window[0];
                    let to = window[1];
                    if from.0 == to.0 {
                        //Same X
                        if from.1 > to.1 {
                            (to.1..=from.1)
                                .map(|y| (from.0, y))
                                .collect::<Vec<(u32, u32)>>()
                        } else {
                            (from.1..=to.1)
                                .map(|y| (from.0, y))
                                .collect::<Vec<(u32, u32)>>()
                        }
                    } else {
                        if from.0 > to.0 {
                            (to.0..=from.0)
                                .map(|x| (x, from.1))
                                .collect::<Vec<(u32, u32)>>()
                        } else {
                            (from.0..=to.0)
                                .map(|x| (x, from.1))
                                .collect::<Vec<(u32, u32)>>()
                        }
                    }
                })
                .collect::<Vec<(u32, u32)>>()
        })
        .collect();
    let max_y = rock_structures.iter().map(|pos| pos.1).max().unwrap();
    let mut sand_at_rest_count: u32 = 0;
    'outer: loop {
        let mut sand_position: (u32, u32) = (500, 0);
        'inner: loop {
            if sand_position.1 > max_y {
                break 'outer;
            }
            let down_pos = (sand_position.0, sand_position.1 + 1);
            let down_left = (sand_position.0 - 1, sand_position.1 + 1);
            let down_right = (sand_position.0 + 1, sand_position.1 + 1);

            if move_possible(&rock_structures, &down_pos) {
                sand_position = down_pos;
            } else if move_possible(&rock_structures, &down_left) {
                sand_position = down_left;
            } else if move_possible(&rock_structures, &down_right) {
                sand_position = down_right;
            } else {
                //No move possible, we are at rest
                rock_structures.insert(sand_position);
                sand_at_rest_count += 1;
                break 'inner;
            }
        }
    }
    Some(sand_at_rest_count)
}

fn move_possible(rock_structures: &HashSet<(u32, u32)>, position: &(u32, u32)) -> bool {
    !rock_structures.contains(position)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rock_structures: HashSet<(u32, u32)> = input
        .lines()
        .flat_map(|line| {
            line.split(" -> ")
                .map(|rock_struct| {
                    let (x, y) = rock_struct.split_once(",").unwrap();
                    (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap())
                })
                .collect::<Vec<(u32, u32)>>()
                .windows(2)
                .flat_map(|window| {
                    let from = window[0];
                    let to = window[1];
                    if from.0 == to.0 {
                        //Same X
                        if from.1 > to.1 {
                            (to.1..=from.1)
                                .map(|y| (from.0, y))
                                .collect::<Vec<(u32, u32)>>()
                        } else {
                            (from.1..=to.1)
                                .map(|y| (from.0, y))
                                .collect::<Vec<(u32, u32)>>()
                        }
                    } else {
                        if from.0 > to.0 {
                            (to.0..=from.0)
                                .map(|x| (x, from.1))
                                .collect::<Vec<(u32, u32)>>()
                        } else {
                            (from.0..=to.0)
                                .map(|x| (x, from.1))
                                .collect::<Vec<(u32, u32)>>()
                        }
                    }
                })
                .collect::<Vec<(u32, u32)>>()
        })
        .collect();
    let max_y = rock_structures.iter().map(|pos| pos.1).max().unwrap() + 2;
    let mut sand_at_rest_count: u32 = 0;
    'outer: loop {
        let mut sand_position: (u32, u32) = (500, 0);
        'inner: loop {
            if sand_position.1 == max_y {
                rock_structures.insert(sand_position);
                break 'inner;
            }

            let down_pos = (sand_position.0, sand_position.1 + 1);
            let down_left = (sand_position.0 - 1, sand_position.1 + 1);
            let down_right = (sand_position.0 + 1, sand_position.1 + 1);

            if move_possible(&rock_structures, &down_pos) {
                sand_position = down_pos;
            } else if move_possible(&rock_structures, &down_left) {
                sand_position = down_left;
            } else if move_possible(&rock_structures, &down_right) {
                sand_position = down_right;
            } else if sand_position == (500, 0) {
                sand_at_rest_count += 1; //This counts ;D
                break 'outer;
            } else {
                //No move possible, we are at rest
                rock_structures.insert(sand_position);
                sand_at_rest_count += 1;
                break 'inner;
            }
        }
    }
    Some(sand_at_rest_count)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}
