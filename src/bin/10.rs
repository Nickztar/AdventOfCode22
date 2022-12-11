#[derive(Debug)]
enum Operation {
    Add(i32),
    Noop
}

pub fn part_one(input: &str) -> Option<i32> {
    let operations: Vec<Operation> = input.lines().map(|line| {
        if line.contains("noop") {
            Operation::Noop
        }else {
            let split = line.split(" ");
            let x = split.skip(1).next().unwrap();
            Operation::Add(x.parse().unwrap())
        }
    }).collect();
    let mut signal_str: Vec<i32> = Vec::new();
    let mut cycles: i32 = 0;
    let mut global_x: i32 = 1;
    for operation in operations {
        let action = match operation {
            Operation::Add(x) => {
                (2, x)
            },
            Operation::Noop => {
                (1, 0)
            }
        };
        for _cycle in 0..action.0 {
            cycles += 1;
            match cycles {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    signal_str.push(cycles * global_x);
                }
                _ => {}
            }
        }
        global_x += action.1;
    }
    Some(signal_str.iter().sum::<i32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let operations: Vec<Operation> = input.lines().map(|line| {
        if line.contains("noop") {
            Operation::Noop
        }else {
            let split = line.split(" ");
            let x = split.skip(1).next().unwrap();
            Operation::Add(x.parse().unwrap())
        }
    }).collect();
    let mut crt: Vec<Vec<char>> = Vec::new();
    for _y in 0..6 {
        let mut row: Vec<char> = Vec::new();
        for _x in 0..40 {
            row.push('.');
        }
        crt.push(row);
    }
    let mut y: i32 = 0;
    let mut cycles: i32 = 0;
    let mut global_x: i32 = 1;
    for operation in operations {
        let action = match operation {
            Operation::Add(x) => {
                (2, x)
            },
            Operation::Noop => {
                (1, 0)
            }
        };
        for _cycle in 0..action.0 {
            cycles += 1;
            let cursor = cycles % 40 - 1;
            let drawing = [
                global_x - 1,
                global_x,
                global_x + 1
            ];
            if drawing.contains(&cursor) {
                crt[y as usize][cursor as usize] = '#';
            }
            if y == 0 {
                println!("cursor {0} spite_pos {1:?}", cursor, drawing);
            }
            match cycles % 40 {
                0 => {
                    y += 1;
                }
                _ => {}
            }
        }
        global_x += action.1;
    }
    for row in crt {
        print!("{:?}", row.into_iter().collect::<String>());
        print!("\n");
    }
    Some(1) //Check output instead :D
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(1));
    }
}
