#[derive(Debug, Copy, Clone, PartialEq)]
enum Visibility {
    Visible,
    Invisible,
}

type VisibilityGrid = Vec<Vec<Visibility>>;
type TreeGrid = Vec<Vec<u32>>;

fn make_tree_grid(input: &str) -> TreeGrid {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn make_tree_visibility_grid(tree_grid: &TreeGrid) -> VisibilityGrid {
    let length = tree_grid.len();
    let width = tree_grid.get(0).unwrap().len();
    let mut visibility_grid: Vec<Vec<Visibility>> = Vec::new();
    for _ in 0..length {
        visibility_grid.push(vec![Visibility::Invisible; width]);
    }
    visibility_grid
}
 

fn assess_tree_visibility(
    mut visibility_grid: VisibilityGrid,
    tree_grid: &TreeGrid,
) -> VisibilityGrid {
    let length = tree_grid.len();
    let width = tree_grid[0].len();
    let mut local_max: u32;
 
    // assess left to right
    for i in 0..length {
        local_max = tree_grid[i][0];
        visibility_grid[i][0] = Visibility::Visible;
        for j in 1..width {
            if tree_grid[i][j] > local_max {
                local_max = tree_grid[i][j];
                visibility_grid[i][j] = Visibility::Visible;
            }
        }
    }
    // assess right to left
    for i in 0..length {
        local_max = tree_grid[i][width - 1];
        visibility_grid[i][length - 1] = Visibility::Visible;
        for j in (0..width).rev() {
            if tree_grid[i][j] > local_max {
                local_max = tree_grid[i][j];
                visibility_grid[i][j] = Visibility::Visible;
            }
        }
    }
    // assess top to bottom
    for i in 0..width {
        local_max = tree_grid[0][i];
        visibility_grid[0][i] = Visibility::Visible;
        for j in 0..length {
            if tree_grid[j][i] > local_max {
                local_max = tree_grid[j][i];
                visibility_grid[j][i] = Visibility::Visible;
            }
        }
    }
    // assess bottom to top
    for i in 0..width {
        local_max = tree_grid[length - 1][i];
        visibility_grid[length - 1][i] = Visibility::Visible;
        for j in (0..length).rev() {
            if tree_grid[j][i] > local_max {
                local_max = tree_grid[j][i];
                visibility_grid[j][i] = Visibility::Visible;
            }
        }
    }
    visibility_grid
}

fn assess_scenic_score(tree_grid: TreeGrid) -> u32 {
    let mut scenic_score: u32 = 0;
    let length = tree_grid.len();
    let width = tree_grid[0].len();
    let mut up: u32;
    let mut left: u32;
    let mut right: u32;
    let mut down: u32;
    for i in 1..length - 1 {
        for j in 1..width - 1 {
            let tree_height = tree_grid[i][j];
 
            // assess up
            up = 0;
            for n in (0..i).rev() {
                up += 1;
                if tree_grid[n][j] < tree_height {
                    continue;
                } else {
                    break;
                }
            }
 
            // assess left
            left = 0;
            for n in (0..j).rev() {
                left += 1;
                if tree_grid[i][n] < tree_height {
                    continue;
                } else {
                    break;
                }
            }
 
            // assess right
            right = 0;
            for n in j + 1..width {
                right += 1;
                if tree_grid[i][n] < tree_height {
                    continue;
                } else {
                    break;
                }
            }
 
            // assess down
            down = 0;
            for n in i + 1..length {
                down += 1;
                if tree_grid[n][j] < tree_height {
                    continue;
                } else {
                    break;
                }
            }
 
            // check scenic score
            scenic_score = scenic_score.max(up * right * down * left);
        }
    }
 
    scenic_score
}
pub fn part_one(input: &str) -> Option<usize> {
      let tree_grid = make_tree_grid(input);
    let mut visibility_grid = make_tree_visibility_grid(&tree_grid);
    visibility_grid = assess_tree_visibility(visibility_grid, &tree_grid);
    let length = tree_grid.len();
    let width = tree_grid[0].len();
    let mut visible_count: usize = 0;
    for i in 0..length {
        for j in 0..width {
            if visibility_grid[i][j] == Visibility::Visible {
                visible_count += 1;
            }
        }
    }
    Some(visible_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let tree_grid = make_tree_grid(input);
    Some(assess_scenic_score(tree_grid))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
