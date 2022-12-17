use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize, char);

impl Pos {
    fn neighbours(&self, all_positions: &Vec<(usize, usize, char)>, part2: bool) -> Vec<Pos> {
        let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
        let &Pos(x, y, c) = self;
        let possible_moves: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let max_x = all_positions.iter().map(|p| p.0).max().unwrap();
        let max_y = all_positions.iter().map(|p| p.1).max().unwrap();
        //Find valid positions
        let valid_moves = possible_moves
            .iter()
            .filter_map(|(x_diff, y_diff)| {
                if x_diff < &0 {
                    match x.checked_sub(x_diff.abs() as usize) {
                        Some(subbed) => {
                            return Some((subbed, y));
                        }
                        None => {
                            return None;
                        }
                    }
                }
                if x_diff > &0 {
                    let res = x + *x_diff as usize;
                    if res > max_x {
                        return None;
                    } else {
                        return Some((res, y));
                    }
                }
                if y_diff < &0 {
                    match y.checked_sub(y_diff.abs() as usize) {
                        Some(subbed) => {
                            return Some((x, subbed));
                        }
                        None => {
                            return None;
                        }
                    }
                }
                if y_diff > &0 {
                    let res = y + *y_diff as usize;
                    if res > max_y {
                        return None;
                    } else {
                        return Some((x, res));
                    }
                }
                return None;
            })
            .collect::<Vec<(usize, usize)>>();
        let valid = all_positions
            .iter()
            .filter_map(|position| {
                let is_valid_pos = valid_moves.contains(&(position.0, position.1));
                let is_skippable = ['S'].contains(&c) || ['S'].contains(&position.2);
                if is_valid_pos && is_skippable {
                    return Some(Pos(position.0, position.1, position.2));
                } else if is_skippable {
                    return None;
                }

                let p2 = alphabet.iter().position(|w| w == &position.2).unwrap() as isize;
                let cp = alphabet.iter().position(|w| w == &c).unwrap() as isize;
                let can_move_diff = if part2 { cp - p2 <= 1 } else { p2 - cp <= 1 };
                if is_valid_pos && can_move_diff {
                    Some(Pos(position.0, position.1, position.2))
                } else {
                    None
                }
            })
            .collect::<Vec<Pos>>();
        return valid;
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut goal = Pos(0, 0, 'z');
    let mut start = Pos(0, 0, 'z');
    let mut all_positions: Vec<(usize, usize, char)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'E' {
                all_positions.push((x, y, 'z'));
                goal = Pos(x, y, 'z');
            } else if c == &'S' {
                start = Pos(x, y, 'S');
            } else {
                all_positions.push((x, y, *c));
            }
        }
    }
    let path = bfs(
        &start,
        |p| p.neighbours(&all_positions, false),
        |p| p == &goal,
    )
    .expect("Should have goal");
    Some(path.len() - 1)
}

pub fn part_two(input: &str) -> Option<usize> {
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut start = Pos(0, 0, 'z');
    let mut all_positions: Vec<(usize, usize, char)> = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if c == &'E' {
                all_positions.push((x, y, 'z'));
                start = Pos(x, y, 'z');
            } else if c == &'S' {
                all_positions.push((x, y, 'a'));
            } else {
                all_positions.push((x, y, *c));
            }
        }
    }
    let path = bfs(
        &start,
        |p| p.neighbours(&all_positions, true),
        |p| p.2 == 'a',
    )
    .expect("Should have goal");
    Some(path.len() - 1)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), Some(29));
    }
}
