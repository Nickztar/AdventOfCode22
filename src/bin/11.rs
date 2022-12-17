#![feature(iter_array_chunks)]

use std::collections::HashMap;
#[derive(Debug)]
enum Operation {
    Add(String),
    Multiple(String),
}

impl Operation {
    pub fn execute_operation<'a>(&self, item: u64) -> u64 {
        match self {
            Operation::Add(str) => {
                if str == &"old".to_owned() {
                    return item + item;
                } else {
                    let x = str.parse::<u64>().unwrap();
                    return item + x;
                }
            }
            Operation::Multiple(str) => {
                if str == &"old".to_owned() {
                    return item * item;
                } else {
                    let x = str.parse::<u64>().unwrap();
                    return item * x;
                }
            }
        }
    }
}
#[derive(Debug)]
struct Monkey {
    id: u64,
    inspections: u64,
    operation: Operation,
    divisible: u64,
    true_id: u64,
    false_id: u64,
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = [input, " "].join("\n");
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_items: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in input.lines().array_chunks::<7>() {
        let id = line[0]
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .replace(":", "")
            .parse::<u64>()
            .unwrap();
        let items = line[1]
            .trim()
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|item| item.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let op = line[2]
            .trim()
            .split(" ")
            .skip(4)
            .take(2)
            .collect::<Vec<&str>>();
        let operation = if op[0] == "+" {
            Operation::Add(op[1].to_string())
        } else {
            Operation::Multiple(op[1].to_string())
        };
        let divisible = line[3]
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let true_id = line[4]
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let false_id = line[5]
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        monkey_items.insert(id, items.clone());
        monkeys.push(Monkey {
            id,
            inspections: 0,
            operation,
            divisible,
            true_id,
            false_id,
        })
    }
    for _r in 0..20 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            for item in monkey_items.clone().get(&monkey.id).unwrap() {
                let item = monkey.operation.execute_operation(*item) / 3;
                if item % monkey.divisible == 0 {
                    monkey_items
                        .entry(monkey.true_id)
                        .and_modify(|items| items.push(item));
                } else {
                    monkey_items
                        .entry(monkey.false_id)
                        .and_modify(|items| items.push(item));
                }
                monkey_items.entry(monkey.id).and_modify(|items| {
                    items.pop();
                });
                monkey.inspections += 1;
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    Some(
        monkeys
            .iter()
            .take(2)
            .map(|m| m.inspections)
            .fold(1, |acc, i| acc * i),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let input = [input, " "].join("\n");
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut monkey_items: HashMap<u64, Vec<u64>> = HashMap::new();
    for line in input.lines().array_chunks::<7>() {
        let id = line[0]
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .replace(":", "")
            .parse::<u64>()
            .unwrap();
        let items = line[1]
            .trim()
            .split(": ")
            .last()
            .unwrap()
            .split(", ")
            .map(|item| item.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let op = line[2]
            .trim()
            .split(" ")
            .skip(4)
            .take(2)
            .collect::<Vec<&str>>();
        let operation = if op[0] == "+" {
            Operation::Add(op[1].to_string())
        } else {
            Operation::Multiple(op[1].to_string())
        };
        let divisible = line[3]
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let true_id = line[4]
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let false_id = line[5]
            .trim()
            .split(" ")
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        monkey_items.insert(id, items.clone());
        monkeys.push(Monkey {
            id,
            inspections: 0,
            operation,
            divisible,
            true_id,
            false_id,
        })
    }
    let base = monkeys.iter().fold(1, |product, m| product * m.divisible);

    for _r in 0..10000 {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            for item in monkey_items.clone().get(&monkey.id).unwrap() {
                let item = monkey.operation.execute_operation(*item) % base;
                if item % monkey.divisible == 0 {
                    monkey_items
                        .entry(monkey.true_id)
                        .and_modify(|items| items.push(item));
                } else {
                    monkey_items
                        .entry(monkey.false_id)
                        .and_modify(|items| items.push(item));
                }
                monkey_items.entry(monkey.id).and_modify(|items| {
                    items.pop();
                });
                monkey.inspections += 1;
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspections.cmp(&a.inspections));

    Some(
        monkeys
            .iter()
            .take(2)
            .map(|m| m.inspections)
            .fold(1, |acc, i| acc * i),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
