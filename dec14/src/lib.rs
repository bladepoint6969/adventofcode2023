#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    CubeRock,
    RoundRock,
}

impl Cell {
    fn new(value: char) -> Self {
        match value {
            '#' => Cell::CubeRock,
            'O' => Cell::RoundRock,
            _ => Cell::Empty,
        }
    }
}

impl From<Cell> for char {
    fn from(value: Cell) -> Self {
        match value {
            Cell::CubeRock => '#',
            Cell::Empty => '.',
            Cell::RoundRock => 'O',
        }
    }
}

impl From<&Cell> for char {
    fn from(value: &Cell) -> Self {
        match value {
            Cell::CubeRock => '#',
            Cell::Empty => '.',
            Cell::RoundRock => 'O',
        }
    }
}

fn can_roll_north_to(grid: &[Vec<Cell>], x: usize, y: usize) -> Option<usize> {
    if grid[y][x] == Cell::RoundRock {
        let furthest = get_furthest_reachable_north(grid, x, y);
        if furthest == y {
            return None;
        }
        Some(furthest)
    } else {
        None
    }
}

fn can_roll_south_to(grid: &[Vec<Cell>], x: usize, y: usize) -> Option<usize> {
    if grid[y][x] == Cell::RoundRock {
        let furthest = get_furthest_reachable_south(grid, x, y);
        if furthest == y {
            return None;
        }
        Some(furthest)
    } else {
        None
    }
}

fn can_roll_west_to(row: &[Cell], x: usize) -> Option<usize> {
    if row[x] == Cell::RoundRock {
        let furthest = get_furthest_reachable_west(row, x);
        if furthest == x {
            return None;
        }
        Some(furthest)
    } else {
        None
    }
}

fn can_roll_east_to(row: &[Cell], x: usize) -> Option<usize> {
    if row[x] == Cell::RoundRock {
        let furthest = get_furthest_reachable_east(row, x);
        if furthest == x {
            return None;
        }
        Some(furthest)
    } else {
        None
    }
}

fn get_furthest_reachable_north(grid: &[Vec<Cell>], x: usize, y: usize) -> usize {
    if y == 0 {
        return y;
    }
    if grid[y - 1][x] == Cell::Empty {
        return get_furthest_reachable_north(grid, x, y - 1);
    }
    y
}

fn get_furthest_reachable_south(grid: &[Vec<Cell>], x: usize, y: usize) -> usize {
    if y + 1 == grid.len() {
        return y;
    }
    if grid[y + 1][x] == Cell::Empty {
        return get_furthest_reachable_south(grid, x, y + 1);
    }
    y
}

fn get_furthest_reachable_west(row: &[Cell], x: usize) -> usize {
    if x == 0 {
        return x;
    }
    if row[x - 1] == Cell::Empty {
        return get_furthest_reachable_west(row, x - 1);
    }
    x
}

fn get_furthest_reachable_east(row: &[Cell], x: usize) -> usize {
    if x + 1 == row.len() {
        return x;
    }
    if row[x + 1] == Cell::Empty {
        return get_furthest_reachable_east(row, x + 1);
    }
    x
}

fn roll_north(grid: &mut Vec<Vec<Cell>>) {
    for y in 1..grid.len() {
        for x in 0..grid[y].len() {
            if let Some(furthest) = can_roll_north_to(grid, x, y) {
                grid[y][x] = Cell::Empty;
                grid[furthest][x] = Cell::RoundRock;
            }
        }
    }
}

fn roll_south(grid: &mut Vec<Vec<Cell>>) {
    for y in (0..(grid.len() - 1)).rev() {
        for x in 0..grid[y].len() {
            if let Some(furthest) = can_roll_south_to(grid, x, y) {
                grid[y][x] = Cell::Empty;
                grid[furthest][x] = Cell::RoundRock;
            }
        }
    }
}

fn roll_west(grid: &mut Vec<Vec<Cell>>) {
    for row in grid {
        for x in 1..row.len() {
            if let Some(furthest) = can_roll_west_to(row, x) {
                row[x] = Cell::Empty;
                row[furthest] = Cell::RoundRock;
            }
        }
    }
}

