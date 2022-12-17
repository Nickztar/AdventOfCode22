use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
    let mut pwd: Vec<String> = Vec::new();
    let mut file_sizes: HashMap<String, usize> = HashMap::new();
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        if line.starts_with("$") && line.contains("cd") {
            if line.contains("..") {
                pwd.pop();
            } else {
                let path = [pwd.join("-"), parts[2].to_owned()].join("-");
                pwd.push(path);
            }
        } else if !line.starts_with("$") && !line.contains("dir") {
            let file_size = parts[0].parse::<usize>().unwrap();
            for d in pwd.iter() {
                file_sizes
                    .entry(d.to_string())
                    .and_modify(|e| *e += file_size)
                    .or_insert(file_size);
            }
        }
    }
    // dbg!(&file_sizes);
    Some(
        file_sizes
            .iter()
            .filter_map(|x| if x.1 <= &100000 { Some(x.1) } else { None })
            .sum(),
    )
    //677333 -> Wrong ?
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut pwd: Vec<String> = Vec::new();
    let mut file_sizes: HashMap<String, usize> = HashMap::new();
    for line in input.lines() {
        let parts = line.split(" ").collect::<Vec<&str>>();
        if line.starts_with("$") && line.contains("cd") {
            if line.contains("..") {
                pwd.pop();
            } else {
                let path = [pwd.join("-"), parts[2].to_owned()].join("-");
                pwd.push(path);
            }
        } else if !line.starts_with("$") && !line.contains("dir") {
            let file_size = parts[0].parse::<usize>().unwrap();
            for d in pwd.iter() {
                file_sizes
                    .entry(d.to_string())
                    .and_modify(|e| *e += file_size)
                    .or_insert(file_size);
            }
        }
    }

    let current_occopied = file_sizes
        .iter()
        .find(|d| *d.0 == "-/".to_string())
        .unwrap()
        .1;
    let needed_space: usize = 70000000 - 30000000;
    Some(
        *file_sizes
            .iter()
            .filter_map(|dir| {
                if dir.1 >= &(current_occopied - needed_space) {
                    Some(dir.1)
                } else {
                    None
                }
            })
            .min()
            .unwrap(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
