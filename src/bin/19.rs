use std::{str::FromStr, fmt::Error};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Blueprint {
    ore_cost: u8,
    clay_cost: u8,
    obsidian_cost: (u8, u8),
    geode_cost: (u8,u8),
     max_ore: u8,
}

#[derive(Default, Clone, Copy)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
    geode: u8,
}

lazy_static! {
    static ref PAT: Regex =
        Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.")
            .unwrap();
}
impl FromStr for Blueprint {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = PAT.captures(s).unwrap();
        let ore_cost = captures.get(2).unwrap().as_str().parse::<u8>().unwrap();
        let clay_cost = captures.get(3).unwrap().as_str().parse::<u8>().unwrap();
        let obsidian_cost = (captures.get(4).unwrap().as_str().parse::<u8>().unwrap(), captures.get(5).unwrap().as_str().parse::<u8>().unwrap());
        let geode_cost = (captures.get(6).unwrap().as_str().parse::<u8>().unwrap(), captures.get(7).unwrap().as_str().parse::<u8>().unwrap());
        let max_ore = *[ore_cost, clay_cost, obsidian_cost.0, geode_cost.0].iter().max().unwrap();
        Ok(Blueprint {
            ore_cost,
            clay_cost,
            obsidian_cost,
            geode_cost,
            max_ore
        })
    }
}

fn maximize(
    bp: &Blueprint,
    mat: Resources,
    robots: Resources,
    ore_skipped: bool,
    clay_skipped: bool,
    obs_skipped: bool,
    time: u8,
) -> u8 {
    let can_build_ore = !ore_skipped && mat.ore >= bp.ore_cost && robots.ore < bp.max_ore;
    let can_build_clay = !clay_skipped && mat.ore >= bp.clay_cost && robots.clay < bp.obsidian_cost.1;
    let can_build_obs = !obs_skipped
        && mat.ore >= bp.obsidian_cost.0
        && mat.clay >= bp.obsidian_cost.1
        && robots.obsidian < bp.geode_cost.1;
    let can_build_geode = mat.ore >= bp.geode_cost.0 && mat.obsidian >= bp.geode_cost.1;

    let mut mat = mat;
    mat.ore += robots.ore;
    mat.clay += robots.clay;
    mat.obsidian += robots.obsidian;
    mat.geode += robots.geode;
    if time == 1 {
        return mat.geode;
    }
    if can_build_geode {
        let mut mat = mat;
        let mut robots = robots;
        mat.ore -= bp.geode_cost.0;
        mat.obsidian -= bp.geode_cost.1;
        robots.geode += 1;
        return maximize(bp, mat, robots, false, false, false, time - 1);
    }

    let mut best = 0;
    if can_build_obs {
        let mut mat = mat;
        let mut robots = robots;
        mat.ore -= bp.obsidian_cost.0;
        mat.clay -= bp.obsidian_cost.1;
        robots.obsidian += 1;
        best = maximize(bp, mat, robots, false, false, false, time - 1);
    }
    if can_build_clay {
        let mut mat = mat;
        let mut robots = robots;
        mat.ore -= bp.clay_cost;
        robots.clay += 1;
        best = best.max(maximize(bp, mat, robots, false, false, false, time - 1));
    }
    if can_build_ore {
        let mut mat = mat;
        let mut robots = robots;
        mat.ore -= bp.ore_cost;
        robots.ore += 1;
        best = best.max(maximize(bp, mat, robots, false, false, false, time - 1));
    }
    if !can_build_ore || !can_build_clay || !can_build_obs {
        best = best.max(maximize(
            bp,
            mat,
            robots,
            can_build_ore,
            can_build_clay,
            can_build_obs,
            time - 1,
        ));
    }
    best
}

// Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
pub fn part_one(input: &str) -> Option<usize> {
    let blueprints = input.lines().map(|line| line.parse::<Blueprint>().unwrap()).collect::<Vec<Blueprint>>();
    let mat = Resources::default();
    let robots = Resources {
        ore: 1,
        ..Default::default()
    };
    let mut result = 0;
    for (i, bp) in blueprints.iter().enumerate() {
        result += maximize(bp, mat, robots, false, false, false, 24) as usize * (i + 1);
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<usize> {
    let blueprints = input.lines().map(|line| line.parse::<Blueprint>().unwrap()).collect::<Vec<Blueprint>>();
    let mat = Resources::default();
    let robots = Resources {
        ore: 1,
        ..Default::default()
    };
    let mut result = 1;
    for bp in blueprints.iter().take(3) {
        result *= maximize(bp, mat, robots, false, false, false, 32) as usize;
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 19);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_one(&input), Some(33));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 19);
        assert_eq!(part_two(&input), Some(62));
    }
}
