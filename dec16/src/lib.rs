use std::collections::{BinaryHeap, HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NodeType {
    Empty,
    LeftAngledMirror,
    RightAngledMirror,
    VerticalSplitter,
    HorizontalSplitter,
}

impl From<char> for NodeType {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            '-' => Self::HorizontalSplitter,
            '|' => Self::VerticalSplitter,
            '/' => Self::RightAngledMirror,
            '\\' => Self::LeftAngledMirror,
            _ => unreachable!(),
        }
    }
}

impl NodeType {
    fn redirect(&self, direction: Direction) -> Vec<Direction> {
        use NodeType::*;
        match (self, direction) {
            (Empty, _)
            | (VerticalSplitter, Direction::Down)
            | (VerticalSplitter, Direction::Up)
            | (HorizontalSplitter, Direction::Left)
            | (HorizontalSplitter, Direction::Right) => vec![direction],
            (HorizontalSplitter, Direction::Down) | (HorizontalSplitter, Direction::Up) => {
                vec![Direction::Left, Direction::Right]
            }
            (VerticalSplitter, Direction::Left) | (VerticalSplitter, Direction::Right) => {
                vec![Direction::Up, Direction::Down]
            }
            (LeftAngledMirror, Direction::Down) | (RightAngledMirror, Direction::Up) => {
                vec![Direction::Right]
            }
            (LeftAngledMirror, Direction::Up) | (RightAngledMirror, Direction::Down) => {
                vec![Direction::Left]
            }
            (LeftAngledMirror, Direction::Right) | (RightAngledMirror, Direction::Left) => {
                vec![Direction::Down]
            }
            (LeftAngledMirror, Direction::Left) | (RightAngledMirror, Direction::Right) => {
                vec![Direction::Up]
            }
        }
    }
}

struct Node {
    cell_type: NodeType,
    visited_from: HashSet<Direction>,
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        Self::new(value)
    }
}

impl Node {
    fn new(cell_type: char) -> Self {
        Self {
            cell_type: cell_type.into(),
            visited_from: HashSet::new(),
        }
    }

    fn visit(&mut self, direction: Direction) -> Vec<Direction> {
        if self.visited_from.insert(direction) {
            return self.cell_type.redirect(direction);
        }
        Vec::new()
    }

    fn visited(&self) -> bool {
        !self.visited_from.is_empty()
    }

    fn reset(&mut self) {
        self.visited_from.clear();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CoOrds {
    x: usize,
    y: usize,
}

impl CoOrds {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn shift(&self, direction: Direction, max_x: usize, max_y: usize) -> Option<Self> {
        if (self.x == 0 && direction == Direction::Left)
            || (self.y == 0 && direction == Direction::Up)
            || (self.x + 1 == max_x && direction == Direction::Right)
            || (self.y + 1 == max_y && direction == Direction::Down)
        {
            return None;
        }
        let new_x = match direction {
            Direction::Left => self.x - 1,
            Direction::Right => self.x + 1,
            _ => self.x,
        };
        let new_y = match direction {
            Direction::Up => self.y - 1,
            Direction::Down => self.y + 1,
            _ => self.y,
        };
        Some(Self { x: new_x, y: new_y })
    }

    fn all(x: usize, y: usize) -> impl Iterator<Item = Self> {
        (0..x).flat_map(move |this_x| (0..y).map(move |this_y| Self::new(this_x, this_y)))
    }
}

fn visit_node_at(
    coords: CoOrds,
    direction: Direction,
    map: &mut Vec<Vec<Node>>,
) -> Vec<(CoOrds, Direction)> {
    let node = map.get_mut(coords.y).unwrap().get_mut(coords.x).unwrap();
    let directions = node.visit(direction);
    directions
        .into_iter()
        .filter_map(|direction| {
            coords
                .shift(direction, map[0].len(), map.len())
                .map(|coords| (coords, direction))
        })
        .collect()
}

fn energized(map: &[Vec<Node>]) -> usize {
    map.iter()
        .map(|row| row.iter().filter(|node| node.visited()).count())
        .sum()
}

fn energize_map(coords: CoOrds, direction: Direction, map: &mut Vec<Vec<Node>>) {
    let mut queue: VecDeque<(CoOrds, Direction)> = VecDeque::new();

    queue.push_back((coords, direction));

    while let Some((coords, direction)) = queue.pop_front() {
        visit_node_at(coords, direction, map)
            .into_iter()
            .for_each(|instruction| queue.push_back(instruction));
    }
}

fn reset_map(map: &mut [Vec<Node>]) {
    map.iter_mut()
        .for_each(|row| row.iter_mut().for_each(|node| node.reset()))
}

fn energize_and_reset(coords: CoOrds, direction: Direction, map: &mut Vec<Vec<Node>>) -> usize {
    energize_map(coords, direction, map);

    let energized = energized(map);

    reset_map(map);

    energized
}

pub fn part1(input: &str) -> usize {
    let mut map: Vec<Vec<Node>> = input
        .lines()
        .map(|line| line.chars().map(char::into).collect())
        .collect();

    let energized = energize_and_reset(CoOrds::new(0, 0), Direction::Right, &mut map);

    println!("{energized}");
    energized
}

pub fn part2(input: &str) -> usize {
    let mut map: Vec<Vec<Node>> = input
        .lines()
        .map(|line| line.chars().map(char::into).collect())
        .collect();

    let mut energy_levels = BinaryHeap::new();

    for coords in CoOrds::all(map[0].len(), map.len()) {
        if coords.x == 0 {
            energy_levels.push(energize_and_reset(coords, Direction::Right, &mut map));
        }
        if coords.x + 1 == map[0].len() {
            energy_levels.push(energize_and_reset(coords, Direction::Left, &mut map));
        }
        if coords.y == 0 {
            energy_levels.push(energize_and_reset(coords, Direction::Down, &mut map));
        }
        if coords.y + 1 == map.len() {
            energy_levels.push(energize_and_reset(coords, Direction::Up, &mut map));
        }
    }

    let most_energized = energy_levels.pop().unwrap();

    println!("{most_energized}");
    most_energized
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part1(input), 46);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part2(input), 51);
    }
}
