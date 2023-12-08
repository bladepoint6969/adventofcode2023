use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!(),
        }
    }
}

struct Node {
    left: String,
    right: String,
}

impl Node {
    fn new(left: String, right: String) -> Self {
        Self { left, right }
    }

    fn get_dest(&self, dir: &Direction) -> &str {
        match dir {
            Direction::Left => &self.left,
            Direction::Right => &self.right,
        }
    }
}

pub fn part1(input: &str) -> i32 {
    let mut lines = input.lines();

    let directions: Vec<Direction> = lines.next().unwrap().chars().map(Direction::from).collect();
    lines.next(); // Throw away gap between path and map

    let mut map = HashMap::new();
    let re = Regex::new(r"(?<current>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

    for line in lines {
        let captures = re.captures(line).unwrap();
        let current = captures["current"].to_owned();
        let left = captures["left"].to_owned();
        let right = captures["right"].to_owned();
        let node = Node::new(left, right);
        map.insert(current, node);
    }

    let mut steps = 0;
    let mut current = "AAA";

    let mut directions = directions.iter().cycle();

    while current != "ZZZ" {
        let node = map.get(current).unwrap();
        let direction = directions.next().unwrap();
        current = node.get_dest(direction);
        steps += 1;
    }

    println!("{steps}");
    steps
}

pub fn part2(input: &str) -> u64 {
    let mut lines = input.lines();

    let directions: Vec<Direction> = lines.next().unwrap().chars().map(Direction::from).collect();
    lines.next(); // Throw away gap between path and map

    let mut map = HashMap::new();
    let re = Regex::new(r"(?<current>\w+) = \((?<left>\w+), (?<right>\w+)\)").unwrap();

    for line in lines {
        let captures = re.captures(line).unwrap();
        let current = captures["current"].to_owned();
        let left = captures["left"].to_owned();
        let right = captures["right"].to_owned();
        let node = Node::new(left, right);
        map.insert(current, node);
    }

    let start: Vec<&str> = map
        .keys()
        .filter_map(|key| {
            if key.ends_with('A') {
                Some(key.as_str())
            } else {
                None
            }
        })
        .collect();

    let mut directions = directions.iter().cycle();
    println!("{} paths, {start:?}", start.len());

    let steps = start
        .iter()
        .map(|&node| {
            let mut steps: u64 = 0;
            let mut current = node;
            while !current.ends_with('Z') {
                let node = map.get(current).unwrap();
                let direction = directions.next().unwrap();
                current = node.get_dest(direction);
                steps += 1;
            }
            steps
        })
        .reduce(|acc, elem| {
            if acc == 0 {
                elem
            } else {
                num::integer::lcm(acc, elem)
            }
        })
        .unwrap();

    println!("{steps}");
    steps
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 2);

        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!(part1(input), 6);
    }

    #[test]
    fn test_part2() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!(part2(input), 6);
    }
}
