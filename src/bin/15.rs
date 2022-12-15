use lazy_static::lazy_static;
use regex::Regex;
use std::ops::Range;
const SLICE: isize = 2_000_000;
#[derive(Debug)]
pub struct Sensor {
    x: isize,
    y: isize,
    radius: isize,
    beacon: (isize, isize),
}

fn get_ranges(sensors: &Vec<Sensor>) -> Vec<Range<isize>> {
    let mut ranges: Vec<Range<isize>> = sensors
        .iter()
        .filter_map(|s| {
            let width = s.radius.checked_sub(s.y.abs_diff(SLICE) as isize)?;
            Some((s.x - width)..(s.x + width + 1))
        })
        .collect();
    ranges.sort_unstable_by_key(|r| r.start);
    return ranges;
}

fn condense_ranges(ranges: Vec<Range<isize>>) -> Vec<Range<isize>> {
    let mut condensed = Vec::new();
    if ranges.len() == 0 {
        return condensed;
    }
    let mut cur = ranges[0].clone();
    for r in ranges[1..].iter() {
        if r.start > cur.end + 1 {
            condensed.push(cur);
            cur = r.clone();
        } else if cur.end < r.end {
            cur.end = r.end;
        }
    }
    condensed.push(cur);

    return condensed;
}

fn beacons_on_slice(sensors: &Vec<Sensor>) -> usize {
    let mut beacons: Vec<(isize, isize)> = sensors
        .iter()
        .map(|s| s.beacon)
        .filter(|b| b.1 == SLICE)
        .collect();
    if beacons.len() == 0 {
        return 0;
    }
    beacons.sort();
    let mut count = 1;
    for i in 1..beacons.len() {
        if beacons[i].0 != beacons[i - 1].0 {
            count += 1;
        }
    }
    return count;
}

pub fn part_one(input: &str) -> Option<usize> {
    lazy_static! {
        static ref PAT: Regex = Regex::new(
            r"Sensor at x=(-?{1}\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
        )
        .unwrap();
    }
    let mut sensors = Vec::new();
    for l in input.lines() {
        let captures = PAT.captures(l).unwrap();

        let (s_x, s_y) = (
            captures[1].parse::<isize>().unwrap(),
            captures[2].parse::<isize>().unwrap(),
        );
        let (b_x, b_y) = (
            captures[3].parse::<isize>().unwrap(),
            captures[4].parse::<isize>().unwrap(),
        );

        sensors.push(Sensor {
            x: s_x,
            y: s_y,
            radius: (s_x - b_x).abs() + (s_y - b_y).abs(), //Manhattan distance
            beacon: (b_x, b_y),
        });
    }
    let ranges = get_ranges(&sensors);
    let condensed = condense_ranges(ranges);
    let n_range: usize = condensed.iter().map(|r| r.len()).sum();
    Some(n_range - beacons_on_slice(&sensors))
}

pub fn part_two(input: &str) -> Option<isize> {
    let limit = 4_000_000;
    let regex =
        Regex::new(r"Sensor at x=(-?{1}\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
    let mut sensors = Vec::new();
    for l in input.lines() {
        let captures = regex.captures(l).unwrap();

        let (s_x, s_y) = (
            captures[1].parse::<isize>().unwrap(),
            captures[2].parse::<isize>().unwrap(),
        );
        let (b_x, b_y) = (
            captures[3].parse::<isize>().unwrap(),
            captures[4].parse::<isize>().unwrap(),
        );

        sensors.push(Sensor {
            x: s_x,
            y: s_y,
            radius: (s_x - b_x).abs() + (s_y - b_y).abs(),
            beacon: (b_x, b_y),
        });
    }
    for s in sensors.iter() {
        for x in s.x - s.radius - 1..=s.x + s.radius + 1 {
            if x > limit {
                break;
            } else if x < 0 {
                continue;
            }

            let dy = s.radius - (x - s.x).abs() + 1;
            'a: for y in [s.y + dy, s.y - dy] {
                if y <= limit && y >= 0 {
                    for s2 in sensors.iter() {
                        if (s2.x - x).abs() + (s2.y - y).abs() <= s2.radius {
                            break 'a;
                        }
                    }
                    return Some(x * 4_000_000 + y);
                }
            }
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 15);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_one(&input), Some(26));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 15);
        assert_eq!(part_two(&input), Some(56_000_011));
    }
}
