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

pub fn part_two(input: &str) -> Option<i32> {
    lazy_static! {
        static ref PAT: Regex =
            Regex::new(r"Valve (..) has flow rate=(\d+); tunnels? leads? to valves? ([A-Z, ]*)")
                .unwrap();
    }

    //Could not figure out part 2 by doing something like p1.
    //So instead create a graph, then run DFS for both me and the elefant.

    let mut nodes = std::collections::HashMap::new();
    nodes.insert("AA".to_string(), 0usize);

    let mut graph = Vec::<(i32, Vec<usize>)>::new();
    graph.push((0, Vec::new()));

    let mut get_node = |key: &str| {
        let len = nodes.len();
        *nodes.entry(key.to_string()).or_insert(len)
    };

    for line in input.lines() {
        let captures = PAT.captures(line)?;
        let node = get_node(captures.get(1).unwrap().as_str());
        if node == graph.len() {
            graph.push((
                captures.get(2).unwrap().as_str().parse().unwrap(),
                Vec::new(),
            ));
        } else {
            let node = graph.get_mut(node).unwrap();
            node.0 = captures.get(2).unwrap().as_str().parse().unwrap();
        };

        for neighbor in captures
            .get(3)
            .unwrap()
            .as_str()
            .split(',')
            .map(|str| get_node(str.trim()))
        {
            if neighbor == graph.len() {
                graph.push((0, Vec::new()));
            }
            graph[node].1.push(neighbor);
        }
    }

    //Try and reduce runtime by removing nodes that dont have any flow,
    let mut reduced_nodes = std::collections::HashMap::new();
    reduced_nodes.insert(0, 0);
    for (node, (value, _)) in graph.iter().enumerate() {
        if *value != 0 {
            reduced_nodes.insert(node, reduced_nodes.len());
        }
    }

    let mut reduced_graph = vec![(0, vec![0; reduced_nodes.len()]); reduced_nodes.len()];

    for (node, (value, _)) in graph.iter().enumerate() {
        let reduced_node;

        if let Some(v) = reduced_nodes.get(&node) {
            reduced_node = *v;
        } else {
            continue;
        }

        reduced_graph[reduced_node].0 = *value;

        let mut visited = vec![false; nodes.len()];
        let mut queue = std::collections::VecDeque::new();

        visited[node] = true;
        queue.push_back((node, 1));

        while let Some((node, distance)) = queue.pop_front() {
            if let Some(&next) = reduced_nodes.get(&node) {
                reduced_graph[reduced_node].1[next] = distance;
            }

            for &next in graph[node].1.iter() {
                if !visited[next] {
                    visited[next] = true;
                    queue.push_back((next, distance + 1));
                }
            }
        }
    }
    for node in reduced_graph.iter_mut() {
        node.1.push(0);
    }
    reduced_graph.push((0, vec![26; reduced_graph.len() + 1]));

    Some(twoplayer_dfs::<26>(&reduced_graph))
}

fn twoplayer_dfs<const MAX: i32>(graph: &Vec<(i32, Vec<i32>)>) -> i32 {
    let mut visited = vec![false; graph.len()];
    let max_flow = graph.iter().map(|(flow, _)| *flow).sum();

    let mut max_score = 0;

    fn implementation<const MAX: i32>(
        graph: &Vec<(i32, Vec<i32>)>,
        nodes: (usize, usize),
        distance: (i32, i32),
        mut current_score: i32,
        visited: &mut [bool],
        max_score: &mut i32,
        max_flow: i32,
    ) {
        let (flow0, costs0) = graph.get(nodes.0).unwrap();
        let (flow1, costs1) = graph.get(nodes.1).unwrap();
        let max_flow = max_flow - flow0 - flow1;

        current_score += flow0 * (MAX - distance.0);
        current_score += flow1 * (MAX - distance.1);

        *max_score = (*max_score).max(current_score);

        visited[nodes.0] = true;
        visited[nodes.1] = true;

        for (next0, cost0) in costs0.iter().copied().enumerate() {
            let distance0 = distance.0 + cost0;
            if visited[next0] || distance0 > MAX {
                continue;
            }
            for (next1, cost1) in costs1.iter().copied().enumerate() {
                let distance1 = distance.1 + cost1;
                if next0 == next1 || visited[next1] || distance1 > MAX {
                    continue;
                }
                //Only continue if we actually have a chance of increasing the score
                if max_flow * (MAX - distance0.min(distance1)) + current_score > *max_score {
                    implementation::<MAX>(
                        graph,
                        (next0, next1),
                        (distance0, distance1),
                        current_score,
                        visited,
                        max_score,
                        max_flow,
                    );
                }
            }
        }

        visited[nodes.0] = false;
        visited[nodes.1] = false;
    }

    implementation::<MAX>(
        graph,
        (0, 0),
        (0, 0),
        0,
        &mut visited,
        &mut max_score,
        max_flow,
    );
    max_score
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
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), Some(1707));
    }
}
