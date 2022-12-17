use std::collections::HashMap;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Valve {
    name: String,
    flow: i64,
    tunnels: Vec<String>,
}

//Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
pub fn part_one(input: &str) -> Option<i64> {
    return None;
    lazy_static! {
        static ref PAT: Regex =
            Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]*)")
                .unwrap();
    }
    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|line| {
            let captures = PAT.captures(line).unwrap();
            let valve = captures.get(1).unwrap().as_str().to_string();
            let flow: i64 = captures.get(2).unwrap().as_str().parse().unwrap();
            let tunnels = captures
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|str| str.to_string())
                .collect::<Vec<String>>();
            (
                valve.to_string(),
                Valve {
                    name: valve,
                    flow,
                    tunnels,
                },
            )
        })
        .collect();
    let mut opened = vec![];
    let remaining: i64 = 30;
    let mut cache = HashMap::new();
    let best_pressure = walk(
        valves.get("AA").unwrap(),
        &valves,
        &mut opened,
        &mut cache,
        remaining,
    );
    Some(best_pressure)
}

fn walk(
    node: &Valve,
    valves: &HashMap<String, Valve>,
    opened_valves: &mut Vec<String>,
    cache: &mut HashMap<(String, Vec<String>, i64), i64>,
    remaining_minutes: i64,
) -> i64 {
    //No time left :(
    if remaining_minutes <= 0 {
        return 0;
    }
    //We have already walked this path with X minutes - retrieve cache
    if let Some(&ans) = cache.get(&(
        node.name.to_string(),
        opened_valves.clone(),
        remaining_minutes,
    )) {
        return ans;
    }

    let mut best = i64::MIN;
    //Current node has flow and we havent already walked here.
    if node.flow > 0 && !opened_valves.contains(&node.name) {
        for child_name in &valves.get(&node.name).unwrap().tunnels {
            let child = valves.get(child_name).unwrap();
            opened_valves.push(node.name.to_string());
            //Check all children assuming we open this.
            let sub_result = walk(child, valves, opened_valves, cache, remaining_minutes - 2);
            best = best.max(sub_result + node.flow * (remaining_minutes - 1));
            opened_valves.pop();
        }
    }

    //Check all children assuming we didnt open this.
    for child_name in &valves.get(&node.name).unwrap().tunnels {
        let child = valves.get(child_name).unwrap();
        let sub_result = walk(child, valves, opened_valves, cache, remaining_minutes - 1);
        best = best.max(sub_result);
    }

    //Store this path and seconds.
    cache.insert(
        (
            node.name.to_string(),
            opened_valves.clone(),
            remaining_minutes,
        ),
        best,
    );
    best
}

pub fn part_two(input: &str) -> Option<i64> {
    lazy_static! {
        static ref PAT: Regex =
            Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]*)")
                .unwrap();
    }
    let valves: HashMap<String, Valve> = input
        .lines()
        .map(|line| {
            let captures = PAT.captures(line).unwrap();
            let valve = captures.get(1).unwrap().as_str().to_string();
            let flow: i64 = captures.get(2).unwrap().as_str().parse().unwrap();
            let tunnels = captures
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|str| str.to_string())
                .collect::<Vec<String>>();
            (
                valve.to_string(),
                Valve {
                    name: valve,
                    flow,
                    tunnels,
                },
            )
        })
        .collect();
    let mut opened = vec![];
    let remaining: i64 = 26;
    let mut cache = HashMap::new();
    let (my_pressure, my_opened) = walk_3(
        valves.get("AA").unwrap(),
        &valves,
        &mut opened,
        &mut cache,
        remaining,
    );
    dbg!("Done with me");
    let remaining: i64 = 26;
    let mut opened = my_opened.to_vec();
    let (elefant_pressure, elefant_opened) = walk_3(
        valves.get("AA").unwrap(),
        &valves,
        &mut opened,
        &mut cache,
        remaining,
    );
    let elefant_opened: Vec<&String> = elefant_opened
        .iter()
        .filter(|key| !my_opened.contains(key))
        .collect();
    dbg!(&my_opened, my_pressure, &elefant_opened, elefant_pressure);
    Some(elefant_pressure + my_pressure)
}

fn walk_3<'a>(
    node: &Valve,
    valves: &HashMap<String, Valve>,
    opened_valves: &mut Vec<String>,
    cache: &mut HashMap<(String, Vec<String>, i64), (i64, Vec<String>)>,
    remaining_minutes: i64,
) -> (i64, Vec<String>) {
    //No time left :(
    if remaining_minutes <= 0 {
        return (0, opened_valves.to_vec());
    }
    //We have already walked this path with X minutes - retrieve cache
    if let Some((pressure, opened)) = cache.get(&(
        node.name.to_string(),
        opened_valves.clone(),
        remaining_minutes,
    )) {
        return (*pressure, opened.to_vec());
    }

    let mut best = (i64::MIN, opened_valves.to_vec());
    //Current node has flow and we havent already walked here.
    if node.flow > 0 && !opened_valves.contains(&node.name) {
        for child_name in &valves.get(&node.name).unwrap().tunnels {
            let child = valves.get(child_name).unwrap();
            opened_valves.push(node.name.to_string());
            //Check all children assuming we open this.
            let sub_result = walk_3(child, valves, opened_valves, cache, remaining_minutes - 2);
            if best.0 <= sub_result.0 + node.flow * (remaining_minutes - 1) {
                best = (
                    sub_result.0 + node.flow * (remaining_minutes - 1),
                    sub_result.1.to_vec(),
                )
            }
            // best = best.max(sub_result.0 + node.flow * (remaining_minutes - 1));
            opened_valves.pop();
        }
    }

    //Check all children assuming we didnt open this.
    for child_name in &valves.get(&node.name).unwrap().tunnels {
        let child = valves.get(child_name).unwrap();
        let sub_result = walk_3(child, valves, opened_valves, cache, remaining_minutes - 1);
        if best.0 <= sub_result.0 {
            best = (sub_result.0, sub_result.1.to_vec())
        }
    }

    //Store this path and seconds.
    cache.insert(
        (
            node.name.to_string(),
            opened_valves.clone(),
            remaining_minutes,
        ),
        best.clone(),
    );

    best
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None); //1707
    }
}
