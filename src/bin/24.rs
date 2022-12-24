use std::collections::HashSet;

use glam::IVec2;

fn pos_neighbors(pos: &IVec2) -> [IVec2; 4] {
    [
        IVec2::new(pos.x + 1, pos.y),
        IVec2::new(pos.x - 1, pos.y),
        IVec2::new(pos.x, pos.y + 1),
        IVec2::new(pos.x, pos.y - 1),
    ]
}



pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut blizzards: Vec<(IVec2, IVec2)> = Vec::new();
    let mut start = IVec2::new(1, 0);
    let mut end = IVec2::new(1, 0);
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    // Fill in blizzards
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if y == 0 {
                // On top edge
                if c == '.' {
                    start = IVec2::new(x as i32, y as i32);
                }
            } else if x == 0 || y == lines.len() - 1 || x == line.len() - 1 {
                // On other edge
                if c == '.' {
                    end = IVec2::new(x as i32, y as i32);
                }
            } else if x < line.len() - 1 {
                // In the grid
                let mut dir = IVec2::new(0, 0);
                if c == '>' {
                    dir = IVec2::new(1, 0);
                } else if c == 'v' {
                    dir = IVec2::new(0, 1);
                } else if c == '<' {
                    dir = IVec2::new(-1, 0);
                } else if c == '^' {
                    dir = IVec2::new(0, -1);
                }
                // Add blizzard
                blizzards.push((IVec2::new(x as i32, y as i32), dir));
            }
        }
    }

    let mut min = 0;
    let mut choices: HashSet<IVec2> = HashSet::new();
    choices.insert(IVec2::new(start.x, start.y));
    'searching: loop {
        // Simulate blizzards for this turn
        for (bliz, dir) in blizzards.iter_mut() {
            bliz.x = (bliz.x + dir.x - 1).rem_euclid(width - 2) + 1;
            bliz.y = (bliz.y + dir.y - 1).rem_euclid(height - 2) + 1;
        }
        let mut new_choices = HashSet::new();
        for choice in &choices {
            // Take valid choices for this turn
            let mut neightbors = pos_neighbors(choice).to_vec();
            // Add stand still
            neightbors.push(*choice);
            let valid = neightbors
                .iter()
                .filter_map(|n| {
                    // Space does not contain a blizzard
                    if !blizzards.contains(&(*n, IVec2::new(1, 0)))
                        && !blizzards.contains(&(*n, IVec2::new(0, 1)))
                        && !blizzards.contains(&(*n, IVec2::new(-1, 0)))
                        && !blizzards.contains(&(*n, IVec2::new(0, -1)))
                    {
                        return Some(*n);
                    } else {
                        return None;
                    }
                })
                .collect::<Vec<IVec2>>();
            for v in valid {
                // Check if we are at end
                if v == end {
                    break 'searching;
                }
                if (v.x > 0 && v.x < width - 1 && v.y > 0 && v.y < height - 1)
                    || v == start
                    || v == end
                {
                    new_choices.insert(v);
                }
            }
        }
        // Update for next round
        choices = new_choices;
        min += 1;
        if choices.len() == 0 {
            println!("No choices!");
            break;
        }
    }
    Some(min + 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut blizzards: Vec<(IVec2, IVec2)> = Vec::new();
    let mut start = IVec2::new(1, 0);
    let mut end = IVec2::new(1, 0);
    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    // Fill in blizzards
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if y == 0 {
                // On top edge
                if c == '.' {
                    start = IVec2::new(x as i32, y as i32);
                }
            } else if x == 0 || y == lines.len() - 1 || x == line.len() - 1 {
                // On other edge
                if c == '.' {
                    end = IVec2::new(x as i32, y as i32);
                }
            } else if x < line.len() - 1 {
                // In the grid
                let mut dir = IVec2::new(0, 0);
                if c == '>' {
                    dir = IVec2::new(1, 0);
                } else if c == 'v' {
                    dir = IVec2::new(0, 1);
                } else if c == '<' {
                    dir = IVec2::new(-1, 0);
                } else if c == '^' {
                    dir = IVec2::new(0, -1);
                }
                // Add blizzard
                blizzards.push((IVec2::new(x as i32, y as i32), dir));
            }
        }
    }
    let mut min = 0;
    for _ in 0..3 {
        let mut choices: HashSet<IVec2> = HashSet::new();
        choices.insert(IVec2::new(start.x, start.y));
        'searching: loop {
            if min > 0 && min % 1000 == 0 {
                println!("min: {}", min);
            }
            // Simulate blizzards for this turn
            for (bliz, dir) in blizzards.iter_mut() {
                bliz.x = (bliz.x + dir.x - 1).rem_euclid(width - 2) + 1;
                bliz.y = (bliz.y + dir.y - 1).rem_euclid(height - 2) + 1;
            }
            let mut new_choices = HashSet::new();
            for choice in &choices {
                // Take valid choices for this turn
                let mut neightbors = pos_neighbors(choice).to_vec();
                // Add stand still
                neightbors.push(*choice);
                let valid = neightbors
                    .iter()
                    .filter_map(|n| {
                        // Space does not contain a blizzard
                        if !blizzards.contains(&(*n, IVec2::new(1, 0)))
                            && !blizzards.contains(&(*n, IVec2::new(0, 1)))
                            && !blizzards.contains(&(*n, IVec2::new(-1, 0)))
                            && !blizzards.contains(&(*n, IVec2::new(0, -1)))
                        {
                            return Some(*n);
                        } else {
                            return None;
                        }
                    })
                    .collect::<Vec<IVec2>>();
                for v in valid {
                    // Check if we are at end
                    if v == end {
                        break 'searching;
                    }
                    if (v.x > 0 && v.x < width - 1 && v.y > 0 && v.y < height - 1)
                        || v == start
                        || v == end
                    {
                        new_choices.insert(v);
                    }
                }
            }
            // Update for next round
            choices = new_choices;
            min += 1;

            if choices.len() == 0 {
                println!("No choices!");
                break;
            }
        }
        min += 1;
        // swap start/end
        let temp = start.clone();
        start = end.clone();
        end = temp;
    }
    Some(min)
}
fn main() {
    let input = &advent_of_code::read_file("inputs", 24);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_one(&input), Some(18));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 24);
        assert_eq!(part_two(&input), Some(54));
    }
}
