use serde::Deserialize;
use std::cmp::Ordering;
use std::cmp::Ordering::*;
#[derive(Deserialize, Debug, Eq)]
#[serde(untagged)]
enum Packet {
    List(Vec<Packet>),
    Number(u32),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::List(l0), Self::Number(r0)) => l0 == &vec![Packet::Number(*r0)],
            (Self::Number(l0), Self::List(r0)) => &vec![Packet::Number(*l0)] == r0,
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
            (Packet::List(a), Packet::Number(b)) => a.cmp(&vec![Packet::Number(*b)]),
            (Packet::Number(l), Packet::Number(r)) => l.cmp(r),
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].cmp(&b),
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input: Vec<(Packet, Packet)> = input
        .split("\n\n")
        .map(|chunk| {
            let mut packets = chunk.split("\n");
            let a = packets.next().unwrap();
            let b = packets.next().unwrap();
            (
                serde_json::from_str::<Packet>(a).unwrap(),
                serde_json::from_str::<Packet>(b).unwrap(),
            )
        })
        .collect();
    let correct_packets = input
        .iter()
        .enumerate()
        .flat_map(|(i, (l, r))| match l.cmp(r) {
            Less => Some(i + 1),
            Equal => None,
            Greater => None,
        })
        .sum();
    Some(correct_packets)
}

pub fn part_two(input: &str) -> Option<usize> {
    let decoder_keys = [
        &Packet::List(vec![Packet::List(vec![Packet::Number(2)])]),
        &Packet::List(vec![Packet::List(vec![Packet::Number(6)])]),
    ];
    let input: Vec<(Packet, Packet)> = input
        .split("\n\n")
        .map(|chunk| {
            let mut packets = chunk.split("\n");
            let a = packets.next().unwrap();
            let b = packets.next().unwrap();
            (
                serde_json::from_str::<Packet>(a).unwrap(),
                serde_json::from_str::<Packet>(b).unwrap(),
            )
        })
        .collect();
    let mut packets: Vec<&Packet> = input
        .iter()
        .flat_map(|(left, right)| [left, right])
        .chain(decoder_keys)
        .collect();
    packets.sort();
    let key1 = packets.iter().position(|p| p == &decoder_keys[0]).unwrap() + 1;
    let key2 = packets.iter().position(|p| p == &decoder_keys[1]).unwrap() + 1;
    Some(key1 * key2)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "done"]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    #[ignore = "done"]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), Some(140));
    }
}
