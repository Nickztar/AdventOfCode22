use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
enum JetDirection {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct TetrisPiece {
    position: Vec<(i128, i128)>,
    left_idx: usize,
    right_idx: usize,
    name: String,
}

impl TetrisPiece {
    fn new(position: Vec<(i128, i128)>, name: String) -> Self {
        let left_idx = position
            .iter()
            .enumerate()
            .min_by_key(|(_, a)| a.0)
            .map(|(index, _)| index)
            .unwrap();
        let right_idx = position
            .iter()
            .enumerate()
            .max_by_key(|(_, a)| a.0)
            .map(|(index, _)| index)
            .unwrap();
        TetrisPiece {
            position,
            left_idx,
            right_idx,
            name,
        }
    }
    fn correct_position(&mut self, max_y: i128) {
        for pos in self.position.iter_mut() {
            *pos = (pos.0 + 2, pos.1 + max_y); //They all start from 2 to the left and 3 from top
        }
    }

    fn attempt_jet_push(&mut self, dir: &JetDirection, playground: &HashSet<(i128, i128)>) {
        match dir {
            JetDirection::Left => {
                for pos in self.position.iter_mut() {
                    *pos = (pos.0 - 1, pos.1);
                }
                for square in self.position.iter() {
                    if playground.contains(&square) || square.0 < 0 {
                        for pos in self.position.iter_mut() {
                            *pos = (pos.0 + 1, pos.1);
                        }
                        break;
                    }
                }
            }
            JetDirection::Right => {
                for pos in self.position.iter_mut() {
                    *pos = (pos.0 + 1, pos.1);
                }
                for square in self.position.iter() {
                    if playground.contains(&square) || square.0 > 6 {
                        for pos in self.position.iter_mut() {
                            *pos = (pos.0 - 1, pos.1);
                        }
                        break;
                    }
                }
            }
        }
    }

    fn fall(&mut self, playground: &HashSet<(i128, i128)>) -> bool {
        //It is possible to move
        for pos in self.position.iter_mut() {
            *pos = (pos.0, pos.1 - 1); //Move y one down
        }
        for square in self.position.iter() {
            if playground.contains(&square) || square.1 < 0 {
                for pos in self.position.iter_mut() {
                    *pos = (pos.0, pos.1 + 1); //Move y one up
                }
                return true;
            }
        }
        return false;
    }
}

pub fn part_one(input: &str) -> Option<i128> {
    let jet_directions: Vec<JetDirection> = input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(JetDirection::Left),
            '>' => Some(JetDirection::Right),
            _ => None,
        })
        .collect();
    let name: Vec<&str> = vec!["-", "+", "⅃", "|", "#"];
    let tetris_pieces: Vec<Vec<(i128, i128)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        //0123
        vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
        //.1.
        //012
        //.1.
        vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
        //..2
        //..2
        //012
        vec![(0, 3), (0, 2), (0, 1), (0, 0)],
        //0
        //0
        //0
        //0
        vec![(0, 1), (1, 1), (0, 0), (1, 0)],
        //01
        //01
    ];
    let mut playground: HashSet<(i128, i128)> = HashSet::new();
    let mut pieces_count: usize = 0;
    let mut jet_index = 0usize;
    let mut starting_y: i128 = 3;
    let mut directions = jet_directions.iter().cycle();
    while pieces_count < 2022 {
        //Spawn a piece
        let mut piece = TetrisPiece::new(
            tetris_pieces[pieces_count % tetris_pieces.len()].clone(),
            name[pieces_count % tetris_pieces.len()].to_string(),
        );
        //Increase piece count
        pieces_count += 1;
        //Correct piece position relative to largest Y
        piece.correct_position(starting_y);
        //Play the piece
        'outer: loop {
            // dbg!(&piece);
            //- Respect jetpush if all X in piece are < 0 > 7
            let push_dir = &jet_directions[jet_index % jet_directions.len()];
            piece.attempt_jet_push(push_dir, &playground);
            jet_index += 1;

            //- Move to ground
            if piece.fall(&playground) {
                for position in piece.position.iter() {
                    playground.insert(*position);
                    if starting_y < position.1 + 4 {
                        starting_y = position.1 + 4;
                    }
                }
                break 'outer;
            }
        }
    }
    Some(starting_y - 3)
}