fn roll_east(grid: &mut Vec<Vec<Cell>>) {
    for row in grid {
        for x in (0..(row.len() - 1)).rev() {
            if let Some(furthest) = can_roll_east_to(row, x) {
                row[x] = Cell::Empty;
                row[furthest] = Cell::RoundRock;
            }
        }
    }
}

fn build_grid(input: &str) -> Vec<Vec<Cell>> {
    input
        .lines()
        .map(|line| line.chars().map(Cell::new).collect())
        .collect()
}

pub fn part1(input: &str) -> usize {
    let mut grid: Vec<Vec<Cell>> = build_grid(input);

    roll_north(&mut grid);

    let total_load: usize = grid
        .iter()
        .enumerate()
        .map(|(idx, row)| {
            let count = row.iter().filter(|cell| **cell == Cell::RoundRock).count();
            count * (grid.len() - idx)
        })
        .sum();

    println!("{total_load}");
    total_load
}

fn spin_cycle(grid: &mut Vec<Vec<Cell>>) {
    roll_north(grid);
    roll_west(grid);
    roll_south(grid);
    roll_east(grid);
}

fn load(grid: &[Vec<Cell>]) -> usize {
    grid.iter()
        .enumerate()
        .map(|(idx, row)| {
            let count = row.iter().filter(|cell| **cell == Cell::RoundRock).count();
            count * (grid.len() - idx)
        })
        .sum()
}
pub fn part2(input: &str) -> usize {
    let mut grid: Vec<Vec<Cell>> = build_grid(input);
    let mut old_grids: Vec<Vec<Vec<Cell>>> = vec![grid.clone()];
    let mut loads = vec![load(&grid)];

    for num in 0..1_000_000_000 {
        spin_cycle(&mut grid);
        if let Some(position) = old_grids.iter().position(|e| e == &grid) {
            println!("cycle found at iteration {num}: {position}");

            let cycle_length = num - position + 1;
            let rem = 1_000_000_000 % cycle_length;
            loads.push(load(&grid));
            for i in (0..loads.len() - 1).rev() {
                if i % cycle_length == rem {
                    println!("{}", loads[i]);
                    return loads[i];
                }
            }
        }
        old_grids.push(grid.clone());
        loads.push(load(&grid));
    }

    let total_load: usize = load(&grid);

    println!("{total_load}");
    total_load
}

#[cfg(test)]
mod tests {
    use crate::{build_grid, part1, part2, roll_east, roll_north, roll_south, roll_west, Cell, spin_cycle};

    fn grid_to_string(grid: &[Vec<Cell>]) -> String {
        grid.iter()
            .map(|row| row.iter().map(char::from).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[test]
    fn test_roll_north() {
        let input = include_str!("../input_simple.txt");
        let mut grid = build_grid(input);
        roll_north(&mut grid);
        let rolled: String = grid_to_string(&grid);
        assert_eq!(rolled, include_str!("../input_simple_rolled_north.txt"));
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part1(input), 136);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(part2(input), 64);
    }

    #[test]
    fn test_rolling() {
        let input = include_str!("../input_simple.txt");
        let mut grid = build_grid(input);

        roll_north(&mut grid);
        let mut rolled = grid_to_string(&grid);
        assert_eq!(rolled, include_str!("../input_simple_rolled_north.txt"));

        roll_west(&mut grid);
        rolled = grid_to_string(&grid);
        assert_eq!(rolled, include_str!("../input_simple_cycle_1_west"));

        roll_south(&mut grid);
        rolled = grid_to_string(&grid);
        assert_eq!(rolled, include_str!("../input_simple_cycle_1_south.txt"));

        roll_east(&mut grid);
        rolled = grid_to_string(&grid);
        assert_eq!(rolled, include_str!("../input_simple_cycle_1_east.txt"));

        grid = build_grid(input);
        spin_cycle(&mut grid);
        assert_eq!(grid, build_grid(include_str!("../input_simple_cycle_1_east.txt")));
    }
}
