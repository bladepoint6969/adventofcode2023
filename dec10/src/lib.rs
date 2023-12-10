use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::HashMap;

use priority_queue::PriorityQueue;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct NodeDirections {
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl NodeDirections {
    fn new(input: char) -> Self {
        let mut new = Self::default();
        match input {
            '|' => {
                new.north = true;
                new.south = true;
            }
            '-' => {
                new.west = true;
                new.east = true;
            }
            'L' => {
                new.north = true;
                new.east = true;
            }
            'J' => {
                new.north = true;
                new.west = true;
            }
            '7' => {
                new.west = true;
                new.south = true;
            }
            'F' => {
                new.east = true;
                new.south = true;
            }
            _ => {}
        }
        new
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    directions: NodeDirections,
    visited: bool,
    distance: usize,
    x: usize,
    y: usize,
}

impl Node {
    fn new(x: usize, y: usize, dirs: char) -> Self {
        Self {
            directions: NodeDirections::new(dirs),
            visited: false,
            distance: 0,
            x,
            y,
        }
    }

    fn is_visited(&self) -> bool {
        self.visited
    }

    fn visit(
        &mut self,
        distance: usize,
        map: &HashMap<(usize, usize), RefCell<Node>>,
        queue: &mut PriorityQueue<(usize, usize), Reverse<usize>>,
    ) {
        self.visited = true;
        self.distance = distance;

        if self.directions.north {
            let next = map.get(&(self.x, self.y - 1)).unwrap().borrow();
            if !next.is_visited() {
                queue.push_increase((self.x, self.y - 1), Reverse(distance + 1));
            }
        }
        if self.directions.south {
            let next = map.get(&(self.x, self.y + 1)).unwrap().borrow();
            if !next.is_visited() {
                queue.push_increase((self.x, self.y + 1), Reverse(distance + 1));
            }
        }
        if self.directions.west {
            let next = map.get(&(self.x - 1, self.y)).unwrap().borrow();
            if !next.is_visited() {
                queue.push_increase((self.x - 1, self.y), Reverse(distance + 1));
            }
        }
        if self.directions.east {
            let next = map.get(&(self.x + 1, self.y)).unwrap().borrow();
            if !next.is_visited() {
                queue.push_increase((self.x + 1, self.y), Reverse(distance + 1));
            }
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn prepare_start(x: usize, y: usize, map: &HashMap<(usize, usize), RefCell<Node>>) {
    let mut north = false;
    let mut south = false;
    let mut east = false;
    let mut west = false;
    if y != 0 {
        if let Some(node) = map.get(&(x, y - 1)) {
            north = node.borrow().directions.south;
        }
    }
    if let Some(node) = map.get(&(x, y + 1)) {
        south = node.borrow().directions.north;
    }

    if x != 0 {
        if let Some(node) = map.get(&(x - 1, y)) {
            west = node.borrow().directions.east;
        }
    }
    if let Some(node) = map.get(&(x + 1, y)) {
        east = node.borrow().directions.west;
    }

    let mut start = map.get(&(x, y)).unwrap().borrow_mut();

    start.directions = NodeDirections {
        east,
        north,
        south,
        west,
    };
}

pub fn part1(input: &str) -> usize {
    let mut map = HashMap::new();
    let mut queue = PriorityQueue::new();
    let (mut start_x, mut start_y) = (0, 0);

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, dirs)| {
            if dirs == 'S' {
                start_x = x;
                start_y = y;
            }
            map.insert((x, y), RefCell::new(Node::new(x, y, dirs)));
        });
    });

    prepare_start(start_x, start_y, &map);
    queue.push((start_x, start_y), Reverse(0));

    let mut max_distance = 0;

    while !queue.is_empty() {
        let ((x, y), distance) = queue.pop().unwrap();
        let mut node = map.get(&(x, y)).unwrap().borrow_mut();
        max_distance = distance.0;
        node.visit(distance.0, &map, &mut queue);
    }

    println!("{max_distance}");
    max_distance
}

pub fn part2(input: &str) -> usize {
    let mut map = HashMap::new();
    let mut queue = PriorityQueue::new();
    let (mut start_x, mut start_y) = (0, 0);

    let mut x_size = 0;
    let mut y_size = 0;

    input.lines().enumerate().for_each(|(y, line)| {
        x_size = std::cmp::max(x_size, line.len());
        y_size = std::cmp::max(y_size, y);
        line.chars().enumerate().for_each(|(x, dirs)| {
            if dirs == 'S' {
                start_x = x;
                start_y = y;
            }
            map.insert((x, y), RefCell::new(Node::new(x, y, dirs)));
        });
    });

    prepare_start(start_x, start_y, &map);
    queue.push((start_x, start_y), Reverse(0));

    while !queue.is_empty() {
        let ((x, y), distance) = queue.pop().unwrap();
        let mut node = map.get(&(x, y)).unwrap().borrow_mut();
        node.visit(distance.0, &map, &mut queue);
    }

    let mut count = 0;

    for y in 0..=y_size {
        let mut crossings = 0;
        for x in 0..x_size {
            let node = map.get(&(x, y)).unwrap().borrow();
            if node.is_visited() && node.directions.north {
                crossings += 1;
            }
            if !node.is_visited() && crossings % 2 == 1{
                count += 1
            }
        }
    }

    println!("{count}");
    count
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn test_part2() {
        let input = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            .trim();
        assert_eq!(part2(input), 4);

        let input = "
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            .trim();
        assert_eq!(part2(input), 8);

        let input = "
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L".trim();
        assert_eq!(part2(input), 10);
    }
}
