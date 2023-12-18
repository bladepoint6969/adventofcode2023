use regex::Regex;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(input: &str) -> Self {
        match input {
            "R" | "0" => Self::Right,
            "D" | "1" => Self::Down,
            "L" | "2" => Self::Left,
            "U" | "3" => Self::Up,
            _ => panic!("Disallowed input"),
        }
    }

    fn distance(&self, distance: i64) -> (i64, i64) {
        match self {
            Direction::Down => (0, distance),
            Direction::Left => (-distance, 0),
            Direction::Right => (distance, 0),
            Direction::Up => (0, -distance),
        }
    }
}

fn shoelace(points: &[(i64, i64)]) -> i64 {
    let mut sum1 = 0;
    let mut sum2 = 0;

    for i in 0..(points.len() - 1) {
        sum1 += points[i].0 * points[i + 1].1;
        sum2 += points[i].1 * points[i + 1].0;
    }

    sum1 += points.last().unwrap().0 * points[0].1;
    sum2 += points.last().unwrap().1 * points[0].0;

    (sum1 - sum2).abs() / 2
}

pub fn part1(input: &str) -> i64 {
    let re = Regex::new(r"^(?<direction>\w) (?<distance>\d+)").unwrap();
    let mut points = vec![(0, 0)];
    let mut point_cnt = 0_i64;
    input.lines().for_each(|line| {
        let captures = re.captures(line).unwrap();
        let direction = Direction::new(&captures["direction"]);
        let distance: i64 = captures["distance"].parse().unwrap();
        point_cnt += distance;
        let (x_dir, y_dir) = direction.distance(distance);
        let (last_x, last_y) = points.last().unwrap();

        let x = last_x + x_dir;
        let y = last_y + y_dir;
        points.push((x, y));
    });

    let area = shoelace(&points[1..]) + point_cnt / 2 + 1;
    println!("{area}");
    area
}

pub fn part2(input: &str) -> i64 {
    let re = Regex::new(r"#(?<distance>[0-9a-f]{5})(?<direction>[0-9a-f])").unwrap();
    let mut points = vec![(0, 0)];
    let mut point_cnt = 0_i64;
    input.lines().for_each(|line| {
        let captures = re.captures(line).unwrap();
        let direction_hex = &captures["direction"];
        let direction = Direction::new(direction_hex);

        let distance_hex = &captures["distance"];
        let distance: i64 = i64::from_str_radix(distance_hex, 16).unwrap();

        point_cnt += distance;
        let (x_dir, y_dir) = direction.distance(distance);
        let (last_x, last_y) = points.last().unwrap();

        let x = last_x + x_dir;
        let y = last_y + y_dir;
        points.push((x, y));
    });

    let area = shoelace(&points[1..]) + point_cnt / 2 + 1;
    println!("{area}");
    area
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part1(input), 62);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part2(input), 952408144115);
    }
}