pub fn part_two(input: &str) -> Option<i128> {
    let part2 = play_tetris(&input, 1_000_000_000_000);
    Some(part2)
}

fn play_tetris(input: &str, rounds: i128) -> i128 {
    let jet_directions: Vec<JetDirection> = input
        .chars()
        .filter_map(|c| match c {
            '<' => Some(JetDirection::Left),
            '>' => Some(JetDirection::Right),
            _ => None,
        })
        .collect();
    let name: Vec<&str> = vec!["-", "+", "⅃", "|", "#"];
    let tetris_pieces: Vec<Vec<(i128, i128)>> = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        //0123
        vec![(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
        //.1.
        //012
        //.1.
        vec![(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
        //..2
        //..2
        //012
        vec![(0, 3), (0, 2), (0, 1), (0, 0)],
        //0
        //0
        //0
        //0
        vec![(0, 1), (1, 1), (0, 0), (1, 0)],
        //01
        //01
    ];
    let mut playground: HashSet<(i128, i128)> = HashSet::new();
    let mut skylines = HashMap::new();
    let mut pieces_count: i128 = 0;
    let mut jet_index = 0usize;
    let mut starting_y: i128 = 3;
    while pieces_count < rounds {
        //Spawn a piece
        let mut piece = TetrisPiece::new(
            tetris_pieces[pieces_count as usize % tetris_pieces.len()].clone(),
            name[pieces_count as usize % tetris_pieces.len()].to_string(),
        );
        //Increase piece count
        //Correct piece position relative to largest Y
        piece.correct_position(starting_y);
        //Play the piece
        'outer: loop {
            // dbg!(&piece);
            //- Respect jetpush if all X in piece are < 0 > 7
            let push_dir = &jet_directions[jet_index % jet_directions.len()];
            piece.attempt_jet_push(push_dir, &playground);
            jet_index += 1;
            //- Move to ground
            if piece.fall(&playground) {
                for position in piece.position.iter() {
                    playground.insert(*position);
                    if starting_y < position.1 + 4 {
                        starting_y = position.1 + 4;
                    }
                }
                break 'outer;
            }
        }
        // Cycle Detection. Look for a point at which we have:
        // - a floor profile we've seen before
        // - The instruction pointer is in the same place
        // - The same piece has just been played
        // Measure the interval between two of these, use this to skip
        // most of the one trillion rounds
        // Skip the first 1000 rounds as we seem to see some odd loops before
        // things stabilise
        if pieces_count > 1000 {
            let mut depths = vec![];
            for x in 0..7 {
                if playground.contains(&(x, starting_y - 4)) {
                    depths.push(0);
                } else {
                    let mut y = 1;
                    while y < starting_y {
                        if !playground.contains(&(x, starting_y - 4 - y)) {
                            y += 1;
                        } else {
                            break;
                        }
                    }
                    depths.push(y);
                }
            }

            let piece_index = pieces_count % tetris_pieces.len() as i128;
            if let Some((first_match, height_to_first_match)) = skylines.get(&(
                depths.clone(),
                piece_index,
                jet_index % jet_directions.len(),
            )) {
                let cycle_time = pieces_count - first_match;
                let height_in_cycle = starting_y - 3 - height_to_first_match;
                let cycles_after_first_match = (rounds - first_match) / cycle_time;
                let additional_rounds = (rounds - first_match) % cycle_time;
                let bookends = play_tetris(input, first_match + additional_rounds);
                return cycles_after_first_match * height_in_cycle + bookends;
            } else {
                skylines.insert(
                    (depths, piece_index, jet_index % jet_directions.len()),
                    (pieces_count, starting_y - 3),
                );
            }
        }

        pieces_count += 1;
    }
    // dbg!(playground);
    starting_y - 3
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 17);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_one(&input), Some(3068));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 17);
        assert_eq!(part_two(&input), Some(1514285714288));
    }
}
