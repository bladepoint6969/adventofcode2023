use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
struct Cell {
    x: i64,
    y: i64,
    galaxy_number: i64,
}

impl Cell {
    fn new(x: i64, y: i64, galaxy_number: i64) -> Self {
        Self {
            x,
            y,
            galaxy_number,
        }
    }
}

fn expand_universe(
    mut galaxies: HashMap<(i64, i64), Cell>,
    x_size: i64,
    y_size: i64,
    rows_with_galaxies: &HashSet<i64>,
    cols_with_galaxies: &HashSet<i64>,
    expand_by: i64,
) -> HashMap<i64, Cell> {
    let mut expanded = HashMap::new();
    let mut bump_y_by = 0;

    for y in 0..=y_size {
        if rows_with_galaxies.contains(&y) {
            let mut bump_x_by = 0;

            for x in 0..=x_size {
                if cols_with_galaxies.contains(&x) {
                    if let Some(mut cell) = galaxies.remove(&(x, y)) {
                        cell.x += bump_x_by;
                        cell.y += bump_y_by;
                        expanded.insert(cell.galaxy_number, cell);
                    }
                } else {
                    bump_x_by += expand_by;
                }
            }
        } else {
            bump_y_by += expand_by;
        }
    }
    expanded
}

pub fn part1(input: &str) {
    chart(input, 2);
}

pub fn part2(input: &str) {
    chart(input, 1_000_000);
}

fn chart(input: &str, scale_factor: i64) -> i64 {
    let mut rows_with_galaxies = HashSet::new();
    let mut cols_with_galaxies = HashSet::new();
    let mut unexpanded_map = HashMap::new();
    let mut next_galaxy = 0;

    let mut largest_x = 0;
    let mut largest_y = 0;

    input.lines().enumerate().for_each(|(y, line)| {
        largest_y = std::cmp::max(largest_y, y as i64);
        line.chars().enumerate().for_each(|(x, char)| {
            largest_x = std::cmp::max(largest_x, x as i64);
            let x = x as i64;
            let y = y as i64;
            let cell = {
                if char == '#' {
                    next_galaxy += 1;
                    rows_with_galaxies.insert(y);
                    cols_with_galaxies.insert(x);
                    Cell::new(x, y, next_galaxy)
                } else {
                    Cell::new(x, y, 0)
                }
            };
            unexpanded_map.insert((x, y), cell);
        });
    });

    let expanded_universe = expand_universe(
        unexpanded_map,
        largest_x,
        largest_y,
        &rows_with_galaxies,
        &cols_with_galaxies,
        scale_factor - 1,
    );

    let mut total_distance = 0;

    for start_gal in 1..=next_galaxy {
        for dest_gal in start_gal..=next_galaxy {
            let start_gal = expanded_universe
                .get(&start_gal)
                .unwrap();
            let dest_gal = expanded_universe
                .get(&dest_gal)
                .unwrap();

            let distance = (start_gal.x - dest_gal.x).abs() + (start_gal.y - dest_gal.y).abs();

            total_distance += distance;
        }
    }

    println!("{total_distance}");
    total_distance
}

#[cfg(test)]
mod tests {
    use crate::chart;

    #[test]
    fn test_part1() {
        let input = include_str!("../input_simple.txt");
        assert_eq!(chart(input, 2), 374);
        assert_eq!(chart(input, 10), 1030);
        assert_eq!(chart(input, 100), 8410);
    }
}
