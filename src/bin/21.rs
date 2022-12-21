use std::{str::FromStr, char::ParseCharError, collections::HashMap};

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl FromStr for Operation {
    type Err = ParseCharError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "/" => Ok(Operation::Divide),
            "*" => Ok(Operation::Multiply),
            _ => panic!("Invalid operation")
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Monkey<'a> {
    result: Option<i64>,
    job: Option<(&'a str, Operation, &'a str)>
}

pub fn part_one(input: &str) -> Option<i64> {
    let mut monkeys: HashMap<&str, Monkey> = input.lines().map(|monkey| {
        let mut monkey_parts = monkey.split(": ");
        let id = monkey_parts.next().unwrap();
        let job_or_result = monkey_parts.next().unwrap();
        match job_or_result.parse::<i64>() {
            Ok(result) => {
                return (id, Monkey {
                    result: Some(result),
                    job: None
                });
            },
            Err(_) => {
                let mut job = job_or_result.split(" ");
                return (id, Monkey {
                    result: None,
                    job: Some((job.next().unwrap(), job.next().unwrap().parse::<Operation>().unwrap(), job.next().unwrap()))
                });
            }
        }
    }).collect();
    let monkey_ids: Vec<&str> = monkeys.iter().map(|(key, _value)| *key).collect();
    'outer: loop {
        for id in monkey_ids.iter() {
            let current_monkey = *monkeys.get(id).unwrap();
            if current_monkey.result.is_some() {
                if id == &"root" {
                    break 'outer;
                }
                continue;
            }
            match &current_monkey.job {
                Some(job) => {
                    let left_monkey = *monkeys.get(job.0).unwrap();
                    let right_monkey = *monkeys.get(job.2).unwrap();
                    if left_monkey.result.is_some() && right_monkey.result.is_some() {
                        let result = match job.1 {
                            Operation::Add => {
                                left_monkey.result.unwrap() + right_monkey.result.unwrap()
                            },
                            Operation::Subtract => {
                                left_monkey.result.unwrap() - right_monkey.result.unwrap()
                            },
                            Operation::Divide => {
                                left_monkey.result.unwrap() / right_monkey.result.unwrap()
                            },
                            Operation::Multiply => {
                                left_monkey.result.unwrap() * right_monkey.result.unwrap()
                            },
                        };
                        monkeys.entry(id).and_modify(|monkey| monkey.result = Some(result) );
                    }
                },
                None => {
                    continue;
                }
            }
        }
    }
    Some(monkeys.get(&"root").unwrap().result.unwrap())

}

pub fn part_two(input: &str) -> Option<i64> {
    let monkeys: HashMap<&str, Monkey> = input.lines().map(|monkey| {
        let mut monkey_parts = monkey.split(": ");
        let id = monkey_parts.next().unwrap();
        let job_or_result = monkey_parts.next().unwrap();
        match job_or_result.parse::<i64>() {
            Ok(result) => {
                return (id, Monkey {
                    result: Some(result),
                    job: None
                });
            },
            Err(_) => {
                let mut job = job_or_result.split(" "); 
                return (id, Monkey {
                    result: None,
                    job: Some((job.next().unwrap(), job.next().unwrap().parse::<Operation>().unwrap(), job.next().unwrap()))
                });
            }
        }
    }).collect();
    let mut unsolved: HashMap<&str, (&str, Operation, &str)> = monkeys.iter().filter_map(|(key, value)| {
        if value.result.is_none() {
            Some((*key, value.job.unwrap()))
        } else{
            None
        }
    }).collect();
    let mut solved: HashMap<&str, i64> = monkeys.iter().filter_map(|(key, value)| {
        if value.result.is_some() {
            Some((*key, value.result.unwrap()))
        } else{
            None
        }
    }).collect();
    solved.remove("humn"); // Revert wrong expression
    let (root_left, _, root_right) = unsolved.remove("root").unwrap();
    // First, solve lhs or rhs of "root" considering lhs is equal to rhs.
    solve(&mut solved, &mut unsolved);
    if let Some(&v) = solved.get(root_left) {
        solved.insert(root_right, v);
    } else if let Some(&v) = solved.get(root_right) {
        solved.insert(root_left, v);
    }
    // Second, try to solve "humn"
    solve(&mut solved, &mut unsolved);
    solved.get("humn").copied()
}

fn solve<'a>(solved: &mut HashMap<&'a str, i64>, unsolved: &mut HashMap<&'a str, (&'a str, Operation, &'a str)>) {
    while !unsolved.is_empty() {
        while !unsolved.is_empty() {
        let Some(n) = unsolved
            .iter()
            .find_map(|(n, (l, op, r))| {
                let x = solved.get(n).copied();
                let y = solved.get(l).copied();
                let z = solved.get(r).copied();
                let (k, v) = match (x, y, z) {
                    (None, Some(y), Some(z)) => {
                        let v = match op {
                            Operation::Add => y + z,
                            Operation::Subtract => y - z,
                            Operation::Multiply => y * z,
                            Operation::Divide => y / z,
                        };
                        (*n, v)
                    }
                    (Some(x), None, Some(z)) => {
                        let v = match op {
                            Operation::Add => x - z, // x = y + z
                            Operation::Subtract => x + z, // x = y - z
                            Operation::Multiply => x / z, // x = y * z
                            Operation::Divide => x * z, // x = y / z
                        };
                        (*l, v)
                    }
                    (Some(x), Some(y), None) => {
                        let v = match op {
                            Operation::Add => x - y, // x = y + z
                            Operation::Subtract => y - x, // x = y - z
                            Operation::Multiply => x / y, // x = y * z
                            Operation::Divide => y / x, // x = y / z
                        };
                        (*r, v)
                    }
                    _ => return None,
                };
                solved.insert(k, v);
                Some(*n)
            }) else {
                return;
            };
            unsolved.remove(n);
        }
    }
}


fn main() {
    let input = &advent_of_code::read_file("inputs", 21);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_one(&input), Some(152));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 21);
        assert_eq!(part_two(&input), Some(301));
    }
}
