#![feature(array_windows)]
use std::collections::HashSet;

type Position = (i32, i32);
type Offset = (i32, i32);

const R_MOVE: Offset = (1, 0);
const L_MOVE: Offset = (-1, 0);
const D_MOVE: Offset = (0, 1);
const U_MOVE: Offset = (0, -1);

fn diff(a: i32, b: i32) -> i32 {
    a - b
}

fn distance_between_positions((x1, y1): Position, (x2, y2): Position) -> Offset {
    (diff(x1, x2), diff(y1, y2))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut tail_visited: HashSet<Position> = HashSet::new();
    let mut head_pos: Position = (0, 4);
    let mut tail_positions: Vec<Position> = (0..9).map(|_| (0, 4)).collect::<Vec<Position>>();
    let head_movements = input
        .lines()
        .flat_map(|l| {
            let mut split = l.split(" ");
            let c = split.next().unwrap().chars().next().unwrap();
            let count = split.next().unwrap().parse::<i32>().unwrap();
            (0..count).map(move |_| c)
        })
        .collect::<Vec<char>>();
    for movement in head_movements.iter() {
        let position_diff = match movement {
            'R' => R_MOVE,
            'L' => L_MOVE,
            'D' => D_MOVE,
            'U' => U_MOVE,
            _ => panic!("Invalid movement"),
        };
        head_pos = (head_pos.0 + position_diff.0, head_pos.1 + position_diff.1);
        let mut last_tail: Option<(i32, i32)> = None;
        for tail_pos in tail_positions.iter_mut() {
            //Calculate offset between head and tail
            let head = last_tail.unwrap_or(head_pos);
            let tail_distance_to_head = distance_between_positions(head, *tail_pos);
            if tail_distance_to_head.0 > 1
                || tail_distance_to_head.1 > 1
                || tail_distance_to_head.0 < -1
                || tail_distance_to_head.1 < -1
            {
                let x_diff = tail_distance_to_head.0;
                let y_diff = tail_distance_to_head.1;
                if x_diff == 0 {
                    //we are on same row
                    if y_diff > 0 {
                        *tail_pos = (tail_pos.0, tail_pos.1 + 1)
                    } else {
                        *tail_pos = (tail_pos.0, tail_pos.1 - 1)
                    }
                } else if y_diff == 0 {
                    //we are on same column
                    if x_diff > 0 {
                        *tail_pos = (tail_pos.0 + 1, tail_pos.1)
                    } else {
                        *tail_pos = (tail_pos.0 - 1, tail_pos.1)
                    }
                } else {
                    //Diagonal required
                    if x_diff > 0 && y_diff > 0 {
                        //Move up and right
                        *tail_pos = (tail_pos.0 + 1, tail_pos.1 + 1)
                    } else if x_diff < 0 && y_diff > 0 {
                        //Move up and left
                        *tail_pos = (tail_pos.0 - 1, tail_pos.1 + 1)
                    } else if x_diff > 0 && y_diff < 0 {
                        //Move down and right
                        *tail_pos = (tail_pos.0 + 1, tail_pos.1 - 1)
                    } else if x_diff < 0 && y_diff < 0 {
                        //Move down and left
                        *tail_pos = (tail_pos.0 - 1, tail_pos.1 - 1)
                    } else {
                        dbg!("Missing case?");
                    }
                }
            }
            last_tail = Some(*tail_pos);
        }
        tail_visited.insert(*tail_positions.last().unwrap());
    }
    Some(tail_visited.len())
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut tail_visited: HashSet<Position> = HashSet::new();
    let mut head_pos: Position = (0, 4);
    let mut tail_pos: Position = (0, 4);
    let head_movements = input
        .lines()
        .flat_map(|l| {
            let mut split = l.split(" ");
            let c = split.next().unwrap().chars().next().unwrap();
            let count = split.next().unwrap().parse::<i32>().unwrap();
            (0..count).map(move |_| c)
        })
        .collect::<Vec<char>>();
    for movement in head_movements.iter() {
        let position_diff = match movement {
            'R' => R_MOVE,
            'L' => L_MOVE,
            'D' => D_MOVE,
            'U' => U_MOVE,
            _ => panic!("Invalid movement"),
        };
        head_pos = (head_pos.0 + position_diff.0, head_pos.1 + position_diff.1);
        //Calculate offset between head and tail
        let tail_distance_to_head = distance_between_positions(head_pos, tail_pos);
        if tail_distance_to_head.0 > 1
            || tail_distance_to_head.1 > 1
            || tail_distance_to_head.0 < -1
            || tail_distance_to_head.1 < -1
        {
            let x_diff = tail_distance_to_head.0;
            let y_diff = tail_distance_to_head.1;
            if x_diff == 0 {
                //we are on same row
                if y_diff > 0 {
                    tail_pos = (tail_pos.0, tail_pos.1 + 1)
                } else {
                    tail_pos = (tail_pos.0, tail_pos.1 - 1)
                }
            } else if y_diff == 0 {
                //we are on same column
                if x_diff > 0 {
                    tail_pos = (tail_pos.0 + 1, tail_pos.1)
                } else {
                    tail_pos = (tail_pos.0 - 1, tail_pos.1)
                }
            } else {
                //Diagonal required
                if x_diff > 0 && y_diff > 0 {
                    //Move up and right
                    tail_pos = (tail_pos.0 + 1, tail_pos.1 + 1)
                } else if x_diff < 0 && y_diff > 0 {
                    //Move up and left
                    tail_pos = (tail_pos.0 - 1, tail_pos.1 + 1)
                } else if x_diff > 0 && y_diff < 0 {
                    //Move down and right
                    tail_pos = (tail_pos.0 + 1, tail_pos.1 - 1)
                } else if x_diff < 0 && y_diff < 0 {
                    //Move down and left
                    tail_pos = (tail_pos.0 - 1, tail_pos.1 - 1)
                } else {
                    dbg!("Missing case?");
                }
            }
        }
        tail_visited.insert(tail_pos);
        //Check if it is more than 1/-1 in any direction and try to follow along and make sure to mark visibility
    }
    // dbg!(tail_pos);
    Some(tail_visited.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        let cwd = env::current_dir().unwrap();

        let filepath = cwd
            .join("src")
            .join("examples")
            .join(format!("{:02}_2.txt", 9));

        let f = fs::read_to_string(filepath);
        assert_eq!(part_two(&input), Some(1));
        assert_eq!(part_two(&f.unwrap()), Some(36));
    }
}
