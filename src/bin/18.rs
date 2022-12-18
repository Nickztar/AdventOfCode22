use std::collections::HashSet;

type Coordinate = (i32, i32, i32);

pub fn part_one(input: &str) -> Option<i32> {
    let droplet_coordinates: HashSet<Coordinate> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            let pos = (
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
                parts.next().unwrap().parse().unwrap(),
            );
            // grid.insert(pos, 1);
            pos
        })
        .collect();
    let mut unconvered_sides = 0;
    for x in 0i32..20i32 {
        for y in 0i32..20i32 {
            for z in 0i32..20i32 {
                if droplet_coordinates.contains(&(x, y, z)) {
                    if !droplet_coordinates.contains(&(x + 1, y, z)) {
                        unconvered_sides += 1
                    }
                    if !droplet_coordinates.contains(&(x - 1, y, z)) {
                        unconvered_sides += 1
                    }
                    if !droplet_coordinates.contains(&(x, y + 1, z)) {
                        unconvered_sides += 1
                    }
                    if !droplet_coordinates.contains(&(x, y - 1, z)) {
                        unconvered_sides += 1
                    }
                    if !droplet_coordinates.contains(&(x, y, z + 1)) {
                        unconvered_sides += 1
                    }
                    if !droplet_coordinates.contains(&(x, y, z - 1)) {
                        unconvered_sides += 1
                    }
                }
            }
        }
    }

    Some(unconvered_sides)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = [[[0u8; 24]; 24]; 24];
    for line in input.lines() {
        let mut parts = line.split(",");
        grid[parts.next().unwrap().parse::<usize>().unwrap() + 1]
            [parts.next().unwrap().parse::<usize>().unwrap() + 1]
            [parts.next().unwrap().parse::<usize>().unwrap() + 1] = 1
    }
    for x in 1..23 {
        for y in 1..23 {
            for z in 1..23 {
                if grid[x][y][z] == 0 {
                    grid[x][y][z] = 2;
                }
            }
        }
    }
    //Recusivly fill air pockets where it reaches edge
    let mut changed = true;
    while changed {
        changed = false;
        for x in 1..23 {
            for y in 1..23 {
                for z in 1..23 {
                    if grid[x][y][z] == 2
                        && (grid[x + 1][y][z] == 0
                            || grid[x - 1][y][z] == 0
                            || grid[x][y + 1][z] == 0
                            || grid[x][y - 1][z] == 0
                            || grid[x][y][z + 1] == 0
                            || grid[x][y][z - 1] == 0)
                    {
                        grid[x][y][z] = 0;
                        changed = true;
                    }
                }
            }
        }
    }

    let mut unconvered_sides = 0;
    for x in 1..23 {
        for y in 1..23 {
            for z in 1..23 {
                if grid[x][y][z] == 1 {
                    if grid[x + 1][y][z] == 0 {
                        unconvered_sides += 1
                    }
                    if grid[x - 1][y][z] == 0 {
                        unconvered_sides += 1
                    }
                    if grid[x][y + 1][z] == 0 {
                        unconvered_sides += 1
                    }
                    if grid[x][y - 1][z] == 0 {
                        unconvered_sides += 1
                    }
                    if grid[x][y][z + 1] == 0 {
                        unconvered_sides += 1
                    }
                    if grid[x][y][z - 1] == 0 {
                        unconvered_sides += 1
                    }
                }
            }
        }
    }
    Some(unconvered_sides)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 18);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_one(&input), Some(64));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 18);
        assert_eq!(part_two(&input), Some(58));
    }
}
